use crate::class::attribute::AttributeData::{SourceFile, Unknown};
use crate::class::attribute::{Attribute, AttributeData};
use crate::class::constant::ConstantPool;
use crate::class::io::ReadBytesExt;
use crate::error::Result;
use std::io::BufRead;

pub struct AttributeReader<'r, 'c, R: BufRead> {
    reader: &'r mut R,
    constants: &'c ConstantPool,
}

impl<'r, 'c, R: BufRead> AttributeReader<'r, 'c, R> {
    pub fn new(reader: &'r mut R, constants: &'c ConstantPool) -> AttributeReader<'r, 'c, R> {
        AttributeReader { reader, constants }
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

    fn read_unknown_attribute(&mut self, len: usize) -> Result<AttributeData<'c>> {
        let mut info = Vec::with_capacity(len);
        unsafe {
            info.set_len(len);
        }
        self.reader.read_exact(&mut info)?;

        Ok(Unknown(info))
    }
}

#[cfg(test)]
mod test {
    use crate::class::attribute::io::AttributeReader;
    use crate::class::attribute::Attribute;
    use crate::class::attribute::AttributeData::{SourceFile, Unknown};
    use crate::class::constant::Constant::*;
    use crate::class::constant::ConstantPool;
    use std::io::Cursor;

    #[test]
    fn read_unknown_attribute() {
        let mut constants = ConstantPool::new(1);
        constants.add(Utf8("Unknown attribute".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x01, // Name index
            0x00, 0x00, 0x00, 0x02, 0x01, 0x02, // Info
        ]);

        let mut reader = AttributeReader::new(&mut data, &constants);
        let attributes = reader.read_attributes().unwrap();

        assert_eq!(
            attributes,
            vec![Attribute {
                name: "Unknown attribute",
                data: Unknown(vec![0x01, 0x02])
            }]
        );
    }

    #[test]
    fn read_source_file_attribute() {
        let mut constants = ConstantPool::new(3);
        constants.add(Utf8("file.java".to_owned()));
        constants.add(Utf8("SourceFile".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x02, // Name index
            0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // Info
        ]);

        let mut reader = AttributeReader::new(&mut data, &constants);
        let attributes = reader.read_attributes().unwrap();

        assert_eq!(
            attributes,
            vec![Attribute {
                name: "SourceFile",
                data: SourceFile("file.java"),
            }]
        );
    }
}
