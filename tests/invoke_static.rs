extern crate rjvm;

use rjvm::error::Result;
use rjvm::vm::data_type::Value::{Int, Long};
use rjvm::vm::{ClassRegister, VirtualMachine};

#[test]
fn invoke_static_simple_no_args() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/test_data/Simple.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(class_register, "test_data/Simple", "no_args", vec![]);
    assert_eq!(return_value, Int(1));

    Ok(())
}

#[test]
fn invoke_static_simple_add() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/test_data/Simple.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_register,
        "test_data/Simple",
        "add",
        vec![Int(1), Int(5)],
    );
    assert_eq!(return_value, Int(6));

    Ok(())
}

#[test]
fn invoke_static_simple_add_long() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/test_data/Simple.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_register,
        "test_data/Simple",
        "add_long",
        vec![Long(1), Long(5)],
    );
    assert_eq!(return_value, Long(6));

    Ok(())
}

#[test]
fn invoke_static_nested() -> Result<()> {
    let mut class_register = ClassRegister::new();
    class_register
        .register_class("./tests/test_data/Simple.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_register,
        "test_data/Simple",
        "add_nested",
        vec![Int(1), Int(5)],
    );

    assert_eq!(return_value, Int(6));

    Ok(())
}
