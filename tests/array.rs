use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::VirtualMachine;

#[path = "./java_utils.rs"]
mod java;

#[test]
fn create_array() {
    assert_eq!(
        java::run_method("test_data/Array", "create_int_array"),
        Int(3)
    );
}

#[test]
fn array_length() {
    assert_eq!(
        java::run_method("test_data/Array", "array_length"),
        Int(100)
    );
}

/* TODO implement after exceptions
#[test]
fn array_length_npe() {
    assert_eq!(java::run_method("test_data/Array", "array_length_npe"), Int(100));
}*/
