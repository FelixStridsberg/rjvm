pub mod constant;
pub mod io;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    minor: u16,
    major: u16,
}

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
    version: Version,
    access_flags: ClassAccessFlags,
    this_class: &'a str,
    super_class: &'a str,
    interfaces: Vec<&'a str>,
    fields: Vec<FieldInfo<'a>>,
    methods: Vec<MethodInfo<'a>>,
    attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo<'a> {
    access_flags: MethodAccessFlags,
    name: &'a str,
    descriptor: &'a str,
    attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FieldInfo<'a> {
    access_flags: FieldAccessFlags,
    name: &'a str,
    descriptor: &'a str,
    attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    name: &'a str,
    data: AttributeData<'a>,
}

#[derive(Debug, PartialEq)]
pub enum AttributeData<'a> {
    SourceFile(&'a str),
    Unknown(Vec<u8>),
}
