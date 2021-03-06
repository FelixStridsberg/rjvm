use crate::vm::frame::Frame;
use bitflags::_core::fmt::Formatter;
use std::fmt;

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn last_frame(&self) -> bool {
        self.stack.len() == 1
    }

    pub fn push(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    pub fn append(&mut self, other: &mut Vec<Frame>) {
        self.stack.append(other);
    }

    pub fn pop(&mut self) -> Frame {
        self.stack.pop().expect("Tried to pop from empty stack.")
    }

    pub fn current_frame(&self) -> &Frame {
        self.stack
            .last()
            .expect("Tried to get current frame on empty stack.")
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        self.stack
            .last_mut()
            .expect("Tried to get current frame on empty stack.")
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, frame) in self.stack.iter().enumerate() {
            writeln!(f, "#{} {}", i, frame)?
        }
        Ok(())
    }
}
