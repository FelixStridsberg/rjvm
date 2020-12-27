use rjvm::vm::{ClassRegister, VirtualMachine};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let class_name = &args[1];
    let method_name = &args[2];

    let mut file = class_name.clone();
    file.push_str(".class");

    let mut class_register = ClassRegister::new();
    class_register.register_class(&file).unwrap();

    let mut vm = VirtualMachine::default();
    let return_value = vm.run(class_register, class_name, method_name, vec![]);
    println!("Returned {:?}", return_value);
}
