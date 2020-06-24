use crate::class::constant::Constant::*;
use crate::class::constant::MethodHandleKind::*;
use crate::class::constant::{Constant, ConstantPool};
use crate::class::AttributeData::{SourceFile, Unknown};
use crate::class::FieldAccessFlags;
use crate::class::MethodAccessFlags;
use crate::class::{Attribute, AttributeData, ClassAccessFlags};
use crate::class::{Class, FieldInfo, MethodInfo, Version};
use crate::error::ErrorKind::ParseError;
use crate::error::{Error, Result};
use std::io::BufRead;

const SIGNATURE: &[u8] = &[0xCA, 0xFE, 0xBA, 0xBE];

pub struct ClassReader<R: BufRead> {
    reader: R,
    version: Option<Version>,
}

impl<R: BufRead> ClassReader<R> {
    pub fn new(reader: R) -> Self {
        ClassReader {
            reader,
            version: None,
        }
    }

    pub fn verify_meta(&mut self) -> Result<()> {
        self.read_signature()?;
        self.version = Some(self.read_version()?);
        Ok(())
    }

    pub fn read_constant_pool(&mut self) -> Result<ConstantPool> {
        Ok(self.read_constants()?)
    }

    pub fn read_class(mut self, constants: &ConstantPool) -> Result<Class> {
        let access_flags = self.read_access_flags()?;
        let this_class = constants.get_class_info_name(self.read_u16()?);
        let super_class = constants.get_class_info_name(self.read_u16()?);
        let interfaces = self.read_interfaces(constants)?;
        let fields = self.read_fields(constants)?;
        let methods = self.read_methods(constants)?;
        let attributes = self.read_attributes(constants)?;

        Ok(Class {
            version: self.version.unwrap(),
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    fn read_fields<'a>(&mut self, constants: &'a ConstantPool) -> Result<Vec<FieldInfo<'a>>> {
        let len = self.read_u16()?;
        let mut fields = Vec::with_capacity(len as usize);
        for _ in 0..len {
            fields.push(self.read_field(constants)?);
        }
        Ok(fields)
    }

    fn read_attributes<'a>(&mut self, constants: &'a ConstantPool) -> Result<Vec<Attribute<'a>>> {
        let attribute_len = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attribute_len);
        for _ in 0..attribute_len {
            attributes.push(self.read_attribute_info(constants)?)
        }
        Ok(attributes)
    }

    fn read_field<'a>(&mut self, constants: &'a ConstantPool) -> Result<FieldInfo<'a>> {
        let access_flags = FieldAccessFlags::from_bits(self.read_u16()?).unwrap();
        let name = constants.get_utf8(self.read_u16()?);
        let descriptor = constants.get_utf8(self.read_u16()?);
        let attributes = self.read_attributes(constants)?;

        Ok(FieldInfo {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }

    fn read_methods<'a>(&mut self, constants: &'a ConstantPool) -> Result<Vec<MethodInfo<'a>>> {
        let len = self.read_u16()?;
        let mut fields = Vec::with_capacity(len as usize);
        for _ in 0..len {
            fields.push(self.read_method(constants)?);
        }
        Ok(fields)
    }

    fn read_method<'a>(&mut self, constants: &'a ConstantPool) -> Result<MethodInfo<'a>> {
        let access_flags = MethodAccessFlags::from_bits(self.read_u16()?).unwrap();
        let name = constants.get_utf8(self.read_u16()?);
        let descriptor = constants.get_utf8(self.read_u16()?);
        let attributes = self.read_attributes(constants)?;

        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }

    fn read_attribute_info<'a>(&mut self, constants: &'a ConstantPool) -> Result<Attribute<'a>> {
        let name = constants.get_utf8(self.read_u16()?);
        let len = self.read_u32()? as usize;
        let data = match name {
            "SourceFile" => self.read_source_file_attribute(constants)?,
            _ => self.read_unknown_attribute(len, constants)?,
        };

        Ok(Attribute { name, data })
    }

    fn read_source_file_attribute<'a>(
        &mut self,
        constants: &'a ConstantPool,
    ) -> Result<AttributeData<'a>> {
        let name_index = self.read_u16()?;
        Ok(SourceFile(constants.get_utf8(name_index)))
    }

    fn read_unknown_attribute<'a>(
        &mut self,
        len: usize,
        _constants: &'a ConstantPool,
    ) -> Result<AttributeData<'a>> {
        let mut info = Vec::with_capacity(len);
        unsafe {
            info.set_len(len);
        }
        self.reader.read_exact(&mut info)?;

        Ok(Unknown(info))
    }

    fn read_signature(&mut self) -> Result<()> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;

        if !bytes.eq(SIGNATURE) {
            return Err(Error::new(
                ParseError,
                Some("Invalid file signature.".to_owned()),
            ));
        }

        Ok(())
    }

    fn read_version(&mut self) -> Result<Version> {
        let minor = self.read_u16()?;
        let major = self.read_u16()?;

        Ok(Version { minor, major })
    }

    fn read_interfaces<'a>(&mut self, constants: &'a ConstantPool) -> Result<Vec<&'a str>> {
        let len = self.read_u16()?;
        let mut indexes = Vec::with_capacity(len as usize);

        for _ in 0..len {
            indexes.push(constants.get_class_info_name(self.read_u16()?))
        }
        Ok(indexes)
    }

    fn read_constants(&mut self) -> Result<ConstantPool> {
        let entries = self.read_u16()?;
        let mut pool = ConstantPool::new(entries);

        let mut i = entries;
        while i > 1 {
            let constant = self.read_constant()?;

            let size = match constant {
                Long(_) | Double(_) => 2,
                _ => 1,
            };
            i -= size;

            pool.add(constant);
        }

        Ok(pool)
    }

    fn read_constant(&mut self) -> Result<Constant> {
        let tag = self.read_u8()?;

        match tag {
            1 => self.read_utf8_constant(),
            3 => self.read_int_constant(),
            4 => self.read_float_constant(),
            5 => self.read_long_constant(),
            6 => self.read_double_constant(),
            7 => self.read_class_constant(),
            8 => self.read_string_constant(),
            9 => self.read_fieldref_constant(),
            10 => self.read_methodref_constant(),
            11 => self.read_interfacemethodref_constant(),
            12 => self.read_nameandtype_constant(),
            15 => self.read_methodhandle_constant(),
            16 => self.read_methodtype_constant(),
            17 => self.read_dynamic_constant(),
            18 => self.read_invokedynamic_constant(),
            19 => self.read_module_constant(),
            20 => self.read_package_constant(),
            _ => panic!("Unknown constant tag {}", tag),
        }
    }

    fn read_access_flags(&mut self) -> Result<ClassAccessFlags> {
        let flags = self.read_u16()?;
        Ok(ClassAccessFlags::from_bits(flags).unwrap())
    }

    // TODO implement according to spec
    // Naive implementation that do not correspond to spec about modified utf8
    fn read_utf8_constant(&mut self) -> Result<Constant> {
        let len = self.read_u16()?;

        let mut bytes = Vec::with_capacity(len as usize);
        unsafe {
            bytes.set_len(len as usize);
        }
        self.reader.read_exact(&mut bytes)?;

        Ok(Utf8(String::from_utf8_lossy(&bytes).to_owned().to_string()))
    }

    fn read_int_constant(&mut self) -> Result<Constant> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(Integer(i32::from_be_bytes(bytes)))
    }

    fn read_float_constant(&mut self) -> Result<Constant> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(Float(f32::from_be_bytes(bytes)))
    }

    fn read_long_constant(&mut self) -> Result<Constant> {
        let mut bytes = [0u8; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(Long(i64::from_be_bytes(bytes)))
    }

    fn read_double_constant(&mut self) -> Result<Constant> {
        let mut bytes = [0u8; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(Double(f64::from_be_bytes(bytes)))
    }

    fn read_class_constant(&mut self) -> Result<Constant> {
        let class_index = self.read_u16()?;
        Ok(ClassRef(class_index))
    }

    fn read_string_constant(&mut self) -> Result<Constant> {
        let string_index = self.read_u16()?;
        Ok(StringRef(string_index))
    }

    fn read_fieldref_constant(&mut self) -> Result<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Ok(FieldRef(class_index, name_and_type_index))
    }

    fn read_methodref_constant(&mut self) -> Result<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Ok(MethodRef(class_index, name_and_type_index))
    }

    fn read_interfacemethodref_constant(&mut self) -> Result<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Ok(InterfaceMethodRef(class_index, name_and_type_index))
    }

    fn read_nameandtype_constant(&mut self) -> Result<Constant> {
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        Ok(NameAndType(name_index, descriptor_index))
    }

    fn read_methodhandle_constant(&mut self) -> Result<Constant> {
        let reference_kind = match self.read_u8()? {
            1 => GetField,
            2 => GetStatic,
            3 => PutField,
            4 => PutStatic,
            5 => InvokeVirtual,
            6 => InvokeStatic,
            7 => InvokeSpecial,
            8 => NewInvokeSpecial,
            9 => InvokeInterface,
            x => panic!("Unknown method handle kind: {}", x),
        };
        let reference_index = self.read_u16()?;
        Ok(MethodHandle(reference_kind, reference_index))
    }

    fn read_methodtype_constant(&mut self) -> Result<Constant> {
        let descriptor_index = self.read_u16()?;
        Ok(MethodType(descriptor_index))
    }

    fn read_dynamic_constant(&mut self) -> Result<Constant> {
        let bootstrap_method_attr_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Ok(Dynamic(bootstrap_method_attr_index, name_and_type_index))
    }

    fn read_invokedynamic_constant(&mut self) -> Result<Constant> {
        let bootstrap_method_attr_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Ok(InvokeDynamic(
            bootstrap_method_attr_index,
            name_and_type_index,
        ))
    }

    fn read_module_constant(&mut self) -> Result<Constant> {
        let name_index = self.read_u16()?;
        Ok(Module(name_index))
    }

    fn read_package_constant(&mut self) -> Result<Constant> {
        let name_index = self.read_u16()?;
        Ok(Package(name_index))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut bytes = [0u8; 1];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut bytes = [0u8; 2];
        self.reader.read_exact(&mut bytes)?;
        Ok((bytes[0] as u16) << 8 | bytes[1] as u16)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let u16_0 = self.read_u16()? as u32;
        let u16_1 = self.read_u16()? as u32;
        Ok(u16_0 << 16 | u16_1)
    }
}

#[cfg(test)]
mod test {
    use crate::class::constant::Constant::*;
    use crate::class::constant::ConstantPool;
    use crate::class::constant::MethodHandleKind::GetField;
    use crate::class::io::ClassReader;
    use crate::class::AttributeData::{SourceFile, Unknown};
    use crate::class::Version;
    use crate::class::{
        Attribute, ClassAccessFlags, FieldAccessFlags, FieldInfo, MethodAccessFlags, MethodInfo,
    };

    #[test]
    fn read_signature() {
        let data: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE];
        let mut reader = ClassReader::new(data.as_slice());
        assert_eq!(reader.read_signature(), Ok(()));
    }

    #[test]
    fn read_invalid_signature() {
        let data: Vec<u8> = vec![0xCA, 0xFE, 0xAB, 0xBB];
        let mut reader = ClassReader::new(data.as_slice());
        let error = reader.read_signature().unwrap_err();
        assert_eq!(error.to_string(), "Invalid file signature.");
    }

    #[test]
    fn read_version() {
        let data: Vec<u8> = vec![0x01, 0x00, 0x00, 0x37];
        let mut reader = ClassReader::new(data.as_slice());

        assert_eq!(
            reader.read_version(),
            Ok(Version {
                minor: 256,
                major: 55
            })
        );
    }

    #[test]
    fn read_constant_pool() {
        let data: Vec<u8> = vec![
            0x00, 0x14, // Pool length
            0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, // Utf8
            0x03, 0x00, 0x00, 0x00, 0x7B, // Integer
            0x04, 0x42, 0xf6, 0xe6, 0x66, // Float
            0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0xd6, 0x87, // Long
            0x06, 0x40, 0x5e, 0xdd, 0x3a, 0x92, 0xa3, 0x05, 0x53, // Double
            0x07, 0x00, 0x0d, // Class
            0x08, 0x01, 0x00, // String
            0x09, 0x00, 0x01, 0x00, 0x02, // FieldRef
            0x0a, 0x00, 0x03, 0x00, 0x0c, // MethodRef
            0x0b, 0x00, 0x03, 0x00, 0x0c, // InterfaceMethodRef
            0x0c, 0x00, 0x04, 0x00, 0x05, // NameAndType
            0x0f, 0x01, 0x00, 0x05, // MethodHandle
            0x10, 0x00, 0x01, // MethodType
            0x11, 0x00, 0x01, 0x00, 0x05, // Dynamic
            0x12, 0x00, 0x01, 0x00, 0x05, // InvokeDynamic
            0x13, 0x00, 0x01, // Module
            0x14, 0x00, 0x02, // Package
        ];

        let mut reader = ClassReader::new(data.as_slice());
        let pool = reader.read_constant_pool().unwrap();

        assert_eq!(pool.get(1), &Utf8("<init>".to_owned()));
        assert_eq!(pool.get(2), &Integer(123));
        assert_eq!(pool.get(3), &Float(123.45));
        assert_eq!(pool.get(4), &Long(1234567));
        assert_eq!(pool.get(6), &Double(123.4567));
        assert_eq!(pool.get(8), &ClassRef(13));
        assert_eq!(pool.get(9), &StringRef(256));
        assert_eq!(pool.get(10), &FieldRef(1, 2));
        assert_eq!(pool.get(11), &MethodRef(3, 12));
        assert_eq!(pool.get(12), &InterfaceMethodRef(3, 12));
        assert_eq!(pool.get(13), &NameAndType(4, 5));
        assert_eq!(pool.get(14), &MethodHandle(GetField, 5));
        assert_eq!(pool.get(15), &MethodType(1));
        assert_eq!(pool.get(16), &Dynamic(1, 5));
        assert_eq!(pool.get(17), &InvokeDynamic(1, 5));
        assert_eq!(pool.get(18), &Module(1));
        assert_eq!(pool.get(19), &Package(2));
    }

    #[test]
    fn read_access_flags() {
        let data: Vec<u8> = vec![0x00, 0x21];
        let mut reader = ClassReader::new(data.as_slice());

        let flags = reader.read_access_flags().unwrap();
        assert_eq!(flags, ClassAccessFlags::SUPER | ClassAccessFlags::PUBLIC);
    }

    #[test]
    fn read_interfaces() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("interface".to_owned()));
        constants.add(ClassRef(1));

        let data: Vec<u8> = vec![
            0x00, 0x01, // Count
            0x00, 0x02, // ClassRef index
        ];
        let mut reader = ClassReader::new(data.as_slice());

        let indexes = reader.read_interfaces(&constants).unwrap();
        assert_eq!(indexes, vec!["interface"]);
    }

    #[test]
    fn read_attributes() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("SourceFile".to_owned()));
        constants.add(Utf8("Unknown attribute".to_owned()));
        constants.add(Utf8("file.java".to_owned()));

        let data: Vec<u8> = vec![
            0x00, 0x02, // Count 2
            0x00, 0x01, // Attribute1 name index
            0x00, 0x00, 0x00, 0x02, 0x00, 0x03, // Attribute1 info index
            0x00, 0x02, // Attribute2 name index
            0x00, 0x00, 0x00, 0x02, 0x01, 0x02, // Attribute2 info index
        ];
        let mut reader = ClassReader::new(data.as_slice());
        let attributes = reader.read_attributes(&constants).unwrap();

        assert_eq!(
            attributes,
            vec![
                Attribute {
                    name: "SourceFile",
                    data: SourceFile("file.java"),
                },
                Attribute {
                    name: "Unknown attribute",
                    data: Unknown(vec![0x01, 0x02])
                }
            ]
        );
    }

    #[test]
    fn read_fields() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("field_name".to_owned()));
        constants.add(Utf8("description".to_owned()));
        constants.add(Utf8("attribute".to_owned()));

        let data: Vec<u8> = vec![
            0x00, 0x01, // Count 1
            0x00, 0x02, // AccessFlags()
            0x00, 0x01, // Name index
            0x00, 0x02, // Descriptor index
            0x00, 0x01, // Attributes count
            0x00, 0x03, // Attribute name index
            0x00, 0x00, 0x00, 0x02, 0x01, 0x02, // Attribute data
        ];
        let mut reader = ClassReader::new(data.as_slice());

        let indexes = reader.read_fields(&constants).unwrap();
        assert_eq!(
            indexes,
            vec![FieldInfo {
                access_flags: FieldAccessFlags::ACC_PRIVATE,
                name: "field_name",
                descriptor: "description",
                attributes: vec![Attribute {
                    name: "attribute",
                    data: Unknown(vec![0x01, 0x02])
                }]
            }]
        );
    }

    #[test]
    fn read_methods() {
        let mut constants = ConstantPool::new(2);
        constants.add(Utf8("method_name".to_owned()));
        constants.add(Utf8("descriptor".to_owned()));
        constants.add(Utf8("attribute".to_owned()));

        let data: Vec<u8> = vec![
            0x00, 0x01, // Count 1
            0x00, 0x02, // AccessFlags()
            0x00, 0x01, // Name index
            0x00, 0x02, // Descriptor index
            0x00, 0x01, // Attributes count
            0x00, 0x03, // Attribute name index
            0x00, 0x00, 0x00, 0x02, 0x01, 0x02, // Attribute data 0x01, 0x02
        ];
        let mut reader = ClassReader::new(data.as_slice());

        let indexes = reader.read_methods(&constants).unwrap();
        assert_eq!(
            indexes,
            vec![MethodInfo {
                access_flags: MethodAccessFlags::ACC_PRIVATE,
                name: "method_name",
                descriptor: "descriptor",
                attributes: vec![Attribute {
                    name: "attribute",
                    data: Unknown(vec![0x01, 0x02])
                }]
            }]
        );
    }
}
