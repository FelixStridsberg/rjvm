extern crate rjvm;

use rjvm::vm::data_type::Value::Int;

#[path="./java_utils.rs"]
mod java;

#[test]
fn create_object() {
    assert_eq!(java::run_method("test_data/Instance", "main"), Int(3));
}
