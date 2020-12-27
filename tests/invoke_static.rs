extern crate rjvm;

use rjvm::vm::data_type::Value::{Int, Long};

#[path="./java_utils.rs"]
mod java;

#[test]
fn invoke_static_simple_no_args() {
    assert_eq!(java::run_method("test_data/Simple", "no_args"), Int(1));
}

#[test]
fn invoke_static_simple_add() {
    assert_eq!(java::run_method_args("test_data/Simple", "add", vec![Int(1), Int(5)]), Int(6));
}

#[test]
fn invoke_static_simple_add_long() {
    assert_eq!(java::run_method_args("test_data/Simple", "add_long", vec![Long(1), Long(5)]), Long(6));
}

#[test]
fn invoke_static_nested() {
    assert_eq!(java::run_method_args("test_data/Simple", "add_nested", vec![Int(1), Int(5)]), Int(6));
}
