extern crate rjvm;

use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value;
use rjvm::vm::data_type::Value::{Boolean, Double, Float, Int, Long};
use rjvm::vm::VirtualMachine;

fn run_method(method_name: &str) -> Value {
    let mut class_register = ClassLoader::new();
    class_register.set_paths(vec!["./tests/", "./tests/jre/"]);
    let mut vm = VirtualMachine::default();

    vm.run(class_register, "test_data/Fields", method_name, vec![])
}

/*#[test]
fn test_static_boolean_field() {
    assert_eq!(run_method("s_boolean"), Boolean(false));
}*/// TODO

#[test]
fn test_static_int_field() {
    assert_eq!(run_method("s_int"), Int(100));
}

#[test]
fn test_static_long_field() {
    assert_eq!(run_method("s_long"), Long(200));
}

#[test]
fn test_static_float_field() {
    assert_eq!(run_method("s_float"), Float(300.0));
}

#[test]
fn test_static_double_field() {
    assert_eq!(run_method("s_double"), Double(400.0));
}

#[test]
fn test_static_other() {
    assert_eq!(run_method("other"), Int(10));
}

/*
#[test]
fn test_boolean_field() {
    assert_eq!(run_method("t_boolean"), Boolean(true));
}*/// TODO

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
