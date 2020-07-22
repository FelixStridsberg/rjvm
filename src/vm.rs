use crate::class::Class;
use crate::error::Result;
use crate::io::class::ClassReader;
use crate::vm::data_type::{MethodDescriptor, Value};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;
use crate::vm::interpreter::interpret_frame;
use crate::vm::Command::{VMInvokeStatic, VMReturn, VMInvokeSpecial};
use std::collections::HashMap;
use std::convert::TryInto;
use crate::vm::data_type::Value::Reference;

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

    pub fn run(
        &mut self,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
    ) -> Value {
        let mut heap = Heap::default();
        let mut stack = Vec::new();

        if let Ok(value) = self.execute(&mut heap, &mut stack, init_class_name, init_method_name, args) {
            return value
        }

        panic!("Execution failed.");
    }

    pub fn execute<'a>(
        &'a mut self,
        heap: &mut Heap,
        stack: &mut Vec<Frame<'a>>,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        let mut current_frame = self.prepare_static_method(init_class_name, init_method_name, args);

        loop {
            match interpret_frame(&mut current_frame, heap) {
                VMReturn(value) => {
                    if stack.is_empty() {
                        return Ok(value);
                    } else {
                        current_frame = stack.pop().unwrap();
                        current_frame.push_operand(value);
                    }
                }
                VMInvokeStatic(index) => {
                    let (class_name, method_name, descriptor_string) =
                        current_frame.constant_pool.get_method_ref(index)?;
                    let descriptor: MethodDescriptor = descriptor_string.try_into().unwrap();
                    let args = current_frame.pop_field_types(&descriptor.argument_types);
                    let next_frame = self.prepare_static_method(class_name, method_name, args);

                    stack.push(current_frame);
                    current_frame = next_frame;
                },
                VMInvokeSpecial(index) => {
                    let (class_name, method_name, descriptor_string) =
                        current_frame.constant_pool.get_method_ref(index)?;
                    let descriptor: MethodDescriptor = descriptor_string.try_into().unwrap();
                    let object_ref = current_frame.pop_operand().expect_reference();
                    let mut args = current_frame.pop_field_types(&descriptor.argument_types);
                    args.insert(0, Reference(object_ref));

                    let next_frame = self.prepare_method(class_name, method_name, args);

                    stack.push(current_frame);
                    current_frame = next_frame;

                }
            }
        }
    }

    fn prepare_static_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();
        let code = method.get_code().expect("No Code attribute on method.");

        let mut frame = Frame::new(&code, &class.constants);
        frame.load_arguments(args);

        frame
    }

    fn prepare_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_method(method_name).unwrap();
        let code = method.get_code().expect("No Code attribute on method.");

        let mut frame = Frame::new(&code, &class.constants);
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
