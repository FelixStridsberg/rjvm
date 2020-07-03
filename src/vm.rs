use crate::class::constant::ConstantPool;
use crate::class::MethodInfo;
use crate::vm::interpreter::interpret;
use crate::vm::Value::*;

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
}

impl Value {
    pub fn get_category(&self) -> u8 {
        match self {
            Long(_) | Double(_) => 2,
            _ => 1
        }
    }
}

pub struct VirtualMachine<'a> {
    stack: Vec<Frame<'a>>,
}

impl VirtualMachine<'_> {
    pub fn execute_method(constants: &ConstantPool, method: &MethodInfo) {
        let code = method.get_code().expect("No Code attribute on method.");

        let mut frame = Frame::new(code.max_stack, code.max_locals, constants);
        interpret(&mut frame, &code.instructions);
    }
}

pub struct Frame<'a> {
    local_variables: Vec<u32>,
    operand_stack: Vec<Value>,
    operand_stack_depth: u32,
    constant_pool: &'a ConstantPool,
}

pub trait PopOperandFrame<T> {
    fn pop_operand(&mut self) -> T;
}

impl Frame<'_> {
    pub fn new(stack: u16, locals: u16, constant_pool: &ConstantPool) -> Frame {
        Frame {
            local_variables: vec![0; locals as usize],
            operand_stack: Vec::with_capacity(stack as usize),
            operand_stack_depth: 0,
            constant_pool,
        }
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
        self.local_variables[(index + 1) as usize] = (value & 0xFFFFFFFF) as u32;
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
}

#[cfg(test)]
mod test {
    use crate::class::constant::ConstantPool;
    use crate::vm::{Frame, Value};

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
}
