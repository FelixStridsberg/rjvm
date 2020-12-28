use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value;
use rjvm::vm::VirtualMachine;

pub fn run_method(class_name: &str, method_name: &str) -> Value {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/", "./tests/jre/"]);
    let mut vm = VirtualMachine::default();

    vm.run(class_loader, class_name, method_name, vec![])
}

pub fn run_method_args(class_name: &str, method_name: &str, args: Vec<Value>) -> Value {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/", "./tests/jre/"]);
    let mut vm = VirtualMachine::default();

    vm.run(class_loader, class_name, method_name, args)
}
