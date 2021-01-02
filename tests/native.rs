use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::native::Native;
use rjvm::vm::VirtualMachine;

#[test]
fn test_load_archive() {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/"]);

    let mut native = Native::new();
    native.register_method("test_data/Native", "native_method", |_| Some(Int(20)));

    let mut vm = VirtualMachine::default();
    let value = vm.run(
        class_loader,
        native,
        "test_data/Native",
        "call_native",
        vec![],
    );

    assert_eq!(value, Int(20))
}
