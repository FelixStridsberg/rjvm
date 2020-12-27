extern crate rjvm;

use rjvm::error::Result;
use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value;
use rjvm::vm::VirtualMachine;
use rjvm::vm::data_type::Value::{Int, Long, Float, Double};

fn run_method(method_name: &str) -> Value {
    let mut class_register = ClassLoader::new();
    class_register
        .set_paths(vec!["./tests/", "./tests/jre/"]);
    let mut vm = VirtualMachine::default();

    vm.run(
        class_register,
        "test_data/Fields",
        method_name,
        vec![],
    )
}

#[test]
fn test_int_field() {
    assert_eq!(run_method("t_int"), Int(0));
}

#[test]
fn test_long_field() {
    assert_eq!(run_method("t_long"), Long(100));
}

#[test]
fn test_float_field() {
    assert_eq!(run_method("t_float"), Float(1.0));
}

#[test]
fn test_double_field() {
    assert_eq!(run_method("t_double"), Double(2.0));
}
