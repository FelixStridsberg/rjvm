use crate::error::Result;
use crate::vm::class_loader::ClassLoader;
use crate::vm::data_type::Value::{Null, Reference};
use crate::vm::data_type::{ReferenceType, Value};
use crate::vm::frame::Frame;
use crate::vm::heap::{Heap, HeapObject};
use crate::vm::interpreter::interpret_frame;
use crate::vm::native::Native;
use crate::vm::stack::Stack;
use crate::vm::VMCommand::{
    VMAllocateReferenceArray, VMException, VMGetField, VMGetStatic, VMInvokeInterface,
    VMInvokeSpecial, VMInvokeStatic, VMInvokeVirtual, VMNative, VMPutField, VMPutStatic, VMReturn,
};
use std::collections::HashMap;

#[macro_export]
macro_rules! expect_type {
    ($value:expr, $($expected_type:path)|+) => {
        match $value {
            $($expected_type(i) => i,)+
            value => panic!("Tried to use a {:?} as a {}", value, stringify!($($expected_type,)+)),
        }
    };
    ($value:expr, $($expected_type:path)|+, $type:ty) => {
        match $value {
            $($expected_type(i) => i as $type,)+
            value => panic!("Tried to use a {:?} as a {}", value, stringify!($($expected_type,)+)),
        }
    };
}

pub mod class_loader;
pub mod data_type;
mod frame;
mod heap;
mod interpreter;
pub mod native;
mod stack;

#[derive(Debug)]
enum VMCommand {
    VMReturn(Value),
    VMInvokeStatic(u16),
    VMInvokeSpecial(u16),
    VMInvokeVirtual(u16),
    VMInvokeInterface(u16),
    VMPutField(u16),
    VMGetField(u16),
    VMPutStatic(u16),
    VMGetStatic(u16),
    VMAllocateReferenceArray(u16),
    VMException(),
    VMNative(),
}

#[derive(Debug, PartialEq)]
pub struct Object {
    class: String,
    fields: HashMap<String, Value>,
    // TODO fields etc
}

type StaticContext = HashMap<String, HashMap<String, Value>>;

pub struct VirtualMachine {}

impl VirtualMachine {
    pub fn run(
        &mut self,
        class_loader: ClassLoader,
        native: Native,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Value {
        let mut heap = Heap::default();
        let mut stack = Stack::new();
        let mut class_loader = class_loader;
        let mut native = native;
        let mut static_context: StaticContext = HashMap::new();

        /*
        let init_result = self.execute(
            &mut static_context,
            &mut heap,
            &mut stack,
            &mut class_loader,
            "java/lang/System",
            "initializeSystemClass",
            vec![],
            &mut native,
        );

        panic!("INIT RET {:?}", init_result.unwrap());
         */

        let result = self.execute(
            &mut static_context,
            &mut heap,
            &mut stack,
            &mut class_loader,
            class_name,
            method_name,
            args,
            &mut native,
        );

        if let Ok(value) = result {
            value
        } else {
            println!("Stack:\n{}", stack);
            println!("Heap: {:#?}", heap);

            panic!(
                "Runtime error {:?}",
                result.unwrap_err().message().expect("No error message.")
            );
        }
    }

    pub fn execute(
        &mut self,
        static_context: &mut StaticContext,
        heap: &mut Heap,
        stack: &mut Stack,
        class_loader: &mut ClassLoader,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
        native: &mut Native,
    ) -> Result<Value> {
        self.prepare_static_method(class_loader, init_class_name, init_method_name, args, stack);

        loop {
            let mut freeze_pc = false;
            let stack_size = stack.len();

            let frame = stack.current_frame();
            match interpret_frame(frame, heap)? {
                VMReturn(value) => {
                    if stack.last_frame() {
                        return Ok(value);
                    } else {
                        let frame = stack.current_frame();
                        let void_return = frame.method.descriptor.return_type.is_none();

                        // We must not update the pc when returning from implicit frames.
                        if frame.implicit {
                            freeze_pc = true;
                        }

                        stack.pop();

                        if !void_return {
                            stack.current_frame().push_operand(value);
                        }
                    }
                }
                VMInvokeStatic(index) => {
                    self.invoke_static(class_loader, index, stack)?;
                }
                VMInvokeSpecial(index) => {
                    self.invoke_special(class_loader, index, stack)?;
                }
                VMInvokeVirtual(index) => {
                    self.invoke_virtual(class_loader, index, stack)?;
                }
                VMInvokeInterface(index) => {
                    self.invoke_interface(heap, class_loader, index, stack)?;
                }
                VMPutField(index) => {
                    self.put_field(heap, index, stack);
                }
                VMGetField(index) => {
                    self.get_field(heap, index, stack);
                }
                VMPutStatic(index) => {
                    self.put_static(static_context, index, stack);
                }
                VMGetStatic(index) => {
                    self.get_static(class_loader, static_context, index, stack);
                }
                VMAllocateReferenceArray(index) => {
                    self.allocate_reference_array(heap, class_loader, index, stack)?;
                }
                VMException() => {
                    // We must not update PC after exception resolution, the pc is placed at the
                    // handler.
                    freeze_pc = true;
                    self.handle_exception(heap, stack);
                }
                VMNative() => {
                    self.call_native(stack, native);
                }
            };

            // Update pc only if we did not get a new frame, or the pc is frozen.
            if !freeze_pc && stack.len() <= stack_size {
                let frame = stack.current_frame();
                frame.pc_next();
            }
        }
    }

    fn call_native(&self, stack: &mut Stack, native: &mut Native) {
        let val = native.invoke(stack.current_frame());
        stack.pop();

        if let Some(val) = val {
            stack.current_frame().push_operand(val);
        }
    }

    fn handle_exception(&self, heap: &Heap, stack: &mut Stack) {
        let reference = stack.current_frame().pop_operand().expect_reference();
        let exception = heap.get(reference).expect_instance();

        println!("Exception thrown: {:?}", exception);

        loop {
            let frame = stack.current_frame();

            if frame.handle_exception(&exception) {
                frame.push_operand(Reference(reference));
                break;
            }

            if stack.last_frame() {
                panic!("Uncaught exception: {:?}", exception);
            }

            stack.pop();
        }
    }

    fn put_static(&self, static_context: &mut StaticContext, index: u16, stack: &mut Stack) {
        let frame = stack.current_frame();
        let value = frame.pop_operand();
        let field = frame.class.constants.get_field_ref(index).unwrap();

        let context = static_context
            .entry(field.class_name)
            .or_insert_with(HashMap::new);
        context.insert(field.field_name, value);
    }

    fn get_static(
        &self,
        class_loader: &mut ClassLoader,
        static_context: &StaticContext,
        index: u16,
        stack: &mut Stack,
    ) {
        let field = stack
            .current_frame()
            .class
            .constants
            .get_field_ref(index)
            .unwrap();
        let (_class, init_frame) = class_loader.resolve(&field.class_name).unwrap();

        // The class we are trying to access is not yet initialized, we must initialize it and try again.
        if let Some(frame) = init_frame {
            stack.push(frame);
            return;
        }

        let v = static_context
            .get(&field.class_name)
            .unwrap()
            .get(&field.field_name)
            .unwrap()
            .clone();

        stack.current_frame().push_operand(v);
    }

    fn allocate_reference_array(
        &self,
        heap: &mut Heap,
        class_loader: &mut ClassLoader,
        index: u16,
        stack: &mut Stack,
    ) -> Result<()> {
        let frame = stack.current_frame();
        let length = frame.pop_operand().expect_int();
        let class_name = frame.class.constants.get_class_info_name(index).unwrap();

        let (class, init_frame) = class_loader.resolve(class_name)?;
        let reference = heap.allocate_reference_array(length, class);

        frame.push_operand(Reference(reference as ReferenceType));

        if let Some(init_frame) = init_frame {
            stack.push(init_frame);
        }

        Ok(())
    }

    fn put_field(&self, heap: &mut Heap, index: u16, stack: &mut Stack) {
        let value = stack.current_frame().pop_operand();
        let reference = stack.current_frame().pop_operand().expect_reference();

        if let HeapObject::Instance(object) = heap.get_mut(reference) {
            let field = stack
                .current_frame()
                .class
                .constants
                .get_field_ref(index)
                .unwrap();

            if object.class != field.class_name {
                eprintln!(
                    "Put field expected class {} found class {}, TODO check if instance of",
                    object.class, field.class_name
                );
            }

            object.fields.insert(field.field_name, value);
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get_mut(reference)
            );
        }
    }

    fn get_field(&self, heap: &mut Heap, index: u16, stack: &mut Stack) {
        let reference = stack.current_frame().pop_operand().expect_reference();
        if let HeapObject::Instance(object) = heap.get_mut(reference) {
            let field = stack
                .current_frame()
                .class
                .constants
                .get_field_ref(index)
                .unwrap();

            if object.class != field.class_name {
                eprintln!(
                    "Get field expected class {} found class {}, TODO check if instance of",
                    object.class, field.class_name
                );
            }

            stack.current_frame().push_operand(
                object
                    .fields
                    .get(&field.field_name)
                    .map_or(field.field_type.default_value(), |f| f.clone()),
            );
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get_mut(reference)
            );
        }
    }

    fn invoke_special(
        &self,
        class_loader: &mut ClassLoader,
        index: u16,
        stack: &mut Stack,
    ) -> Result<()> {
        let (class_name, method_name, descriptor) = stack
            .current_frame()
            .class
            .constants
            .get_method_ref(index)?;
        let (class, init_frame) = class_loader.resolve(class_name)?;
        let method = class
            .resolve_method(method_name, descriptor)
            .expect("Method not found");

        let mut args = stack
            .current_frame()
            .pop_field_types(&method.descriptor.argument_types);

        let object_ref = stack.current_frame().pop_operand().expect_reference();
        args.insert(0, Reference(object_ref));

        let mut frame = Frame::new(class, method);
        frame.load_arguments(args);

        stack.push(frame);
        if let Some(init_frame) = init_frame {
            stack.push(init_frame);
        }
        Ok(())
    }

    fn invoke_virtual(
        &self,
        class_loader: &mut ClassLoader,
        index: u16,
        stack: &mut Stack,
    ) -> Result<()> {
        let (class_name, method_name, descriptor) = stack
            .current_frame()
            .class
            .constants
            .get_method_ref(index)?;

        let mut class_name = class_name.to_owned();
        loop {
            let (class, init_frame) = class_loader.resolve(&class_name)?;
            let method = class.resolve_method(method_name, descriptor);

            if let None = method {
                class_name = class.super_class.clone();
                continue;
            }

            let method = method.unwrap();
            let mut args = stack
                .current_frame()
                .pop_field_types(&method.descriptor.argument_types);

            let object_ref = stack.current_frame().pop_operand().expect_reference();
            args.insert(0, Reference(object_ref));

            let mut frame = Frame::new(class, method);
            frame.load_arguments(args);

            stack.push(frame);
            if let Some(init_frame) = init_frame {
                stack.push(init_frame);
            }

            return Ok(());
        }
    }

    fn invoke_interface(
        &self,
        heap: &mut Heap,
        class_loader: &mut ClassLoader,
        index: u16,
        stack: &mut Stack,
    ) -> Result<()> {
        let (interface_name, method_name, descriptor) = stack
            .current_frame()
            .class
            .constants
            .get_interface_method_ref(index)?;

        let interface_name = interface_name.to_owned();
        let method_name = method_name.to_owned();
        let descriptor = descriptor.to_owned();

        let object_ref = stack.current_frame().pop_operand().expect_reference();
        let instance = heap.get(object_ref).expect_instance();

        let mut class_name = instance.class.to_owned();

        loop {
            let (class, init_frame) = class_loader.resolve(&class_name)?;

            if matches!(init_frame, Some(_)) {
                panic!("Todo init_frame on resolving interface method");
            }

            /* TODO interfaces can extend each other etc...
            if !class.interfaces.contains(&interface_name) {
                println!("NOT FOUND");
                class_name = class.super_class.to_owned();
                continue;
            }*/

            let method = class.resolve_method(&method_name, &descriptor);
            if matches!(method, None) {
                class_name = class.super_class.to_owned();
                continue;
            }

            let method = method.unwrap();
            let mut args = stack
                .current_frame()
                .pop_field_types(&method.descriptor.argument_types);

            args.insert(0, Reference(object_ref));

            let mut frame = Frame::new(class, method);
            frame.load_arguments(args);

            stack.push(frame);
            if let Some(init_frame) = init_frame {
                stack.push(init_frame);
            }

            return Ok(());
        }
    }

    fn invoke_static(
        &self,
        class_loader: &mut ClassLoader,
        index: u16,
        stack: &mut Stack,
    ) -> Result<()> {
        let (class_name, method_name, descriptor) = stack
            .current_frame()
            .class
            .constants
            .get_method_ref(index)?;

        let (class, init_frame) = class_loader.resolve(class_name)?;
        let method = class
            .resolve_method(method_name, descriptor)
            .expect("Method not found");

        let args = stack
            .current_frame()
            .pop_field_types(&method.descriptor.argument_types);

        let mut frame = Frame::new(class, method);
        frame.load_arguments(args);

        stack.push(frame);
        if let Some(init_frame) = init_frame {
            stack.push(init_frame);
        }
        Ok(())
    }

    fn prepare_static_method(
        &self,
        class_loader: &mut ClassLoader,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
        stack: &mut Stack,
    ) {
        let (class, init_frame) = class_loader.resolve(class_name).expect("Unknown class"); // TODO more info in errors
        let method = class
            .find_public_static_method(method_name)
            .expect("Method not found");

        let mut frame = Frame::new(class, method);
        frame.load_arguments(args);

        stack.push(frame);
        if let Some(init_frame) = init_frame {
            stack.push(init_frame);
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {}
    }
}
