use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType};
use crate::class::Class;
use crate::error::Result;
use crate::io::class::ClassReader;
use crate::vm::data_type::{MethodDescriptor, Value};
use crate::vm::frame::Frame;
use crate::vm::interpreter::interpret_frame;
use crate::vm::Command::{VMInvokeStatic, VMReturn};
use std::collections::HashMap;
use std::convert::TryInto;

pub mod data_type;
mod frame;
mod interpreter;

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
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
        let initial_frame = self.prepare_static_method(init_class_name, init_method_name, args);

        let mut stack = Vec::new();
        let mut current_frame = initial_frame;

        loop {
            match interpret_frame(&mut current_frame) {
                VMReturn(value) => {
                    if stack.is_empty() {
                        return value;
                    } else {
                        current_frame = stack.pop().unwrap();
                        current_frame.push_operand(value);
                    }
                }
                VMInvokeStatic(index) => {
                    let (descriptor, class_name, method_name) =
                        Self::get_static_method(&current_frame, index);
                    let args = current_frame.pop_field_types(&descriptor.argument_types);
                    let next_frame = self.prepare_static_method(class_name, method_name, args);

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

    fn get_static_method<'a>(
        frame: &Frame<'a>,
        index: u16,
    ) -> (MethodDescriptor<'a>, &'a str, &'a str) {
        let method_ref = frame.constant_pool.get(index);

        let mut class_name = "";
        let mut method_name = "";
        let mut descriptor_str = "";
        match method_ref {
            MethodRef(class_index, name_type_index) => {
                if let ClassRef(class) = frame.constant_pool.get(*class_index) {
                    class_name = frame.constant_pool.get_utf8(*class);
                }
                if let NameAndType(name_idx, type_idx) = frame.constant_pool.get(*name_type_index) {
                    method_name = frame.constant_pool.get_utf8(*name_idx);
                    descriptor_str = frame.constant_pool.get_utf8(*type_idx);
                    // println!("type: {:?}", frame.constant_pool.get(*type_idx));
                }
            }
            _ => panic!(""),
        }

        let descriptor = descriptor_str.try_into().unwrap();
        (descriptor, class_name, method_name)
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }
}
