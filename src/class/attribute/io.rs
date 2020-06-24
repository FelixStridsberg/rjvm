use crate::class::attribute::AttributeData::{SourceFile, Unknown, LineNumberTable};
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
            "LineNumberTable" => self.read_line_number_table()?,
            _ => self.read_unknown_attribute(len)?,
        };

        Ok(Attribute { name, data })
    }

    fn read_source_file_attribute(&mut self) -> Result<AttributeData<'c>> {
        let name_index = self.reader.read_u2()?;
        Ok(SourceFile(self.constants.get_utf8(name_index)))
    }

    fn read_line_number_table(&mut self) -> Result<AttributeData<'c>> {
        let length = self.reader.read_u2()?;
        let mut table = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let start_pc = self.reader.read_u2()?;
            let line_number = self.reader.read_u2()?;
            table.push((start_pc, line_number));
        }
        Ok(LineNumberTable(table))
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
    use crate::class::attribute::AttributeData::{SourceFile, Unknown, LineNumberTable};
    use crate::class::constant::Constant::*;
    use crate::class::constant::ConstantPool;
    use std::io::{Cursor, BufRead};

    #[test]
    fn read_unknown_attribute() {
        let mut constants = ConstantPool::new(1);
        constants.add(Utf8("Unknown attribute".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x01, // Name index
            0x00, 0x00, 0x00, 0x02, // Info length
            0x01, 0x02, // Info
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
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
            0x00, 0x00, 0x00, 0x02, // Info length
            0x00, 0x01, // Info
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
            vec![Attribute {
                name: "SourceFile",
                data: SourceFile("file.java"),
            }]
        );
    }

    #[test]
    fn read_line_number_table_attribute() {
        let mut constants = ConstantPool::new(3);
        constants.add(Utf8("LineNumberTable".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x01, // Name index
            0x00, 0x00, 0x00, 0x0a, // Info length
            0x00, 0x02, // Table length
            0x00, 0x00, // 1. Start PC
            0x00, 0x05, // 1. Line number
            0x00, 0x04, // 2. Start PC
            0x00, 0x07, // Line number
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
            vec![Attribute {
                name: "LineNumberTable",
                data: LineNumberTable(vec![(0, 5), (4, 7)]),
            }]
        );
    }

    fn read_attributes<'a, R: BufRead>(r: &mut R, constants: &'a ConstantPool) -> Vec<Attribute<'a>> {
        let mut reader = AttributeReader::new(r, &constants);
        reader.read_attributes().unwrap()
    }
}
