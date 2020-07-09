use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType, Utf8};
use crate::class::Class;
use crate::error::Result;
use crate::io::class::ClassReader;
use crate::vm::frame::Frame;
use crate::vm::interpreter::interpret_frame;
use crate::vm::Command::{VMInvokeStatic, VMReturn};
use crate::vm::Value::*;
use std::collections::HashMap;

mod frame;
mod interpreter;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(char),
    Float(f32),
    Double(f64),
    Reference(i32),
    ReturnAddress(i32),
    Null,
}

impl Value {
    pub fn get_category(&self) -> u8 {
        match self {
            Long(_) | Double(_) => 2,
            _ => 1,
        }
    }

    pub fn get_int_value(&self) -> u32 {
        match self {
            Boolean(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Byte(b) => *b as u32,
            Short(s) => *s as u32,
            Char(c) => *c as u32,
            Float(f) => (*f).to_bits(),
            Null => 0,
            Int(i) | Reference(i) | ReturnAddress(i) => *i as u32,
            _ => panic!("Tried to get int value of {:?}", self),
        }
    }

    pub fn get_long_value(&self) -> u64 {
        match self {
            Long(l) => *l as u64,
            Double(d) => (*d).to_bits(),
            _ => panic!("Tried to get long value of {:?}", self),
        }
    }
}

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
}

pub struct VirtualMachine {
    class_register: HashMap<String, Class>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }

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
                    // Temp hack for POC, this should be read from method type I assume.
                    let arg1 = current_frame.pop_operand_int();
                    let arg2 = current_frame.pop_operand_int();
                    let args = vec![Int(arg1), Int(arg2)];

                    let (class_name, method_name) = Self::get_static(&current_frame, index);
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

    fn get_static<'a>(frame: &'a Frame, index: u16) -> (&'a str, &'a str) {
        let method_ref = frame.constant_pool.get(index);

        let mut class_name = "";
        let mut method_name = "";
        match method_ref {
            MethodRef(class_index, name_type_index) => {
                if let ClassRef(class) = frame.constant_pool.get(*class_index) {
                    if let Utf8(s) = frame.constant_pool.get(*class) {
                        class_name = s;
                    }
                }
                if let NameAndType(name_idx, type_idx) = frame.constant_pool.get(*name_type_index) {
                    if let Utf8(s) = frame.constant_pool.get(*name_idx) {
                        method_name = s;
                    }
                    // println!("type: {:?}", frame.constant_pool.get(*type_idx));
                }
            }
            _ => panic!(""),
        }

        (class_name, method_name)
    }
}
