extern crate rjvm;

use rjvm::error::Result;
use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value::Int;
use rjvm::vm::VirtualMachine;

#[test]
fn load_and_invoke_simple() -> Result<()> {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/"]);
    let mut vm = VirtualMachine::default();

    let return_value = vm.run(
        class_loader,
        "test_data/Simple",
        "add",
        vec![Int(1), Int(5)],
    );
    assert_eq!(return_value, Int(6));

    Ok(())
}
