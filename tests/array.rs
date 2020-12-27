use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::VirtualMachine;
use rjvm::vm::data_type::Value::Int;

#[path="./java_utils.rs"]
mod java;

#[test]
fn create_array() {
    assert_eq!(java::run_method("test_data/Array", "create_int_array"), Int(3));
}
