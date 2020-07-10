use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType};
use crate::class::Class;
use crate::error::{Error, Result};
use crate::io::class::ClassReader;
use crate::vm::frame::Frame;
use crate::vm::interpreter::interpret_frame;
use crate::vm::Command::{VMInvokeStatic, VMReturn};
use crate::vm::Value::*;
use bitflags::_core::convert::TryFrom;
use std::collections::HashMap;
use std::convert::TryInto;

mod frame;
mod interpreter;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(char),
    Float(f32),
    Double(f64),
    Reference(i32),
    ReturnAddress(i32),
    Null,
}

impl Value {
    pub fn get_category(&self) -> u8 {
        match self {
            Long(_) | Double(_) => 2,
            _ => 1,
        }
    }

    pub fn get_int_value(&self) -> u32 {
        match self {
            Boolean(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Byte(b) => *b as u32,
            Short(s) => *s as u32,
            Char(c) => *c as u32,
            Float(f) => (*f).to_bits(),
            Null => 0,
            Int(i) | Reference(i) | ReturnAddress(i) => *i as u32,
            _ => panic!("Tried to get int value of {:?}", self),
        }
    }

    pub fn get_long_value(&self) -> u64 {
        match self {
            Long(l) => *l as u64,
            Double(d) => (*d).to_bits(),
            _ => panic!("Tried to get long value of {:?}", self),
        }
    }
}

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
}

pub struct VirtualMachine {
    class_register: HashMap<String, Class>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }

    pub fn register_class(&mut self, filename: &str) -> Result<()> {
        let class = ClassReader::from_file(filename)?;
        self.class_register.insert(class.this_class.clone(), class);

        Ok(())
    }

    pub fn run(
        &mut self,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
    ) -> Value {
        let initial_frame = self.prepare_static_method(init_class_name, init_method_name, args);

        let mut stack = Vec::new();
        let mut current_frame = initial_frame;

        loop {
            match interpret_frame(&mut current_frame) {
                VMReturn(value) => {
                    if stack.is_empty() {
                        return value;
                    } else {
                        current_frame = stack.pop().unwrap();
                        current_frame.push_operand(value);
                    }
                }
                VMInvokeStatic(index) => {
                    let (descriptor, class_name, method_name) =
                        Self::get_static_method(&current_frame, index);
                    let args = current_frame.pop_field_types(&descriptor.argument_types);
                    let next_frame = self.prepare_static_method(class_name, method_name, args);

                    stack.push(current_frame);
                    current_frame = next_frame;
                }
            }
        }
    }

    fn prepare_static_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();
        let code = method.get_code().expect("No Code attribute on method.");

        let mut frame = Frame::new(&code, &class.constants);
        frame.load_arguments(args);

        frame
    }

    fn get_static_method<'a>(
        frame: &Frame<'a>,
        index: u16,
    ) -> (MethodDescriptor<'a>, &'a str, &'a str) {
        let method_ref = frame.constant_pool.get(index);

        let mut class_name = "";
        let mut method_name = "";
        let mut descriptor_str = "";
        match method_ref {
            MethodRef(class_index, name_type_index) => {
                if let ClassRef(class) = frame.constant_pool.get(*class_index) {
                    class_name = frame.constant_pool.get_utf8(*class);
                }
                if let NameAndType(name_idx, type_idx) = frame.constant_pool.get(*name_type_index) {
                    method_name = frame.constant_pool.get_utf8(*name_idx);
                    descriptor_str = frame.constant_pool.get_utf8(*type_idx);
                    // println!("type: {:?}", frame.constant_pool.get(*type_idx));
                }
            }
            _ => panic!(""),
        }

        let descriptor = descriptor_str.try_into().unwrap();
        (descriptor, class_name, method_name)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FieldType<'a> {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Object(&'a str),
    Array(Box<FieldType<'a>>),
}

impl FieldType<'_> {
    fn str_len(&self) -> usize {
        match self {
            FieldType::Object(s) => s.len() + 2,
            FieldType::Array(t) => t.str_len() + 1,
            _ => 1,
        }
    }
}
impl<'a> TryFrom<&'a str> for FieldType<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> std::result::Result<FieldType<'a>, Self::Error> {
        Ok(match s.chars().next().unwrap() {
            'B' => FieldType::Byte,
            'C' => FieldType::Char,
            'D' => FieldType::Double,
            'F' => FieldType::Float,
            'I' => FieldType::Int,
            'J' => FieldType::Long,
            'S' => FieldType::Short,
            'Z' => FieldType::Boolean,
            'L' => {
                let index = s.find(';').unwrap();
                FieldType::Object(&s[1..index])
            }
            '[' => FieldType::Array(Box::new(s[1..].try_into()?)),
            _ => panic!("Invalid field type '{}'", s),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MethodDescriptor<'a> {
    argument_types: Vec<FieldType<'a>>,
    return_type: Option<FieldType<'a>>,
}

impl MethodDescriptor<'_> {
    pub fn new<'a>(
        arguments: Vec<FieldType<'a>>,
        return_type: Option<FieldType<'a>>,
    ) -> MethodDescriptor<'a> {
        MethodDescriptor {
            argument_types: arguments,
            return_type,
        }
    }

    fn parse_argument_str(s: &str) -> std::result::Result<Vec<FieldType>, Error> {
        let mut argument_types = Vec::new();

        let mut i = 0;
        while i < s.len() {
            let field_type: FieldType = (&s[i..]).try_into()?;
            i += field_type.str_len();

            argument_types.push(field_type);
        }
        Ok(argument_types)
    }

    fn parse_return_type(s: &str) -> std::result::Result<Option<FieldType>, Error> {
        if s == "V" {
            Ok(None)
        } else {
            Ok(Some(s.try_into()?))
        }
    }
}

impl<'a> TryFrom<&'a str> for MethodDescriptor<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> std::result::Result<MethodDescriptor<'a>, Self::Error> {
        let parts: Vec<&str> = s.split(|c| c == '(' || c == ')').collect();
        if parts.len() != 3 || !parts[0].is_empty() || parts[2].len() != 1 {
            panic!("Invalid method descriptor '{}'.", s);
        }

        Ok(MethodDescriptor {
            argument_types: Self::parse_argument_str(parts[1])?,
            return_type: Self::parse_return_type(parts[2])?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::vm::FieldType::*;
    use crate::vm::MethodDescriptor;
    use std::str::FromStr;

    use crate::error::Result;
    use std::convert::TryInto;

    #[test]
    fn parse_method_descriptor_return_int() {
        let descriptor: MethodDescriptor = "()I".try_into().unwrap();
        assert_eq!(descriptor, MethodDescriptor::new(vec![], Some(Int)));
    }

    #[test]
    fn parse_base_type_method_descriptors() {
        let descriptor: MethodDescriptor = "(BCDFIJSZ)V".try_into().unwrap();
        assert_eq!(
            descriptor,
            MethodDescriptor::new(
                vec![Byte, Char, Double, Float, Int, Long, Short, Boolean],
                None
            )
        );
    }

    #[test]
    fn parse_complex_method_descriptors() {
        let descriptor: MethodDescriptor = "(Ljava/lang/Object;[I[Ljava/lang/Object;)V"
            .try_into()
            .unwrap();

        assert_eq!(
            descriptor,
            MethodDescriptor::new(
                vec![
                    Object("java/lang/Object"),
                    Array(Box::new(Int)),
                    Array(Box::new(Object("java/lang/Object")))
                ],
                None
            )
        );
    }
}
