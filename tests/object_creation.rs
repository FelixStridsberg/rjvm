extern crate rjvm;

use rjvm::error::Result;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::{ClassRegister, VirtualMachine};

#[test]
fn create_array() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/test_data/Array.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_register,
        "test_data/Array",
        "create_int_array",
        vec![],
    );
    assert_eq!(return_value, Int(3));

    Ok(())
}

#[test]
fn create_object() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/jre/java/lang/Object.class")
        .unwrap();
    class_register
        .register_class("./tests/test_data/Instance.class")
        .unwrap();

    let mut vm = VirtualMachine::default();
    let return_value = vm.run(class_register, "test_data/Instance", "main", vec![]);
    assert_eq!(return_value, Int(3));

    Ok(())
}
