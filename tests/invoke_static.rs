extern crate rjvm;

use rjvm::io::class::ClassReader;
use rjvm::vm::VirtualMachine;
use rjvm::vm::Value::{Int, Long};
use rjvm::error::Result;

#[test]
fn invoke_static_simple_no_args() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.register_class("./tests/test_data/Simple.class");

    let return_value = vm.run("test_data/Simple", "no_args", vec![]);
    assert_eq!(return_value, Int(1));

    Ok(())
}

#[test]
fn invoke_static_simple_add() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.register_class("./tests/test_data/Simple.class");

    let return_value = vm.run("test_data/Simple", "add", vec![Int(1), Int(5)]);
    assert_eq!(return_value, Int(6));

    Ok(())
}

#[test]
fn invoke_static_simple_add_long() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.register_class("./tests/test_data/Simple.class");

    let return_value = vm.run("test_data/Simple", "add_long", vec![Long(1), Long(5)]);
    assert_eq!(return_value, Long(6));

    Ok(())
}

/*
#[test]
fn invoke_static_nested() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.register_class("./tests/test_data/Simple.class");
    vm.run("test_data/Simple", "add_nested", vec![Int(1), Int(5)]);

//    assert_eq!(return_value, Some(Int(6)));

    Ok(())
}
*/
