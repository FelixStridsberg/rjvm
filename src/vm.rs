use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType};
use crate::class::constant::ConstantPool;
use crate::class::Class;
use crate::vm::interpreter::{interpret_instruction, State};
use crate::vm::interpreter::State::*;
use crate::vm::Value::*;
use crate::error::Result;
use std::collections::HashMap;
use crate::class::attribute::Code;
use crate::io::class::ClassReader;

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

pub type ClassRegister = HashMap<String, Class>;

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

    pub fn run(&mut self, class_name: &str, method_name: &str, _args: Vec<Value>) {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

        let code = method.get_code().expect("No Code attribute on method.");

        let mut stack = Vec::new();
        stack.push(Frame::new(&code, &class.constants));


        loop {
            let mut frame = stack.pop().unwrap();

            match Self::interpret(&mut frame) {
                Returned(_value) => {
                    panic!("Returned!")
                },
                InvokedStatic(_index) => {
                    panic!("Invoked!")
                    //Self::invoke_static(&mut frame, index)
                },
                _ => panic!("don't know what it did")
            }
        }
    }

    fn interpret(frame: &mut Frame) -> State {
        loop {
            let instructions = &frame.code.instructions[frame.pc as usize];
            match interpret_instruction(frame, instructions) {
                Running => {},
                state => return state,
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

        let mut index = 0;
        for arg in args {
            if arg.get_category() == 1 {
                frame.set_local(index, arg.get_int_value());
                index += 1;
            } else {
                frame.set_local_long(index, arg.get_long_value());
                index += 2;
            }
        }

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

#[derive(Debug)]
pub struct Frame<'a> {
    pc: u32,
    local_variables: Vec<u32>,
    operand_stack: Vec<Value>,
    operand_stack_depth: u32,
    constant_pool: &'a ConstantPool,
    code: &'a Code,
}

pub trait PopOperandFrame<T> {
    fn pop_operand(&mut self) -> T;
}

impl Frame<'_> {
    pub fn new<'a>(code: &'a Code, constant_pool: &'a ConstantPool) -> Frame<'a> {
        Frame {
            pc: 0,
            local_variables: vec![0; code.max_locals as usize],
            operand_stack: Vec::with_capacity(code.max_stack as usize),
            operand_stack_depth: 0,
            code,
            constant_pool,
        }
    }

    pub fn set_operand_stack(&mut self, stack: Vec<Value>) {
        self.operand_stack_depth = stack
            .as_slice()
            .iter()
            .fold(0, |sum, i| sum + i.get_category() as u32);
        self.operand_stack = stack;
    }

    pub fn set_locals(&mut self, locals: Vec<u32>) {
        self.local_variables = locals;
    }

    pub fn get_local(&self, index: u16) -> u32 {
        self.local_variables[index as usize]
    }

    pub fn get_local_long(&self, index: u16) -> u64 {
        let i1 = self.local_variables[index as usize] as u64;
        let i2 = self.local_variables[(index + 1) as usize] as u64;
        (i1 << 32) | i2
    }

    pub fn set_local(&mut self, index: u16, value: u32) {
        self.local_variables[index as usize] = value
    }

    pub fn set_local_long(&mut self, index: u16, value: u64) {
        self.local_variables[index as usize] = (value >> 32) as u32;
        self.local_variables[(index + 1) as usize] = (value & 0xFFFF_FFFF) as u32;
    }

    pub fn push_operand(&mut self, value: Value) {
        self.operand_stack_depth += match value {
            Long(_) | Double(_) => 2,
            _ => 1,
        };
        self.operand_stack.push(value);
    }

    pub fn pop_operand(&mut self) -> Value {
        if let Some(value) = self.operand_stack.pop() {
            self.operand_stack_depth -= match value {
                Long(_) | Double(_) => 2,
                _ => 1,
            };
            value
        } else {
            panic!("Tried to pop value from empty stack.");
        }
    }

    pub fn pop_operand_int(&mut self) -> i32 {
        match self.pop_operand() {
            Int(i) => i,
            op => panic!("Expected int to pop, found {:?}", op),
        }
    }

    pub fn pop_operand_long(&mut self) -> i64 {
        match self.pop_operand() {
            Long(l) => l,
            op => panic!("Expected long to pop, found {:?}", op),
        }
    }

    pub fn pop_operand_float(&mut self) -> f32 {
        match self.pop_operand() {
            Float(f) => f,
            op => panic!("Expected float to pop, found {:?}", op),
        }
    }

    pub fn pop_operand_double(&mut self) -> f64 {
        match self.pop_operand() {
            Double(d) => d,
            op => panic!("Expected double to pop, found {:?}", op),
        }
    }

    pub fn pop_operand_reference(&mut self) -> i32 {
        match self.pop_operand() {
            Reference(i) => i,
            op => panic!("Expected reference to pop, found {:?}", op),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::class::constant::ConstantPool;
    use crate::vm::{Frame, Value};

    /*
    #[test]
    fn pop_bool() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Boolean(true));

        assert_eq!(frame.pop_operand(), Value::Boolean(true));
    }

    #[test]
    fn set_local() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(0, 2, &constants);

        frame.set_local(1, 13);
        assert_eq!(frame.get_local(0), 0);
        assert_eq!(frame.get_local(1), 13);
    }
     */
}
