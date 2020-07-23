use crate::vm::frame::Frame;
use bitflags::_core::fmt::Formatter;
use std::fmt;

#[derive(Debug)]
pub struct Stack<'a> {
    stack: Vec<Frame<'a>>,
}

impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack { stack: Vec::new() }
    }

    pub fn last_frame(&self) -> bool {
        self.stack.len() == 1
    }

    pub fn push(&mut self, frame: Frame<'a>) {
        self.stack.push(frame);
    }

    pub fn pop(&mut self) -> Frame {
        self.stack.pop().expect("Tried to pop from empty stack.")
    }

    pub fn current_frame(&mut self) -> &mut Frame<'a> {
        self.stack
            .last_mut()
            .expect("Tried to get current frame on empty stack.")
    }
}

impl fmt::Display for Stack<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for frame in &self.stack {
            writeln!(f, "Frame {}", frame)?
        }
        Ok(())
    }
}
