use crate::error::Error;
use crate::vm::data::Value::*;
use std::convert::{TryFrom, TryInto};

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
