use crate::vm::data_type::Value;
use std::collections::HashMap;
use crate::vm::stack::Stack;

pub struct Native {
    methods: HashMap<String, fn(stack: &mut Stack) -> Option<Value>>,
}

impl Native {
    pub fn new() -> Self {
        let mut native = Native {
            methods: HashMap::new(),
        };

        java_lang_float::auto_register_natives(&mut native);
        java_lang_double::auto_register_natives(&mut native);
        java_lang_throwable::auto_register_natives(&mut native);

        native
    }

    pub fn register_method(
        &mut self,
        class_name: &str,
        method_name: &str,
        method: fn(stack: &mut Stack) -> Option<Value>,
    ) {
        let key = Self::method_key(class_name, method_name);
        self.methods.insert(key, method);
    }

    pub fn invoke(&mut self, stack: &mut Stack) -> Option<Value> {
        let frame = stack.current_frame();
        if frame.method.name == "registerNatives" {
            self.register_natives(&frame.class.this_class);
            return None;
        }

        let key = Self::method_key(&frame.class.this_class, &frame.method.name);
        if !self.methods.contains_key(&key) {
            eprintln!("Called undefined native method: {}", key);
            return None;
        }

        let method = self.methods.get(&key).unwrap();
        method(stack)
    }

    fn register_natives(&mut self, class_name: &str) {
        match class_name {
            "java/lang/Class" => java_lang_class::register_natives(self),
            "java/lang/System" => java_lang_system::register_natives(self),
            "java/lang/Object" => java_lang_object::register_natives(self),
            _ => eprintln!("No natives to register for {}", class_name),
        }
    }

    fn method_key(class_name: &str, method_name: &str) -> String {
        let mut key = class_name.to_owned();
        key.push(':');
        key.push_str(method_name);
        key
    }
}

mod java_lang_object {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::Int;
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn register_natives(native: &mut Native) {
        native.register_method("java/lang/Object", "hashCode", init_properties);
    }

    fn init_properties(stack: &mut Stack) -> Option<Value> {
        println!("MOCK, hashCode arg: {:?}", stack.current_frame().get_local(0));
        Some(Int(0))
    }
}

mod java_lang_throwable {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::Null;
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn auto_register_natives(native: &mut Native) {
        native.register_method(
            "java/lang/Throwable",
            "fillInStackTrace",
            fill_in_stack_trace,
        );
    }

    fn fill_in_stack_trace(stack: &mut Stack) -> Option<Value> {
        println!("MOCK, fillInStackTrace arg: {:?}", stack.current_frame().get_local(0));
        Some(Null)
    }
}

mod java_lang_system {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::Null;
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn register_natives(native: &mut Native) {
        native.register_method("java/lang/System", "initProperties", init_properties);
    }

    fn init_properties(stack: &mut Stack) -> Option<Value> {
        println!("MOCK, initProperties arg: {:?}", stack.current_frame().get_local(0));
        Some(Null)
    }
}

mod java_lang_class {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::Null;
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn register_natives(native: &mut Native) {
        native.register_method(
            "java/lang/Class",
            "desiredAssertionStatus0",
            desired_assertion_status0,
        );

        native.register_method("java/lang/Class", "getPrimitiveClass", get_primitive_class);
    }

    fn desired_assertion_status0(_stack: &mut Stack) -> Option<Value> {
        println!("MOCK desiredAssertionStatus0, return Int(0)");
        Some(Value::Int(0))
    }

    fn get_primitive_class(stack: &mut Stack) -> Option<Value> {
        println!("MOCK getPrimitiveClass, arg: {:?}", stack.current_frame().get_local(0));
        Some(Null)
    }
}

mod java_lang_float {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::Int;
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn auto_register_natives(native: &mut Native) {
        native.register_method(
            "java/lang/Float",
            "floatToRawIntBits",
            float_to_raw_int_bits,
        );
    }

    fn float_to_raw_int_bits(stack: &mut Stack) -> Option<Value> {
        println!("MOCK floatToRawIntBits, arg: {:?}", stack.current_frame().get_local(0));
        Some(Int(0))
    }
}

mod java_lang_double {
    use crate::vm::data_type::Value;
    use crate::vm::data_type::Value::{Double, Int};
    use crate::vm::native::Native;
    use crate::vm::stack::Stack;

    pub fn auto_register_natives(native: &mut Native) {
        native.register_method(
            "java/lang/Double",
            "doubleToRawLongBits",
            double_to_raw_int_bits,
        );

        native.register_method("java/lang/Double", "longBitsToDouble", long_bits_to_double);
    }

    fn double_to_raw_int_bits(stack: &mut Stack) -> Option<Value> {
        println!("MOCK doubleToRawIntBits, arg: {:?}", stack.current_frame().get_local(0));
        Some(Int(0))
    }

    fn long_bits_to_double(stack: &mut Stack) -> Option<Value> {
        println!("MOCK longBitsToDouble, arg: {:?}", stack.current_frame().get_local(0));
        Some(Double(0.0))
    }
}
