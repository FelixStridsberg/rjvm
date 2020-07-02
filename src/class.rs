use crate::class::attribute::AttributeData::CodeInfo;
use crate::class::attribute::{Attribute, Code};

pub mod attribute;
pub mod code;
pub mod constant;

bitflags! {
    pub struct ClassAccessFlags: u16 {
        const PUBLIC     = 0x0001;
        const FINAL      = 0x0010;
        const SUPER      = 0x0020;
        const INTERFACE  = 0x0200;
        const ABSTRACT   = 0x0400;
        const SYNTHETIC  = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM       = 0x4000;
        const MODULE     = 0x8000;
    }
}

bitflags! {
    pub struct FieldAccessFlags: u16 {
        const ACC_PUBLIC    = 0x0001;
        const ACC_PRIVATE   = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC    = 0x0008;
        const ACC_FINAL     = 0x0010;
        const ACC_VOLATILE  = 0x0040;
        const ACC_TRANSIENT = 0x0080;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ENUM      = 0x4000;
    }
}

bitflags! {
    pub struct MethodAccessFlags: u16 {
        const ACC_PUBLIC       = 0x0001;
        const ACC_PRIVATE      = 0x0002;
        const ACC_PROTECTED    = 0x0004;
        const ACC_STATIC       = 0x0008;
        const ACC_FINAL        = 0x0010;
        const ACC_SYNCHRONIZED = 0x0020;
        const ACC_BRIDGE       = 0x0040;
        const ACC_VARARGS      = 0x0080;
        const ACC_NATIVE       = 0x0100;
        const ACC_ABSTRACT     = 0x0400;
        const ACC_STRICT       = 0x0800;
        const ACC_SYNTHETIC    = 0x1000;
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    pub version: Version,
    pub access_flags: ClassAccessFlags,
    pub this_class: &'a str,
    pub super_class: &'a str,
    pub interfaces: Vec<&'a str>,
    pub fields: Vec<FieldInfo<'a>>,
    pub methods: Vec<MethodInfo<'a>>,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub minor: u16,
    pub major: u16,
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo<'a> {
    pub access_flags: MethodAccessFlags,
    pub name: &'a str,
    pub descriptor: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

impl MethodInfo<'_> {
    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        for a in &self.attributes {
            if a.name == name {
                return Some(a);
            }
        }
        None
    }

    pub fn get_code(&self) -> Option<&Code> {
        if let Some(attribute) = self.get_attribute("Code") {
            match &attribute.data {
                CodeInfo(c) => Some(c),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldInfo<'a> {
    pub access_flags: FieldAccessFlags,
    pub name: &'a str,
    pub descriptor: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}
