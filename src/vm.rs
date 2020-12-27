use crate::error::Result;
use crate::vm::class_loader::ClassLoader;
use crate::vm::data_type::Value;
use crate::vm::data_type::Value::Reference;
use crate::vm::frame::Frame;
use crate::vm::heap::{Heap, HeapObject};
use crate::vm::interpreter::interpret_frame;
use crate::vm::stack::Stack;
use crate::vm::Command::{
    VMGetField, VMGetStatic, VMInvokeSpecial, VMInvokeStatic, VMInvokeVirtual, VMPutField,
    VMPutStatic, VMReturn,
};
use std::collections::HashMap;

#[macro_export]
macro_rules! expect_type (
    ($value:expr, $expected_type:path) => {
        match $value {
            $expected_type(i) => i,
            value => panic!("Tried to use a {:?} as a {}", value, stringify!($expected_type)),
        }
    }
);

pub mod class_loader;
pub mod data_type;
mod frame;
mod heap;
mod interpreter;
mod stack;

#[derive(Debug)]
enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
    VMInvokeSpecial(u16),
    VMInvokeVirtual(u16),
    VMPutField(u16),
    VMGetField(u16),
    VMPutStatic(u16),
    VMGetStatic(u16),
}

#[derive(Debug)]
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
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Value {
        let mut heap = Heap::default();
        let mut stack = Stack::new();
        let mut class_loader = class_loader;
        let mut static_context: StaticContext = HashMap::new();

        let result = self.execute(
            &mut static_context,
            &mut heap,
            &mut stack,
            &mut class_loader,
            class_name,
            method_name,
            args,
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
    ) -> Result<Value> {
        self.prepare_static_method(
            class_loader,
            init_class_name,
            init_method_name,
            args,
            stack,
        );

        loop {
            match interpret_frame(stack.current_frame(), heap)? {
                VMReturn(value) => {
                    if stack.last_frame() {
                        return Ok(value);
                    } else {
                        let void_return = stack
                            .current_frame()
                            .method
                            .descriptor
                            .return_type
                            .is_none();

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
                    self.invoke_special(class_loader, index, stack)?;
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
            }
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
            stack.current_frame().pc -= 3; // Get static is 1 byte and 2 arguments
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

    fn put_field(&self, heap: &mut Heap, index: u16, stack: &mut Stack) {
        let value = stack.current_frame().pop_operand();
        let reference = stack.current_frame().pop_operand().expect_reference();

        if let HeapObject::Instance(object) = heap.get(reference) {
            let field = stack
                .current_frame()
                .class
                .constants
                .get_field_ref(index)
                .unwrap();

            if object.class != field.class_name {
                panic!(
                    "Put field expected class {} found class {}",
                    object.class, field.class_name
                );
            }

            object.fields.insert(field.field_name, value);
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get(reference)
            );
        }
    }

    fn get_field(&self, heap: &mut Heap, index: u16, stack: &mut Stack) {
        let reference = stack.current_frame().pop_operand().expect_reference();
        if let HeapObject::Instance(object) = heap.get(reference) {
            let field = stack
                .current_frame()
                .class
                .constants
                .get_field_ref(index)
                .unwrap();

            if object.class != field.class_name {
                panic!(
                    "Get field expected class {} found class {}",
                    object.class, field.class_name
                );
            }

            stack
                .current_frame()
                .push_operand(object.fields.get(&field.field_name).unwrap().clone());
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get(reference)
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
        let method = class.resolve_method(method_name, descriptor)?;
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
        let method = class.resolve_method(method_name, descriptor)?;
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
        let (class, init_frame) = class_loader.resolve(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

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
