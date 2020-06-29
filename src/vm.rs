use crate::class::constant::ConstantPool;
use crate::vm::Value::*;

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    Char(char),
    Float(f32),
    Double(f64),
    Reference, // TODO
    ReturnAddress, // TODO
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

    pub fn pop_operand_bool(&mut self) -> bool {
        let boxed = self.operand_stack.pop();
        if let Some(Boolean(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop a bool from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_byte(&mut self) -> u8 {
        let boxed = self.operand_stack.pop();
        if let Some(Byte(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop a byte from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_short(&mut self) -> u16 {
        let boxed = self.operand_stack.pop();
        if let Some(Short(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop a short from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_int(&mut self) -> u32 {
        let boxed = self.operand_stack.pop();
        if let Some(Int(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop an int from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_long(&mut self) -> u64 {
        let boxed = self.operand_stack.pop();
        if let Some(Long(value)) = boxed {
            self.operand_stack_depth -= 2;
            value
        } else {
            panic!(
                "Tried to pop a long from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_char(&mut self) -> char {
        let boxed = self.operand_stack.pop();
        if let Some(Char(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop a char from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_float(&mut self) -> f32 {
        let boxed = self.operand_stack.pop();
        if let Some(Float(value)) = boxed {
            self.operand_stack_depth -= 1;
            value
        } else {
            panic!(
                "Tried to pop a float from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_double(&mut self) -> f64 {
        let boxed = self.operand_stack.pop();
        if let Some(Double(value)) = boxed {
            self.operand_stack_depth -= 2;
            value
        } else {
            panic!(
                "Tried to pop a double from operand stack, got a {:?}.",
                boxed
            )
        }
    }

    pub fn pop_operand_reference(&mut self) -> () {
        unimplemented!()
    }

    pub fn pop_operand_return_address(&mut self) -> () {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::class::constant::ConstantPool;
    use crate::vm::{Frame, Value, PopOperandFrame};

    #[test]
    fn pop_bool() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Boolean(true));

        assert_eq!(frame.pop_operand_bool(), true);
    }

    #[test]
    fn pop_byte() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Byte(0x42));

        assert_eq!(frame.pop_operand_byte(), 0x42);
    }

    #[test]
    fn pop_short() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Short(16));

        assert_eq!(frame.pop_operand_short(), 16);
    }

    #[test]
    fn pop_int() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Int(1000));

        assert_eq!(frame.pop_operand_int(), 1000);
    }

    #[test]
    fn pop_long() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Long(0x42));

        assert_eq!(frame.pop_operand_long(), 0x42);
    }

    #[test]
    fn pop_char() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Char('a'));

        assert_eq!(frame.pop_operand_char(), 'a');
    }

    #[test]
    fn pop_float() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Float(3.14));

        assert_eq!(frame.pop_operand_float(), 3.14);
    }

    #[test]
    fn pop_double() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 0, &constants);
        frame.push_operand(Value::Double(3.14));

        assert_eq!(frame.pop_operand_double(), 3.14);
    }
}
