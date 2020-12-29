use rjvm::vm::data_type::Value::Int;

#[path = "./java_utils.rs"]
mod java;

#[test]
fn int_array() {
    assert_eq!(java::run_method("test_data/Array", "int_array"), Int(3));
}

#[test]
fn byte_array() {
    assert_eq!(java::run_method("test_data/Array", "byte_array"), Int(0xff));
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
