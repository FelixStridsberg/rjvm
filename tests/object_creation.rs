extern crate rjvm;

use rjvm::error::Result;
use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::VirtualMachine;

#[test]
fn create_array() -> Result<()> {
    let mut class_loader = ClassLoader::new();
    class_loader
        .load_class_file("./tests/test_data/Array.class")
        .unwrap();
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_loader,
        "test_data/Array",
        "create_int_array",
        vec![],
    );
    assert_eq!(return_value, Int(3));

    Ok(())
}

#[test]
fn create_object() -> Result<()> {
    let mut class_loader = ClassLoader::new();
    class_loader
        .load_class_file("./tests/jre/java/lang/Object.class")
        .unwrap();
    class_loader
        .load_class_file("./tests/test_data/Instance.class")
        .unwrap();

    let mut vm = VirtualMachine::default();
    let return_value = vm.run(class_loader, "test_data/Instance", "main", vec![]);
    assert_eq!(return_value, Int(3));

    Ok(())
}
