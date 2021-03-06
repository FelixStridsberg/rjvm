use crate::class::attribute::AttributeData::{
    CodeInfo, ConstantValue, Exceptions, LineNumberTable, SourceFile, Unknown,
};
use crate::class::attribute::{Attribute, AttributeData, Code, ExceptionHandler};
use crate::class::constant::ConstantPool;
use crate::error::Result;
use crate::io::code::CodeReader;
use crate::io::ReadBytesExt;
use std::io::BufRead;

pub trait AttributeRead {}

pub struct AttributeReader<'r, 'c, R: BufRead> {
    reader: &'r mut R,
    constants: &'c ConstantPool,
}

impl<'r, 'c, R: BufRead> AttributeReader<'r, 'c, R> {
    pub fn new(reader: &'r mut R, constants: &'c ConstantPool) -> AttributeReader<'r, 'c, R> {
        AttributeReader { reader, constants }
    }

    pub fn read_attributes(&mut self) -> Result<Vec<Attribute>> {
        let attribute_len = self.reader.read_u2()? as usize;
        let mut attributes = Vec::with_capacity(attribute_len);
        for _ in 0..attribute_len {
            attributes.push(self.read_attribute_info()?)
        }
        Ok(attributes)
    }

    fn read_attribute_info(&mut self) -> Result<Attribute> {
        let name = self.constants.get_utf8(self.reader.read_u2()?)?.to_owned();
        let len = self.reader.read_u4()? as usize;
        let data = match &name[..] {
            "SourceFile" => self.read_source_file_attribute()?,
            "LineNumberTable" => self.read_line_number_table_attribute()?,
            "Code" => self.read_code_attribute()?,
            "ConstantValue" => self.read_constant_value_attribute()?,
            "Exceptions" => self.read_exceptions_attribute()?,
            _ => self.read_unknown_attribute(len)?,
        };

        Ok(Attribute { name, data })
    }

    fn read_source_file_attribute(&mut self) -> Result<AttributeData> {
        let name_index = self.reader.read_u2()?;
        Ok(SourceFile(self.constants.get_utf8(name_index)?.to_owned()))
    }

    fn read_constant_value_attribute(&mut self) -> Result<AttributeData> {
        let value_index = self.reader.read_u2()?;
        Ok(ConstantValue((*self.constants.get(value_index)).clone()))
    }

    fn read_exceptions_attribute(&mut self) -> Result<AttributeData> {
        let exception_count = self.reader.read_u2()?;
        let mut exceptions = Vec::with_capacity(exception_count as usize);

        for _ in 0..exception_count {
            exceptions.push(
                self.constants
                    .get_class_info_name(self.reader.read_u2()?)?
                    .to_owned(),
            );
        }

        Ok(Exceptions(exceptions))
    }

    fn read_code_attribute(&mut self) -> Result<AttributeData> {
        let max_stack = self.reader.read_u2()?;
        let max_locals = self.reader.read_u2()?;

        let mut code_reader = CodeReader::new(&mut self.reader);
        let instructions = code_reader.read_code()?;

        let exception_table_length = self.reader.read_u2()?;
        let exception_handlers = if exception_table_length > 0 {
            self.read_code_exception_table(exception_table_length)?
        } else {
            vec![]
        };

        let attributes = self.read_attributes()?;

        Ok(CodeInfo(Code {
            max_stack,
            max_locals,
            exception_handlers,
            attributes,
            instructions,
        }))
    }

    fn read_code_exception_table(&mut self, length: u16) -> Result<Vec<ExceptionHandler>> {
        let mut result = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let start_pc = self.reader.read_u2()?;
            let end_pc = self.reader.read_u2()?;
            let handler_pc = self.reader.read_u2()?;

            let catch_type_index = self.reader.read_u2()?;
            let catch_type = if catch_type_index == 0 {
                None
            } else {
                Some(
                    self.constants
                        .get_class_info_name(catch_type_index)?
                        .to_owned(),
                )
            };

            result.push(ExceptionHandler {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }

        Ok(result)
    }

    fn read_line_number_table_attribute(&mut self) -> Result<AttributeData> {
        let length = self.reader.read_u2()?;
        let mut table = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let start_pc = self.reader.read_u2()?;
            let line_number = self.reader.read_u2()?;
            table.push((start_pc, line_number));
        }
        Ok(LineNumberTable(table))
    }

    fn read_unknown_attribute(&mut self, len: usize) -> Result<AttributeData> {
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
    use crate::class::attribute::AttributeData::{
        CodeInfo, ConstantValue, Exceptions, LineNumberTable, SourceFile, Unknown,
    };
    use crate::class::attribute::{Attribute, Code, ExceptionHandler};
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::Nop;
    use crate::class::constant::Constant::*;
    use crate::class::constant::ConstantPool;
    use crate::io::attribute::AttributeReader;
    use std::io::{BufRead, Cursor};

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
                name: "Unknown attribute".to_owned(),
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
                name: "SourceFile".to_owned(),
                data: SourceFile("file.java".to_owned()),
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
                name: "LineNumberTable".to_owned(),
                data: LineNumberTable(vec![(0, 5), (4, 7)]),
            }]
        );
    }

    #[test]
    fn read_constant_value_attribute() {
        let mut constants = ConstantPool::new(1);
        constants.add(Utf8("ConstantValue".to_owned()));
        constants.add(Long(10));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x01, // Name index
            0x00, 0x00, 0x00, 0x02, // Info length
            0x00, 0x02, // Constant value index
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
            vec![Attribute {
                name: "ConstantValue".to_owned(),
                data: ConstantValue(Long(10)),
            }]
        );
    }

    #[test]
    fn read_exceptions_attribute() {
        let mut constants = ConstantPool::new(1);
        constants.add(Utf8("Exceptions".to_owned()));
        constants.add(ClassRef(3));
        constants.add(Utf8("java/lang/Exception".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Count
            0x00, 0x01, // Name index
            0x00, 0x00, 0x00, 0x04, // Info length
            0x00, 0x01, // Number of exceptions
            0x00, 0x02, // Exception index
        ]);

        assert_eq!(
            read_attributes(&mut data, &constants),
            vec![Attribute {
                name: "Exceptions".to_owned(),
                data: Exceptions(vec!["java/lang/Exception".to_owned()]),
            }]
        );
    }

    #[test]
    fn read_code_attribute() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("Code".to_owned()));
        constants.add(Utf8("LineNumberTable".to_owned()));
        constants.add(ClassRef(4));
        constants.add(Utf8("java/lang/Exception".to_owned()));

        let mut data = Cursor::new(vec![
            0x00, 0x01, // Attribute count
            0x00, 0x01, // Name index (Code)
            0x00, 0x00, 0x00, 0x28, // Length
            0x00, 0x03, // Max stack
            0x00, 0x01, // Max locals
            0x00, 0x00, 0x00, 0x01, // Code length
            0x00, // Code: nop
            0x00, 0x02, // Exception table length
            0x00, 0x00, // Start pc = 0
            0x00, 0x03, // End pc = 3
            0x00, 0x06, // Handler pc = 6
            0x00, 0x03, // Catch type
            0x00, 0x00, // Start pc = 0
            0x00, 0x03, // End pc = 3
            0x00, 0x0a, // Handler pc = 10
            0x00, 0x00, // Catch type = 0, finally block
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
                name: "Code".to_owned(),
                data: CodeInfo(Code {
                    max_stack: 3,
                    max_locals: 1,
                    exception_handlers: vec![
                        ExceptionHandler {
                            start_pc: 0,
                            end_pc: 3,
                            handler_pc: 6,
                            catch_type: Some("java/lang/Exception".to_string())
                        },
                        ExceptionHandler {
                            start_pc: 0,
                            end_pc: 3,
                            handler_pc: 10,
                            catch_type: None,
                        }
                    ],
                    attributes: vec![Attribute {
                        name: "LineNumberTable".to_owned(),
                        data: LineNumberTable(vec![(0, 5), (4, 7)]),
                    }],
                    instructions: vec![Instruction::new(Nop, vec![])]
                }),
            }]
        );
    }

    fn read_attributes<R: BufRead>(r: &mut R, constants: &ConstantPool) -> Vec<Attribute> {
        let mut reader = AttributeReader::new(r, &constants);
        reader.read_attributes().unwrap()
    }
}
