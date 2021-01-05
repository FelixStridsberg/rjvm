use crate::class::attribute::AttributeData::CodeInfo;
use crate::class::attribute::{Attribute, Code};
use crate::class::constant::ConstantPool;
use crate::vm::data_type::MethodDescriptor;
use std::convert::TryInto;
use std::rc::Rc;

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

#[derive(Debug, Clone)]
pub struct Class {
    pub version: Version,
    pub constants: ConstantPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: String,
    pub super_class: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<Rc<MethodInfo>>,
    pub attributes: Vec<Attribute>,
}

impl Class {
    // Only used by tests, probably put somewhere else?
    #[cfg(test)]
    pub(crate) fn from_constant_pool(constants: ConstantPool) -> Class {
        Class {
            version: Version { minor: 0, major: 0 },
            constants,
            access_flags: ClassAccessFlags::PUBLIC,
            this_class: "<Anonymous>".to_string(),
            super_class: "<Anonymous>".to_string(),
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    pub fn resolve_method(&self, name: &str, descriptor: &str) -> Option<Rc<MethodInfo>> {
        self.methods
            .iter()
            .filter(|m| m.name == name)
            .find(|m| m.descriptor == descriptor.try_into().unwrap())
            .cloned()
    }

    pub fn find_public_static_method(&self, name: &str) -> Option<Rc<MethodInfo>> {
        self.methods
            .iter()
            .find(|m| {
                m.name.ends_with(name)
                    && m.access_flags
                        .contains(MethodAccessFlags::ACC_PUBLIC | MethodAccessFlags::ACC_STATIC)
            })
            .cloned()
    }

    pub fn find_static_method(&self, name: &str) -> Option<Rc<MethodInfo>> {
        self.methods
            .iter()
            .find(|m| {
                m.name.ends_with(name) && m.access_flags.contains(MethodAccessFlags::ACC_STATIC)
            })
            .cloned()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub minor: u16,
    pub major: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name: String,
    pub descriptor: MethodDescriptor,
    pub attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn from_code(code: Code) -> MethodInfo {
        MethodInfo {
            access_flags: MethodAccessFlags::ACC_PUBLIC,
            name: "<Internal>".to_owned(),
            descriptor: "()V".try_into().unwrap(),
            attributes: vec![Attribute {
                name: "Code".to_string(),
                data: CodeInfo(code),
            }],
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        for a in &self.attributes {
            if a.name == name {
                return Some(a);
            }
        }
        None
    }

    pub fn get_code(&self) -> Option<Rc<Code>> {
        if let Some(attribute) = self.get_attribute("Code") {
            match &attribute.data {
                CodeInfo(c) => Some(Rc::new(c.clone())),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
}

#[cfg(test)]
mod test {
    use crate::class::constant::ConstantPool;
    use crate::class::{Class, MethodAccessFlags, MethodInfo};
    use std::convert::TryInto;
    use std::rc::Rc;

    #[test]
    fn resolve_non_existing_method() {
        let class = class_with_methods(vec![("not_me", "()V")]);
        let method = class.resolve_method("method", "()V");
        assert!(matches!(method, None));
    }

    #[test]
    fn resolve_method_single_match() {
        let class = class_with_methods(vec![("the_method", "()V")]);
        let method = class.resolve_method("the_method", "()V").unwrap();
        assert_eq!(method.descriptor, "()V".try_into().unwrap());
    }

    #[test]
    fn resolve_method_multiple_matches() {
        let class = class_with_methods(vec![("the_method", "()V"), ("the_method", "(I)V")]);
        let method = class.resolve_method("the_method", "(I)V").unwrap();
        assert_eq!(method.descriptor, "(I)V".try_into().unwrap());
    }

    fn class_with_methods(methods: Vec<(&str, &str)>) -> Class {
        let constants = ConstantPool::new(10);
        let mut class = Class::from_constant_pool(constants);

        for (method_name, method_descriptor) in methods {
            class.methods.push(Rc::new(MethodInfo {
                access_flags: MethodAccessFlags::ACC_PUBLIC,
                name: method_name.to_string(),
                descriptor: method_descriptor.try_into().unwrap(),
                attributes: vec![],
            }));
        }

        class
    }
}
