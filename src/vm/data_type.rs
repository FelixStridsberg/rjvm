use crate::error::Error;
use crate::vm::data_type::Value::*;
use std::convert::{TryFrom, TryInto};

pub type ByteType = u8;
pub type ShortType = i16;
pub type IntType = i32;
pub type LongType = i64;
pub type FloatType = f32;
pub type DoubleType = f64;
pub type ReferenceType = u32;
pub type ReturnAddressType = u32;


#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Short(ShortType),
    Int(IntType),
    Long(LongType),
    Char(char),
    Float(FloatType),
    Double(DoubleType),
    Reference(ReferenceType),
    ReturnAddress(ReturnAddressType),
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
            Int(i) => *i as u32,
            Reference(i) | ReturnAddress(i) => *i as u32,
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
    pub(crate) fn str_len(&self) -> usize {
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
pub struct MethodDescriptor<'a> {
    pub argument_types: Vec<FieldType<'a>>,
    pub return_type: Option<FieldType<'a>>,
}

impl MethodDescriptor<'_> {
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
    use crate::vm::data_type::FieldType::*;
    use crate::vm::data_type::MethodDescriptor;
    use std::convert::TryInto;

    #[test]
    fn parse_method_descriptor_return_int() {
        let descriptor: MethodDescriptor = "()I".try_into().unwrap();
        assert_eq!(
            descriptor,
            MethodDescriptor {
                argument_types: vec![],
                return_type: Some(Int)
            }
        );
    }

    #[test]
    fn parse_base_type_method_descriptors() {
        let descriptor: MethodDescriptor = "(BCDFIJSZ)V".try_into().unwrap();
        assert_eq!(
            descriptor,
            MethodDescriptor {
                argument_types: vec![Byte, Char, Double, Float, Int, Long, Short, Boolean],
                return_type: None,
            }
        );
    }

    #[test]
    fn parse_complex_method_descriptors() {
        let descriptor: MethodDescriptor = "(Ljava/lang/Object;[I[Ljava/lang/Object;)V"
            .try_into()
            .unwrap();

        assert_eq!(
            descriptor,
            MethodDescriptor {
                argument_types: vec![
                    Object("java/lang/Object"),
                    Array(Box::new(Int)),
                    Array(Box::new(Object("java/lang/Object")))
                ],
                return_type: None
            }
        );
    }
}
