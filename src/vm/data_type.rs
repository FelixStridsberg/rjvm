use crate::error::Error;
use crate::vm::data_type::Value::*;
use std::convert::{TryFrom, TryInto};

pub type BooleanType = bool;
pub type CharType = char;
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
    Boolean(BooleanType),
    Byte(ByteType),
    Short(ShortType),
    Int(IntType),
    Long(LongType),
    Char(CharType),
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

    pub fn expect_boolean(self) -> BooleanType {
        expect_type!(self, Boolean)
    }

    pub fn expect_byte(self) -> ByteType {
        expect_type!(self, Byte)
    }

    pub fn expect_short(self) -> ShortType {
        expect_type!(self, Short)
    }

    pub fn expect_int(self) -> IntType {
        expect_type!(self, Int)
    }

    pub fn expect_long(self) -> LongType {
        expect_type!(self, Long)
    }

    pub fn expect_char(self) -> CharType {
        expect_type!(self, Char)
    }

    pub fn expect_float(self) -> FloatType {
        expect_type!(self, Float)
    }

    pub fn expect_double(self) -> DoubleType {
        expect_type!(self, Double)
    }

    pub fn expect_reference(self) -> ReferenceType {
        expect_type!(self, Reference)
    }

    pub fn expect_type(self, field_type: &FieldType) -> Value {
        match field_type {
            FieldType::Byte => Byte(self.expect_byte()),
            FieldType::Char => Char(self.expect_char()),
            FieldType::Double => Double(self.expect_double()),
            FieldType::Float => Float(self.expect_float()),
            FieldType::Int => Int(self.expect_int()),
            FieldType::Long => Long(self.expect_long()),
            FieldType::Short => Short(self.expect_short()),
            FieldType::Boolean => Boolean(self.expect_boolean()),
            FieldType::Object(_) => Reference(self.expect_reference()),
            FieldType::Array(_) => Reference(self.expect_reference()),
        }
    }

    pub fn as_int_value(&self) -> u32 {
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
            _ => panic!("Tried to use {:?} as int value.", self),
        }
    }

    pub fn as_long_value(&self) -> u64 {
        match self {
            Long(l) => *l as u64,
            Double(d) => (*d).to_bits(),
            _ => panic!("Tried to use {:?} as long value.", self),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FieldType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    pub(crate) fn str_len(&self) -> usize {
        match self {
            FieldType::Object(s) => s.len() + 2,
            FieldType::Array(t) => t.str_len() + 1,
            _ => 1,
        }
    }
}

impl<'a> TryFrom<&'a str> for FieldType {
    type Error = Error;

    fn try_from(s: &'a str) -> std::result::Result<FieldType, Self::Error> {
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
                FieldType::Object((&s[1..index]).to_owned())
            }
            '[' => FieldType::Array(Box::new(s[1..].try_into()?)),
            _ => panic!("Invalid field type '{}'", s),
        })
    }
}

/// Method descriptors defines a method signature; the argument and the return types.
///
/// Raw JVM method descriptors are strings that looks like this: `(IJ)F`
///
/// Where the contents of the parentheses are the arguments and the string outside at the end the
/// return type. This particular signature is a method taking an `int` and a `long` as arguments and
/// returning a `float`.
///
/// Primitive types are only one character long, but references (`L<ClassName>;`) and arrays
/// (`[<type>`) are more characters.
///
/// MethodDescriptor's can be parsed with the `TryInto` trait:
/// ```
///# use rjvm::vm::data_type::MethodDescriptor;
///# use std::convert::TryInto;
///# use crate::rjvm::error::Result;
///# fn main() -> Result<()> {
///# use rjvm::vm::data_type::FieldType;
/// let descriptor: MethodDescriptor = "(IJ)F".try_into()?;
///
/// assert_eq!(descriptor.argument_types, vec![FieldType::Int, FieldType::Long]);
/// assert_eq!(descriptor.return_type, Some(FieldType::Float));
///# Ok(())
///# }
/// ```
/// Return value is `None` on void methods.
///
/// Reference: https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.3
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MethodDescriptor {
    pub argument_types: Vec<FieldType>,
    pub return_type: Option<FieldType>,
}

impl MethodDescriptor {
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

impl TryFrom<&str> for MethodDescriptor {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<MethodDescriptor, Self::Error> {
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
                    Object("java/lang/Object".to_owned()),
                    Array(Box::new(Int)),
                    Array(Box::new(Object("java/lang/Object".to_owned())))
                ],
                return_type: None
            }
        );
    }
}
