use crate::class::attribute::{Code, ExceptionHandler};
use crate::class::code::Opcode::OperationSpacer;
use crate::class::{Class, MethodInfo};
use crate::vm::data_type::Value::*;
use crate::vm::data_type::{FieldType, Value};
use crate::vm::Object;
use core::fmt;
use std::cmp::{max, min};
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug)]
pub struct Frame {
    pub pc: u32,      // TODO this should be u16
    pub last_pc: u32, // TODO refactor so this is not required
    pub local_variables: Vec<u32>,
    pub operand_stack: Vec<Value>,
    pub operand_stack_depth: u32,
    pub class: Rc<Class>,
    pub method: Rc<MethodInfo>,
    pub code: Rc<Code>,
}

impl Frame {
    pub fn new(class: Rc<Class>, method: Rc<MethodInfo>) -> Frame {
        let code = method.get_code().expect("No Code attribute on method.");

        Frame {
            pc: 0,
            last_pc: 0,
            local_variables: vec![0; code.max_locals as usize],
            operand_stack: Vec::with_capacity(code.max_stack as usize),
            operand_stack_depth: 0,
            class,
            method,
            code,
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

    pub fn handle_exception(&mut self, exception: &Object) -> bool {
        if let Some(handler) = self.find_exception_handler(exception) {
            self.pc = handler.handler_pc as u32;
            true
        } else {
            false
        }
    }

    fn find_exception_handler(&self, exception: &Object) -> Option<&ExceptionHandler> {
        // TODO finally block catch_type is empty
        self.code.exception_handlers.iter().find(|e| {
            e.catch_type == exception.class
                && (self.last_pc as u16) >= e.start_pc
                && (self.last_pc as u16) < e.end_pc
        })
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}::{}", self.class.this_class, self.method.name)?;

        let code_start = max(self.pc as usize, 5) - 5;
        let code_end = min(self.pc as usize + 5, self.code.instructions.len());

        for i in code_start..code_end {
            let instruction = &self.code.instructions[i];
            if instruction.opcode == OperationSpacer {
                continue;
            }

            if self.pc == i as u32 {
                write!(f, "> ")?;
            } else {
                write!(f, "  ")?;
            }
            writeln!(f, "{:<5}{}", i, instruction)?;
        }

        writeln!(f, "Operands: {:?}", self.operand_stack)?;
        writeln!(f, "Locals: {:?}", self.local_variables)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::class::attribute::Code;
    use crate::class::constant::ConstantPool;
    use crate::class::{Class, MethodInfo};
    use crate::vm::data_type::Value;
    use crate::vm::frame::Frame;
    use std::rc::Rc;

    #[test]
    fn pop_bool() {
        let constants = ConstantPool::new(0);
        let class = Class::from_constant_pool(constants);
        let method = MethodInfo::from_code(Code::new(1, 0, vec![], vec![], vec![]));
        let mut frame = Frame::new(Rc::new(class), Rc::new(method));
        frame.push_operand(Value::Boolean(true));

        assert_eq!(frame.pop_operand(), Value::Boolean(true));
    }

    #[test]
    fn set_local() {
        let constants = ConstantPool::new(0);
        let class = Class::from_constant_pool(constants);
        let method = MethodInfo::from_code(Code::new(0, 2, vec![], vec![], vec![]));
        let mut frame = Frame::new(Rc::new(class), Rc::new(method));

        frame.set_local(1, 13);
        assert_eq!(frame.get_local(0), 0);
        assert_eq!(frame.get_local(1), 13);
    }
}
