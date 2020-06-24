pub mod io;

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    pub(crate) name: &'a str,
    pub(crate) data: AttributeData<'a>,
}

#[derive(Debug, PartialEq)]
pub enum AttributeData<'a> {
    SourceFile(&'a str),
    Unknown(Vec<u8>),
}
