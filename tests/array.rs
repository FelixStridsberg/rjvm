extern crate rjvm;

use rjvm::vm::VirtualMachine;
use rjvm::error::Result;
use rjvm::vm::data_type::Value::Int;

#[test]
fn create_array() -> Result<()> {
    let mut vm = VirtualMachine::default();
    vm.register_class("./tests/test_data/Array.class").unwrap();

    let return_value = vm.run("test_data/Array", "create_array", vec![]);
    assert_eq!(return_value, Int(3));

    Ok(())
}

#[test]
fn create_object() -> Result<()> {
    let mut vm = VirtualMachine::default();
    vm.register_class("./tests/jre/java/lang/Object.class").unwrap();
    vm.register_class("./tests/test_data/Array.class").unwrap();

    let return_value = vm.run("test_data/Array", "main", vec![]);
    assert_eq!(return_value, Int(3));

    Ok(())
}
