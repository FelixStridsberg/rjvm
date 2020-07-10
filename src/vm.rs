use crate::class::constant::Constant::{ClassRef, MethodRef, NameAndType};
use crate::class::Class;
use crate::error::{Error, Result};
use crate::io::class::ClassReader;
use crate::vm::data::{FieldType, Value};
use crate::vm::frame::Frame;
use crate::vm::interpreter::interpret_frame;
use crate::vm::Command::{VMInvokeStatic, VMReturn};
use bitflags::_core::convert::TryFrom;
use std::collections::HashMap;
use std::convert::TryInto;

pub mod data;
mod frame;
mod interpreter;

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
}

pub struct VirtualMachine {
    class_register: HashMap<String, Class>,
}

impl VirtualMachine {
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

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MethodDescriptor<'a> {
    argument_types: Vec<FieldType<'a>>,
    return_type: Option<FieldType<'a>>,
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
    use crate::error::Result;
    use crate::vm::data::FieldType::*;
    use crate::vm::MethodDescriptor;
    use std::convert::TryInto;
    use std::str::FromStr;

    #[test]
    fn parse_method_descriptor_return_int() {
        let descriptor: MethodDescriptor = "()I".try_into().unwrap();
        assert_eq!(descriptor, MethodDescriptor { argument_types: vec![], return_type: Some(Int) });
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
