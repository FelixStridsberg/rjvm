extern crate rjvm;

use rjvm::vm::data_type::Value::{Double, Float, Int, Long};

#[path = "utils.rs"]
mod java;

#[test]
fn test_static_boolean_field() {
    assert_eq!(java::run_method("test_data/Fields", "s_boolean"), Int(0));
}

#[test]
fn test_static_int_field() {
    assert_eq!(java::run_method("test_data/Fields", "s_int"), Int(100));
}

#[test]
fn test_static_long_field() {
    assert_eq!(java::run_method("test_data/Fields", "s_long"), Long(200));
}

#[test]
fn test_static_float_field() {
    assert_eq!(
        java::run_method("test_data/Fields", "s_float"),
        Float(300.0)
    );
}

#[test]
fn test_static_double_field() {
    assert_eq!(
        java::run_method("test_data/Fields", "s_double"),
        Double(400.0)
    );
}

#[test]
fn test_static_other() {
    assert_eq!(java::run_method("test_data/Fields", "other"), Int(10));
}

/*
#[test]
fn test_boolean_field() {
    assert_eq!(run_method("t_boolean"), Boolean(true));
}*/// TODO

#[test]
fn test_int_field() {
    assert_eq!(java::run_method("test_data/Fields", "t_int"), Int(0));
}

#[test]
fn test_long_field() {
    assert_eq!(java::run_method("test_data/Fields", "t_long"), Long(100));
}

#[test]
fn test_float_field() {
    assert_eq!(java::run_method("test_data/Fields", "t_float"), Float(1.0));
}

#[test]
fn test_double_field() {
    assert_eq!(
        java::run_method("test_data/Fields", "t_double"),
        Double(2.0)
    );
}
