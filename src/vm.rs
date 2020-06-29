use std::any::Any;
use crate::class::constant::ConstantPool;

#[derive(Debug)]
pub enum ValueType {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    ReturnAddress,
}

#[derive(Debug)]
pub struct Value {
    value_type: ValueType,
    value: Box<dyn Any>, // TODO this is probably not the best idea
}

pub struct VirtualMachine<'a> {
    stack: Vec<Frame<'a>>,
}

struct Frame<'a> {
    local_variables: Vec<u32>,
    operand_stack: Vec<Value>,
    operand_stack_depth: u32,
    constant_pool: &'a ConstantPool
}
