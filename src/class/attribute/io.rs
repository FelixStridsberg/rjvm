use crate::class::attribute::AttributeData::{SourceFile, Unknown, LineNumberTable, CodeInfo};
use crate::class::attribute::{Attribute, AttributeData, Code};
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
            "LineNumberTable" => self.read_line_number_table_attribute()?,
            "Code" => self.read_code_attribute()?,
            _ => self.read_unknown_attribute(len)?,
        };

        Ok(Attribute { name, data })
    }

    fn read_source_file_attribute(&mut self) -> Result<AttributeData<'c>> {
        let name_index = self.reader.read_u2()?;
        Ok(SourceFile(self.constants.get_utf8(name_index)))
    }

    fn read_code_attribute(&mut self) -> Result<AttributeData<'c>> {
        let max_stack = self.reader.read_u2()?;
        let max_locals = self.reader.read_u2()?;

        let code_length = self.reader.read_u4()?;

        // TODO code reader
        let mut _code = Vec::with_capacity(code_length as usize);
        unsafe {
            _code.set_len(code_length as usize);
        }
        self.reader.read_exact(&mut _code)?;

        let exception_table_length = self.reader.read_u2()?;
        if exception_table_length > 0 {
            panic!("Reading exception table not implemented.");
        }
        // TODO read exception table

        let attributes = self.read_attributes()?;

        Ok(CodeInfo(Code {
            max_stack,
            max_locals,
            attributes,
        }))
    }

    fn read_line_number_table_attribute(&mut self) -> Result<AttributeData<'c>> {
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
    use crate::class::attribute::{Attribute, Code};
    use crate::class::attribute::AttributeData::{SourceFile, Unknown, LineNumberTable, CodeInfo};
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
        let mut constants = ConstantPool::new(2);
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
        let mut constants = ConstantPool::new(1);
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

    #[test]
    fn read_code_attribute() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("Code".to_owned()));
        constants.add(Utf8("LineNumberTable".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Attribute count
            0x00, 0x01, // Name index (Code)
            0x00, 0x00, 0x00, 0x28, // Length
            0x00, 0x03, // Max stack
            0x00, 0x01, // Max locals
            0x00, 0x00, 0x00, 0x0c, // Code length
            0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x14, 0x00, 0x02, 0xb5, 0x00, 0x04, 0xb1, // Code
            0x00, 0x00, // Exception table length
            // (No exception table)
            0x00, 0x01, // Attributes count
            0x00, 0x02, // Attribute name index
            0x00, 0x00, 0x00, 0x0a, // Attribute length
            0x00, 0x02, // LineNumberTableLength
            0x00, 0x00, // Start PC
            0x00, 0x05, // Line number
            0x00, 0x04, // Start PC
            0x00, 0x07, // Line number
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
            vec![Attribute {
                name: "Code",
                data: CodeInfo(
                    Code {
                        max_stack: 3,
                        max_locals: 1,
                        attributes: vec![Attribute {
                            name: "LineNumberTable",
                            data: LineNumberTable(vec![(0, 5), (4, 7)]),
                        }]
                    }
                ),
            }]
        );
    }

    fn read_attributes<'a, R: BufRead>(r: &mut R, constants: &'a ConstantPool) -> Vec<Attribute<'a>> {
        let mut reader = AttributeReader::new(r, &constants);
        reader.read_attributes().unwrap()
    }
}
