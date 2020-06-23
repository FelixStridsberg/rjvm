use crate::class::constant::Constant::{ClassRef, Double, Long, Utf8, NOOP};

type Index = u16;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
        println!("Index: {}", index);
        &self.constants[(index - 1) as usize]
    }

    pub fn get_utf8(&self, index: u16) -> &str {
        let entry = self.get(index);
        if let Utf8(s) = entry {
            s.as_ref()
        } else {
            panic!(format!("Tried to get {:?} as an utf8", entry))
        }
    }

    pub fn get_class_info_name(&self, index: u16) -> &str {
        let entry = self.get(index);
        if let ClassRef(name_index) = entry {
            self.get_utf8(*name_index)
        } else {
            panic!(format!("Tried to get {:?} as a class reference", entry))
        }
    }
}
