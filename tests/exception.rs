use rjvm::vm::data_type::Value::Int;

#[path = "utils.rs"]
mod java;

#[test]
fn throw_and_catch() {
    assert_eq!(java::run_method("test_data/Exceptions", "simple"), Int(1));
}

#[test]
fn throw_and_catch_finally() {
    assert_eq!(
        java::run_method("test_data/Exceptions", "with_finally"),
        Int(2)
    );
}
