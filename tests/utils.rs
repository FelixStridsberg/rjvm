use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value;
use rjvm::vm::native::Native;
use rjvm::vm::VirtualMachine;
use rjvm::vm::stack::Stack;

fn java_assert_equals(stack: &mut Stack) -> Option<Value> {
    let frame = stack.current_frame();
    let left = &frame.local_variables[0];
    let right = &frame.local_variables[1];

    if left == right {
        return None;
    }

    eprintln!("Stack: \n{}", stack);

    let frame = stack.current_frame();
    eprintln!("Java assertion failed:\n\n\t{:?} == {:?}\n\n", frame.local_variables[0], frame.local_variables[1]);

    panic!("Assertion failed");
}

#[allow(dead_code)]
pub fn run_method(class_name: &str, method_name: &str) -> Value {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/", "./tests/jre/"]);

    let mut native = Native::new();
    native.register_method("vadeen/test/Assertion", "assertEquals", java_assert_equals);

    let mut vm = VirtualMachine::default();
    vm.run(class_loader, native, class_name, method_name, vec![])
}

#[allow(dead_code)]
pub fn run_method_args(class_name: &str, method_name: &str, args: Vec<Value>) -> Value {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/", "./tests/jre/"]);

    let mut native = Native::new();
    native.register_method("vadeen/test/Assertion", "assertEquals", java_assert_equals);

    let mut vm = VirtualMachine::default();
    vm.run(class_loader, native, class_name, method_name, args)
}
