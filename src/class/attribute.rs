use std::io::BufRead;
use crate::error::Result;
use crate::class::constant::ConstantPool;
use crate::class::attribute::AttributeData::{SourceFile, Unknown};
use crate::class::io::ReadBytesExt;

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

pub struct AttributeReader<'r, 'c, R: BufRead> {
    reader: &'r mut R,
    constants: &'c ConstantPool,
}

impl <'r, 'c, R: BufRead> AttributeReader<'r, 'c, R> {
    pub fn new(reader: &'r mut R, constants: &'c ConstantPool) -> AttributeReader<'r, 'c, R> {
        AttributeReader {
            reader,
            constants,
        }
    }

    pub fn read_attributes(&mut self) -> Result<Vec<Attribute<'c>>> {
        let attribute_len = self.reader.read_u2()? as usize;
        let mut attributes = Vec::with_capacity(attribute_len);
        for _ in 0..attribute_len {
            attributes.push(self.read_attribute_info()?)
        }
        Ok(attributes)
    }

    fn read_attribute_info(&mut self) -> Result<Attribute<'c>> {
        let name = self.constants.get_utf8(self.reader.read_u2()?);
        let len = self.reader.read_u4()? as usize;
        let data = match name {
            "SourceFile" => self.read_source_file_attribute()?,
            _ => self.read_unknown_attribute(len)?,
        };

        Ok(Attribute { name, data })
    }

    fn read_source_file_attribute(&mut self) -> Result<AttributeData<'c>> {
        let name_index = self.reader.read_u2()?;
        Ok(SourceFile(self.constants.get_utf8(name_index)))
    }

    fn read_unknown_attribute(
        &mut self,
        len: usize
    ) -> Result<AttributeData<'c>> {
        let mut info = Vec::with_capacity(len);
        unsafe {
            info.set_len(len);
        }
        self.reader.read_exact(&mut info)?;

        Ok(Unknown(info))
    }
}