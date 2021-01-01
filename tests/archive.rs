use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::VirtualMachine;

#[test]
fn test_load_archive() {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/archive.jar"]);
    let mut vm = VirtualMachine::default();

    let value = vm.run(
        class_loader,
        "test_data/Archive",
        "hello_from_archive",
        vec![],
    );
    assert_eq!(value, Int(2))
}
