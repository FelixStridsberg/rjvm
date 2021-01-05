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
    pub pc: u16,
    pub local_variables: Vec<Option<Value>>,
    pub operand_stack: Vec<Value>,
    pub operand_stack_depth: u32,
    pub class: Rc<Class>,
    pub method: Rc<MethodInfo>,
    pub code: Option<Rc<Code>>,
    pub implicit: bool, // Implicit frames are created by the VM and not by java code.
}

impl Frame {
    pub fn new(class: Rc<Class>, method: Rc<MethodInfo>) -> Frame {
        let code = method.get_code();
        let max_locals = code.as_ref().map_or(10, |c| c.max_locals);
        let max_stack = code.as_ref().map_or(10, |c| c.max_stack);

        Frame {
            pc: 0,
            local_variables: vec![None; max_locals as usize],
            operand_stack: Vec::with_capacity(max_stack as usize),
            operand_stack_depth: 0,
            class,
            method,
            code,
            implicit: false,
        }
    }

    pub fn pc_next(&mut self) {
        self.pc += self.code.as_ref().unwrap().instructions[self.pc as usize].size();
    }

    pub fn pc_offset(&mut self, offset: i16) {
        let new_pc = (self.pc as i16 + offset) as u16;
        debug!("[F] PC: {} -> {}", self.pc, new_pc);
        self.pc = new_pc;
    }

    pub fn pc_offset_wide(&mut self, offset: i32) {
        self.pc = (self.pc as i32 + offset) as u16;
    }

    pub fn load_arguments(&mut self, args: Vec<Value>) {
        let mut index = 0;
        for arg in args {
            if arg.get_category() == 1 {
                self.set_local(index, arg);
                index += 1;
            } else {
                self.set_local(index, arg);
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

    pub fn set_locals(&mut self, locals: Vec<Value>) {
        self.local_variables = locals.into_iter().map(Option::Some).collect();
    }

    pub fn get_local(&self, index: u16) -> Value {
        self.local_variables[index as usize]
            .clone()
            .expect("Illegal use of an uninitialized local")
    }

    pub fn set_local(&mut self, index: u16, value: Value) {
        self.local_variables[index as usize] = Some(value)
    }

    pub fn pop_field_types(&mut self, types: &[FieldType]) -> Vec<Value> {
        let mut values = Vec::with_capacity(types.len());

        for field_type in types.iter().rev() {
            values.push(self.pop_operand().expect_type(&field_type));
        }
        values.reverse();
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
            self.pc = handler.handler_pc;
            true
        } else {
            false
        }
    }

    fn find_exception_handler(&self, exception: &Object) -> Option<&ExceptionHandler> {
        if self.code.is_none() {
            return None;
        }

        self.code
            .as_ref()
            .unwrap()
            .exception_handlers
            .iter()
            .find(|e| {
                if let Some(catch_type) = &e.catch_type {
                    catch_type == &exception.class
                        && (self.pc as u16) >= e.start_pc
                        && (self.pc as u16) < e.end_pc
                } else {
                    false
                }
            })
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}::{}", self.class.this_class, self.method.name)?;

        if self.code.is_none() {
            writeln!(f, "<Native>")?;
            return Ok(());
        }

        let code = self.code.as_ref().unwrap();
        let code_start = max(self.pc as usize, 5) - 5;
        let code_end = min(self.pc as usize + 5, code.instructions.len());

        for i in code_start..code_end {
            let instruction = &code.instructions[i];
            if instruction.opcode == OperationSpacer {
                continue;
            }

            if self.pc == i as u16 {
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
    use crate::vm::data_type::Value::Int;
    use crate::vm::frame::Frame;
    use std::rc::Rc;

    #[test]
    fn set_local() {
        let constants = ConstantPool::new(0);
        let class = Class::from_constant_pool(constants);
        let method = MethodInfo::from_code(Code::new(0, 2, vec![], vec![], vec![]));
        let mut frame = Frame::new(Rc::new(class), Rc::new(method));

        frame.set_local(1, Int(13));
        assert_eq!(frame.get_local(1), Int(13));
    }

    #[test]
    #[should_panic]
    fn get_uninitialized_local() {
        let constants = ConstantPool::new(0);
        let class = Class::from_constant_pool(constants);
        let method = MethodInfo::from_code(Code::new(0, 2, vec![], vec![], vec![]));
        let frame = Frame::new(Rc::new(class), Rc::new(method));

        frame.get_local(0); // Will panic, has not been initialized
    }
}
