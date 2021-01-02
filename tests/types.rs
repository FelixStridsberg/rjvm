use rjvm::vm::data_type::Value::{Double, Float, Int, Long, Reference};

#[path = "./java_utils.rs"]
mod java;

#[test]
fn byte_as_int() {
    assert_eq!(java::run_method("test_data/Types", "byte_as_int"), Int(10));
}

#[test]
fn short_as_int() {
    assert_eq!(
        java::run_method("test_data/Types", "short_as_int"),
        Int(2233)
    );
}
