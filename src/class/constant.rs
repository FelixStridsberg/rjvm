use crate::class::constant::Constant::{
    ClassRef, Double, FieldRef, Long, MethodRef, NameAndType, Utf8, NOOP,
};
use crate::error::{Error, ErrorKind, Result};

type Index = u16;

#[derive(Debug, PartialEq, Clone)]
pub enum MethodHandleKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassRef(Index),
    StringRef(Index),
    FieldRef(Index, Index),
    MethodRef(Index, Index),
    InterfaceMethodRef(Index, Index),
    NameAndType(Index, Index),
    MethodHandle(MethodHandleKind, Index),
    MethodType(Index),
    Dynamic(Index, Index),
    InvokeDynamic(Index, Index),
    Module(Index),
    Package(Index),
    NOOP,
}

#[derive(Debug)]
pub struct ConstantPool {
    constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn new(size: u16) -> Self {
        ConstantPool {
            constants: Vec::with_capacity(size as usize),
        }
    }

    pub fn add(&mut self, constant: Constant) {
        let double = match constant {
            Long(_) | Double(_) => true,
            _ => false,
        };

        self.constants.push(constant);

        // Long and doubles takes up two spaces. We have to add a noop to keep the indexes intact
        // since we don't store the actual bytes.
        if double {
            self.constants.push(NOOP)
        }
    }

    pub fn get(&self, index: u16) -> &Constant {
        &self.constants[(index - 1) as usize]
    }

    pub fn get_utf8(&self, index: u16) -> Result<&str> {
        let entry = self.get(index);
        if let Utf8(s) = entry {
            Ok(s.as_ref())
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                Some(format!("Tried to get {:?} as a utf8", entry)),
            ))
        }
    }

    pub fn get_class_info_name(&self, index: u16) -> Result<&str> {
        let entry = self.get(index);
        if let ClassRef(name_index) = entry {
            self.get_utf8(*name_index)
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                Some(format!("Tried to get {:?} as a class reference", entry)),
            ))
        }
    }

    pub fn get_name_and_type(&self, index: u16) -> Result<(&str, &str)> {
        let entry = self.get(index);
        if let NameAndType(name_index, descriptor_index) = entry {
            Ok((
                self.get_utf8(*name_index)?,
                self.get_utf8(*descriptor_index)?,
            ))
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                Some(format!("Tried to get {:?} as a class reference", entry)),
            ))
        }
    }

    pub fn get_method_ref(&self, index: u16) -> Result<(&str, &str, &str)> {
        let entry = self.get(index);
        if let MethodRef(class_index, name_type_index) = entry {
            let class_name = self.get_class_info_name(*class_index)?;
            let (method_name, descriptor_string) = self.get_name_and_type(*name_type_index)?;
            Ok((class_name, method_name, descriptor_string))
        } else {
            runtime_error!("Tried to get {:?} as a method reference", entry)
        }
    }

    pub fn get_field_ref(&self, index: u16) -> Result<(&str, &str, &str)> {
        let entry = self.get(index);
        if let FieldRef(class_index, name_type_index) = entry {
            let class_name = self.get_class_info_name(*class_index)?;
            let (method_name, descriptor_string) = self.get_name_and_type(*name_type_index)?;
            Ok((class_name, method_name, descriptor_string))
        } else {
            runtime_error!("Tried to get {:?} as a field reference", entry)
        }
    }
}
