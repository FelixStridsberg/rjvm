use crate::class::constant::ConstantPool;
use crate::vm::Value::*;

#[derive(Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    Char(char),
    Float(f32),
    Double(f64),
    Reference(i32),
    ReturnAddress(i32),
}

pub struct VirtualMachine<'a> {
    stack: Vec<Frame<'a>>,
}

struct Frame<'a> {
    local_variables: Vec<u32>,
    operand_stack: Vec<Value>,
    operand_stack_depth: u32,
    constant_pool: &'a ConstantPool,
}

pub trait PopOperandFrame<T> {
    fn pop_operand(&mut self) -> T;
}

impl Frame<'_> {
    pub fn new(stack: usize, locals: usize, constant_pool: &ConstantPool) -> Frame {
        Frame {
            local_variables: Vec::with_capacity(locals),
            operand_stack: Vec::with_capacity(stack),
            operand_stack_depth: 0,
            constant_pool,
        }
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
            value
        } else {
            panic!("Tried to pop value from empty stack.");
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
}
