use crate::class::code::Instruction;
use crate::class::constant::Constant;

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub data: AttributeData,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeData {
    SourceFile(String),
    LineNumberTable(Vec<(u16, u16)>),
    CodeInfo(Code),
    ConstantValue(Constant),
    Exceptions(Vec<String>),
    Unknown(Vec<u8>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    // TODO exception table
    pub attributes: Vec<Attribute>,
    pub instructions: Vec<Instruction>,
}

impl Code {
    pub fn new(
        max_stack: u16,
        max_locals: u16,
        attributes: Vec<Attribute>,
        instructions: Vec<Instruction>,
    ) -> Code {
        Code {
            max_stack,
            max_locals,
            attributes,
            instructions,
        }
    }
}
