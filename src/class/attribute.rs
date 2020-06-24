pub mod io;

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    pub(crate) name: &'a str,
    pub(crate) data: AttributeData<'a>,
}

#[derive(Debug, PartialEq)]
pub enum AttributeData<'a> {
    SourceFile(&'a str),
    LineNumberTable(Vec<(u16, u16)>),
    CodeInfo(Code<'a>),
    Unknown(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct Code<'a> {
    max_stack: u16,
    max_locals: u16,
    // TODO code
    // TODO exception table
    attributes: Vec<Attribute<'a>>
}
