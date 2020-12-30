use rjvm::vm::data_type::Value::{Double, Float, Int, Long};

#[path = "./java_utils.rs"]
mod java;

#[test]
fn short_array() {
    assert_eq!(java::run_method("test_data/Array", "short_array"), Int(3));
}

#[test]
fn int_array() {
    assert_eq!(java::run_method("test_data/Array", "int_array"), Int(3));
}

#[test]
fn long_array() {
    assert_eq!(java::run_method("test_data/Array", "long_array"), Long(3));
}

#[test]
fn byte_array() {
    assert_eq!(java::run_method("test_data/Array", "byte_array"), Int(0xff));
}

#[test]
fn char_array() {
    assert_eq!(
        java::run_method("test_data/Array", "char_array"),
        Int('b' as i32)
    );
}

#[test]
fn float_array() {
    assert_eq!(
        java::run_method("test_data/Array", "float_array"),
        Float(-8.0)
    );
}

#[test]
fn double_array() {
    assert_eq!(
        java::run_method("test_data/Array", "double_array"),
        Double(8.0)
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
