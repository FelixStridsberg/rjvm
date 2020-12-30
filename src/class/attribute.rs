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
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub exception_handlers: Vec<ExceptionHandler>,
    pub attributes: Vec<Attribute>,
    pub instructions: Vec<Instruction>,
}

impl Code {
    pub fn new(
        max_stack: u16,
        max_locals: u16,
        exception_handlers: Vec<ExceptionHandler>,
        attributes: Vec<Attribute>,
        instructions: Vec<Instruction>,
    ) -> Code {
        Code {
            max_stack,
            max_locals,
            exception_handlers,
            attributes,
            instructions,
        }
    }
}
