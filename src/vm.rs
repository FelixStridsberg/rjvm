use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType};
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

    pub fn run(&mut self, class_name: &str, method_name: &str, args: Vec<Value>) -> Value {
        // TODO separate the running and initialization of frames.
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

        let code = method.get_code().expect("No Code attribute on method.");

        let mut stack = Vec::new();

        let mut new_frame = Frame::new(&code, &class.constants);
        let mut index = 0;
        for arg in args {
            if arg.get_category() == 1 {
                new_frame.set_local(index, arg.get_int_value());
                index += 1;
            } else {
                new_frame.set_local_long(index, arg.get_long_value());
                index += 2;
            }
        }

        stack.push(new_frame);

        loop {
            let mut frame = stack.pop().unwrap();

            match interpret_frame(&mut frame) {
                VMReturn(value) => return value,
                VMInvokeStatic(_index) => {
                    panic!("Invoked!")
                    //Self::invoke_static(&mut frame, index)
                }
            }
        }
    }

    /*
        pub fn invoke_static_entry_point(
            &mut self,
            class_register: &mut ClassRegister,
            class_name: &str,
            method_name: &str,
            args: Vec<Value>,
        ) -> Option<Value> {
            let class = class_register.get(class_name).expect("Unknown class");
            let method = class.find_public_static_method(method_name).unwrap();

            let code = method.get_code().expect("No Code attribute on method.");
            let mut frame = Frame::new(code.max_stack, code.max_locals, &class.constants);



            loop {
                let instructions = &code.instructions[frame.pc as usize];
                match interpret_instruction(&mut frame, instructions) {
                    Running => {}
                    Returned(value) => return value,
                    InvokedStatic(index) => Self::invoke_static(&mut frame, index),
                }
            }

    //        Self::execute_frame(&mut frame, &code, class_register)
        }*/

    fn invoke_static(frame: &mut Frame, index: u16) {
        let method_ref = frame.constant_pool.get(index);

        match method_ref {
            MethodRef(class_index, name_type_index) => {
                if let ClassRef(class) = frame.constant_pool.get(*class_index) {
                    println!("class: {:?}", frame.constant_pool.get(*class));
                }
                if let NameAndType(name_idx, type_idx) = frame.constant_pool.get(*name_type_index) {
                    println!("name: {:?}", frame.constant_pool.get(*name_idx));
                    println!("type: {:?}", frame.constant_pool.get(*type_idx));
                }
            }
            _ => panic!(""),
        }

        panic!(
            "Invoked static at index {:?}: {:?}",
            index,
            frame.constant_pool.get(index)
        );
    }
}
