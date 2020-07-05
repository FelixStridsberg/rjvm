extern crate rjvm;

use rjvm::io::class::ClassReader;
use rjvm::vm::VirtualMachine;
use rjvm::vm::Value::{Int, Long};
use rjvm::error::Result;

macro_rules! read_class (
    ($class:ident, $filename:expr) => {
        let reader = ClassReader::open($filename)?;
        let $class = reader.read_class()?;
    }
);

#[test]
fn invoke_static_simple_no_args() -> Result<()> {
    read_class!(class, "./tests/test_data/Simple.class");

    // public static void no_args()
    let method = class.find_public_static_method("no_args").unwrap();
    let mut vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&class, method, vec![]);
    assert_eq!(return_value, Some(Int(1)));
    Ok(())
}

#[test]
fn invoke_static_simple_add() -> Result<()> {
    read_class!(class, "./tests/test_data/Simple.class");

    // public static int add(int a, int b)
    let method = class.find_public_static_method("add").unwrap();
    let mut vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&class, method, vec![Int(1), Int(5)]);
    assert_eq!(return_value, Some(Int(6)));
    Ok(())
}

#[test]
fn invoke_static_simple_add_long() -> Result<()> {
    read_class!(class, "./tests/test_data/Simple.class");

    // public static int add(int a, int b)
    let method = class.find_public_static_method("add_long").unwrap();
    let mut vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&class, method, vec![Long(1), Long(5)]);
    assert_eq!(return_value, Some(Long(6)));
    Ok(())
}

#[test]
fn invoke_static_nested() -> Result<()> {
    read_class!(class, "./tests/test_data/Simple.class");

    // public static int add(int a, int b)
    let method = class.find_public_static_method("add_nested").unwrap();
    let mut vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&class, method, vec![Int(1), Int(5)]);
    assert_eq!(return_value, Some(Int(6)));
    Ok(())
}