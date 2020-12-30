use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::VirtualMachine;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        println!("Usage: ./rjvm class_path class_name [method_name]")
    }

    let class_path: &Vec<&str> = &args[1].split(':').collect();
    let class_name = &args[2];
    let method_name = if args.len() == 4 { &args[3] } else { "main" };

    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(class_path.to_owned());

    let mut vm = VirtualMachine::default();
    let return_value = vm.run(class_loader, class_name, method_name, vec![]);
    println!("Returned {:?}", return_value);
}
