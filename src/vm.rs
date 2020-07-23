use crate::class::Class;
use crate::error::{Error, Result};
use crate::io::class::ClassReader;
use crate::vm::data_type::Value;
use crate::vm::data_type::Value::Reference;
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;
use crate::vm::interpreter::interpret_frame;
use crate::vm::stack::Stack;
use crate::vm::Command::{VMInvokeSpecial, VMInvokeStatic, VMReturn};
use std::collections::HashMap;

#[macro_export]
macro_rules! expect_type (
    ($value:expr, $expected_type:path) => {
        match $value {
            $expected_type(i) => i,
            value => panic!("Tried to use a {:?} as a {}", value, stringify!($expected_type)),
        }
    }
);

pub mod data_type;
mod frame;
mod heap;
mod interpreter;
mod stack;

enum Command {
    VMReturn(Value),
    VMInvokeStatic(u16),
    VMInvokeSpecial(u16),
}

#[derive(Debug)]
pub struct Object {
    class: String,
    //  TODO fields etc
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

    pub fn run(&mut self, class_name: &str, method_name: &str, args: Vec<Value>) -> Value {
        let mut heap = Heap::default();
        let mut stack = Stack::new();

        let result = self.execute(&mut heap, &mut stack, class_name, method_name, args);

        if let Ok(value) = result {
            value
        } else {
            println!("Stack:\n{}", stack);
            println!("Heap: {:#?}", heap);

            panic!(
                "Runtime error {:?}",
                result.unwrap_err().message().expect("No error message.")
            );
        }
    }

    pub fn execute<'a>(
        &'a mut self,
        heap: &mut Heap,
        stack: &mut Stack<'a>,
        init_class_name: &str,
        init_method_name: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        let init_frame = self.prepare_static_method(init_class_name, init_method_name, args);
        stack.push(init_frame);

        loop {
            match interpret_frame(stack.current_frame(), heap)? {
                VMReturn(value) => {
                    if stack.last_frame() {
                        return Ok(value);
                    } else {
                        let void_return = stack
                            .current_frame()
                            .method
                            .descriptor
                            .return_type
                            .is_none();

                        stack.pop();

                        if !void_return {
                            stack.current_frame().push_operand(value);
                        }
                    }
                }
                VMInvokeStatic(index) => {
                    let next_frame = self.invoke_static(index, stack.current_frame())?;
                    stack.push(next_frame);
                }
                VMInvokeSpecial(index) => {
                    let next_frame = self.invoke_special(index, stack.current_frame())?;
                    stack.push(next_frame);
                }
            }
        }
    }

    fn invoke_special(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor) =
            current_frame.class.constants.get_method_ref(index)?;
        let class = self.resolve_class(class_name)?;
        let method = class.resolve_method(method_name, descriptor)?;
        let mut args = current_frame.pop_field_types(&method.descriptor.argument_types);
        let object_ref = current_frame.pop_operand().expect_reference();
        args.insert(0, Reference(object_ref));

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);
        Ok(frame)
    }

    fn invoke_static(&self, index: u16, current_frame: &mut Frame) -> Result<Frame> {
        let (class_name, method_name, descriptor) =
            current_frame.class.constants.get_method_ref(index)?;
        let class = self.resolve_class(class_name)?;
        let method = class.resolve_method(method_name, descriptor)?;
        let args = current_frame.pop_field_types(&method.descriptor.argument_types);

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);
        Ok(frame)
    }

    fn prepare_static_method(
        &self,
        class_name: &str,
        method_name: &str,
        args: Vec<Value>,
    ) -> Frame {
        let class = self.class_register.get(class_name).expect("Unknown class");
        let method = class.find_public_static_method(method_name).unwrap();

        let mut frame = Frame::new(&class, &method);
        frame.load_arguments(args);

        frame
    }

    fn resolve_class(&self, class_name: &str) -> Result<&Class> {
        self.class_register
            .get(class_name)
            .ok_or_else(|| Error::runtime(format!("Could not find class {}", class_name)))
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine {
            class_register: HashMap::new(),
        }
    }
}
