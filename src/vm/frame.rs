use crate::class::attribute::Code;
use crate::class::constant::ConstantPool;
use crate::vm::data_type::Value::*;
use crate::vm::data_type::{FieldType, Value};
use core::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Frame<'a> {
    pub pc: u32,
    pub local_variables: Vec<u32>,
    pub operand_stack: Vec<Value>,
    pub operand_stack_depth: u32,
    pub constant_pool: &'a ConstantPool,
    pub code: &'a Code,
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

    pub fn load_arguments(&mut self, args: Vec<Value>) {
        let mut index = 0;
        for arg in args {
            if arg.get_category() == 1 {
                self.set_local(index, arg.as_int_value());
                index += 1;
            } else {
                self.set_local_long(index, arg.as_long_value());
                index += 2;
            }
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

    pub fn pop_field_types(&mut self, types: &[FieldType]) -> Vec<Value> {
        let mut values = Vec::with_capacity(types.len());

        for field_type in types {
            values.push(self.pop_operand().expect_type(&field_type));
        }
        values
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
}

impl fmt::Display for Frame<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PC {}", self.pc)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::class::attribute::Code;
    use crate::class::constant::ConstantPool;
    use crate::vm::data_type::Value;
    use crate::vm::frame::Frame;

    #[test]
    fn pop_bool() {
        let constants = ConstantPool::new(0);
        let code = Code::new(1, 0, vec![], vec![]);
        let mut frame = Frame::new(&code, &constants);
        frame.push_operand(Value::Boolean(true));

        assert_eq!(frame.pop_operand(), Value::Boolean(true));
    }

    #[test]
    fn set_local() {
        let constants = ConstantPool::new(0);
        let code = Code::new(0, 2, vec![], vec![]);
        let mut frame = Frame::new(&code, &constants);

        frame.set_local(1, 13);
        assert_eq!(frame.get_local(0), 0);
        assert_eq!(frame.get_local(1), 13);
    }
}
