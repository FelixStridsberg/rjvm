use crate::class::constant::Constant;
use crate::class::code::Instruction;

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    pub name: &'a str,
    pub data: AttributeData<'a>,
}

#[derive(Debug, PartialEq)]
pub enum AttributeData<'a> {
    SourceFile(&'a str),
    LineNumberTable(Vec<(u16, u16)>),
    CodeInfo(Code<'a>),
    ConstantValue(&'a Constant),
    Exceptions(Vec<&'a str>),
    Unknown(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct Code<'a> {
    pub max_stack: u16,
    pub max_locals: u16,
    // TODO exception table
    pub attributes: Vec<Attribute<'a>>,
    pub instructions: Vec<Instruction>,
}
