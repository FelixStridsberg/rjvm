use crate::class::Class;
use crate::error::{Error, Result};
use crate::io::class::ClassReader;
use crate::vm::data_type::Value;
use crate::vm::data_type::Value::{Int, Reference};
use crate::vm::frame::Frame;
use crate::vm::heap::{Heap, HeapObject};
use crate::vm::interpreter::interpret_frame;
use crate::vm::stack::Stack;
use crate::vm::Command::{
    VMGetField, VMInvokeSpecial, VMInvokeStatic, VMInvokeVirtual, VMPutField, VMReturn,
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

pub mod data_type;
mod frame;
mod heap;
mod interpreter;
mod stack;

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
    VMInvokeSpecial(u16),
    VMInvokeVirtual(u16),
    VMPutField(u16),
    VMGetField(u16),
}

#[derive(Debug)]
pub struct Object {
    class: String,
    fields: HashMap<String, i32>, // TODO poc with ints only.
                                  //  TODO fields etc
}

pub struct VirtualMachine {
    class_register: HashMap<String, Class>,
}

impl VirtualMachine {
    pub fn register_class(&mut self, filename: &str) -> Result<()> {
        let class = ClassReader::from_file(filename)?;
        self.class_register.insert(class.this_class.clone(), class);

        Ok(())
    }

    pub fn run(&mut self, class_name: &str, method_name: &str, args: Vec<Value>) -> Value {
        let mut heap = Heap::default();
        let mut stack = Stack::new();

        let result = self.execute(&mut heap, &mut stack, class_name, method_name, args);

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

    pub fn execute<'a>(
        &'a mut self,
        heap: &mut Heap,
        stack: &mut Stack<'a>,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        let init_frame = self.prepare_static_method(init_class_name, init_method_name, args);
        stack.push(init_frame);

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
                    let next_frame = self.invoke_static(index, stack.current_frame())?;
                    stack.push(next_frame);
                }
                VMInvokeSpecial(index) => {
                    let next_frame = self.invoke_special(index, stack.current_frame())?;
                    stack.push(next_frame);
                }
                VMInvokeVirtual(index) => {
                    let next_frame = self.invoke_special(index, stack.current_frame())?;
                    stack.push(next_frame);
                }
                VMPutField(index) => {
                    self.put_field(heap, index, stack.current_frame());
                }
                VMGetField(index) => {
                    self.get_field(heap, index, stack.current_frame());
                }
            }
        }
    }

    fn put_field(&self, heap: &mut Heap, index: u16, current_frame: &mut Frame) {
        let value = current_frame.pop_operand().expect_int(); // TODO other types than int
        let reference = current_frame.pop_operand().expect_reference();

        println!("Reference {:?}", reference);
        if let HeapObject::Instance(object) = heap.get(reference) {
            let (class_name, field_name, field_type) =
                current_frame.class.constants.get_field_ref(index).unwrap();
            if field_type != "I" {
                panic!("TODO Only int fields implemented.");
            }

            if object.class != class_name {
                panic!(
                    "Put field expected class {} found class {}",
                    object.class, class_name
                );
            }

            object.fields.insert(field_name.to_owned(), value);
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get(reference)
            );
        }
    }

    fn get_field(&self, heap: &mut Heap, index: u16, current_frame: &mut Frame) {
        println!("GET FIELD");
        let reference = current_frame.pop_operand().expect_reference();
        if let HeapObject::Instance(object) = heap.get(reference) {
            let (class_name, field_name, field_type) =
                current_frame.class.constants.get_field_ref(index).unwrap();
            if field_type != "I" {
                panic!("TODO Only int fields implemented.");
            }

            if object.class != class_name {
                panic!(
                    "Get field expected class {} found class {}",
                    object.class, class_name
                );
            }

            current_frame.push_operand(Int(*object.fields.get(field_name).unwrap()));
        } else {
            panic!(
                "Expected instance in heap at index {:?}, got {:?}.",
                reference,
                heap.get(reference)
            );
        }
    }

    fn invoke_special(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor) =
            current_frame.class.constants.get_method_ref(index)?;
        let class = self.resolve_class(class_name)?;
        let method = class.resolve_method(method_name, descriptor)?;
        let mut args = current_frame.pop_field_types(&method.descriptor.argument_types);
        let object_ref = current_frame.pop_operand().expect_reference();
        args.insert(0, Reference(object_ref));

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);
        Ok(frame)
    }

    fn invoke_static(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor) =
            current_frame.class.constants.get_method_ref(index)?;
        let class = self.resolve_class(class_name)?;
        let method = class.resolve_method(method_name, descriptor)?;
        let args = current_frame.pop_field_types(&method.descriptor.argument_types);

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);
        Ok(frame)
    }

    fn prepare_static_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);

        frame
    }

    fn resolve_class(&self, class_name: &str) -> Result<&Class> {
        self.class_register
            .get(class_name)
            .ok_or_else(|| Error::runtime(format!("Could not find class {}", class_name)))
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }
}
