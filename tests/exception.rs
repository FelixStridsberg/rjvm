use rjvm::vm::data_type::Value::Null;

#[path = "./java_utils.rs"]
mod java;

#[test]
fn throw_and_catch() {
    assert_eq!(java::run_method("test_data/Exceptions", "main"), Null);
}
