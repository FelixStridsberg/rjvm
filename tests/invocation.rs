extern crate rjvm;

use rjvm::vm::data_type::Value::{Int, Long};

#[path = "utils.rs"]
mod java;

#[test]
fn invoke_static_simple_no_args() {
    assert_eq!(
        java::run_method("test_data/Invocation", "static_no_args"),
        Int(1)
    );
}

#[test]
fn invoke_static_simple_add() {
    assert_eq!(
        java::run_method_args(
            "test_data/Invocation",
            "static_int_args",
            vec![Int(1), Int(5)]
        ),
        Int(6)
    );
}

#[test]
fn invoke_static_simple_add_long() {
    assert_eq!(
        java::run_method_args(
            "test_data/Invocation",
            "static_long_args",
            vec![Long(1), Long(5)]
        ),
        Long(6)
    );
}

#[test]
fn invoke_static_nested() {
    assert_eq!(
        java::run_method_args(
            "test_data/Invocation",
            "static_nested",
            vec![Int(1), Int(5)]
        ),
        Int(6)
    );
}

#[test]
fn instance_invocation_not_args() {
    assert_eq!(
        java::run_method("test_data/Invocation", "instance_invocation_no_args"),
        Int(3)
    );
}

#[test]
fn instance_invocation_int_arg() {
    assert_eq!(
        java::run_method("test_data/Invocation", "instance_invocation_int_arg"),
        Int(4)
    );
}

#[test]
fn instance_invocation_reference_arg() {
    assert_eq!(
        java::run_method("test_data/Invocation", "instance_invocation_reference_arg"),
        Int(3)
    );
}

#[test]
fn instance_invocation_null_reference_arg() {
    assert_eq!(
        java::run_method(
            "test_data/Invocation",
            "instance_invocation_null_reference_arg"
        ),
        Int(1)
    );
}

#[test]
fn instance_invocation_different_args() {
    assert_eq!(
        java::run_method("test_data/Invocation", "instance_invocation_different_args"),
        Int(10)
    );
}

#[test]
fn interface_invocation() {
    assert_eq!(
        java::run_method("test_data/Invocation", "interface_invocation"),
        Int(100)
    );
}

#[test]
fn super_invocation() {
    assert_eq!(
        java::run_method("test_data/Invocation", "super_invocation"),
        Int(122)
    );
}
