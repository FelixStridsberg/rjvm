use rjvm::vm::data_type::Value::Int;

#[path = "utils.rs"]
mod java;

#[test]
fn switch_simple() {
    assert_eq!(java::run_method("test_data/Switch", "simple"), Int(2));
}

#[test]
fn switch_no_match() {
    assert_eq!(java::run_method("test_data/Switch", "no_match"), Int(0));
}

#[test]
fn switch_default() {
    assert_eq!(java::run_method("test_data/Switch", "default_case"), Int(1));
}

#[test]
fn table_switch_simple() {
    assert_eq!(
        java::run_method("test_data/Switch", "table_switch_simple"),
        Int(2)
    );
}

#[test]
fn table_switch_default() {
    assert_eq!(java::run_method("test_data/Switch", "table_switch"), Int(4));
}
