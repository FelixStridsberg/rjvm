use crate::class::Class;
use crate::error::Result;
use crate::io::class::ClassReader;
use crate::vm::data_type::Value::Reference;
use crate::vm::data_type::{MethodDescriptor, Value};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;
use crate::vm::interpreter::interpret_frame;
use crate::vm::stack::Stack;
use crate::vm::Command::{VMInvokeSpecial, VMInvokeStatic, VMReturn};
use std::collections::HashMap;
use std::convert::TryInto;

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
}

#[derive(Debug)]
pub struct Object {
    class: String,
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
                        stack.pop();
                        stack.current_frame().push_operand(value);
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
            }
        }
    }

    fn invoke_special(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor_string) =
            current_frame.constant_pool.get_method_ref(index)?;
        let descriptor: MethodDescriptor = descriptor_string.try_into().unwrap();
        let object_ref = current_frame.pop_operand().expect_reference();
        let mut args = current_frame.pop_field_types(&descriptor.argument_types);
        args.insert(0, Reference(object_ref));

        Ok(self.prepare_method(class_name, method_name, args))
    }

    fn invoke_static(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor_string) =
            current_frame.constant_pool.get_method_ref(index)?;
        let descriptor: MethodDescriptor = descriptor_string.try_into().unwrap();
        let args = current_frame.pop_field_types(&descriptor.argument_types);
        Ok(self.prepare_static_method(class_name, method_name, args))
    }

    fn prepare_static_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

        let mut frame = Frame::new(&method, &class.constants);
        frame.load_arguments(args);

        frame
    }

    fn prepare_method(&self, class_name: &str, method_name: &str, args: Vec<Value>) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_method(method_name).unwrap();

        let mut frame = Frame::new(&method, &class.constants);
        frame.load_arguments(args);

        frame
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }
}
