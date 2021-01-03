use rjvm::vm::class_loader::ClassLoader;
use rjvm::vm::data_type::Value;
use rjvm::vm::native::Native;
use rjvm::vm::stack::Stack;
use rjvm::vm::VirtualMachine;
use std::fs::read_dir;
use std::path::Path;

fn java_assert_equals(stack: &mut Stack) -> Option<Value> {
    let frame = stack.current_frame();
    let left = &frame.local_variables[0];
    let right = &frame.local_variables[left.get_category() as usize];

    if left == right {
        return None;
    }

    eprintln!("Stack: \n{}", stack);

    eprintln!("Java assertion failed:\n\n\t{:?} == {:?}\n\n", left, right);
    panic!("Assertion failed");
}

#[test]
fn run_java_tests() {
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/java_lib/", "./tests/"]);

    let classes = find_test_classes(&mut class_loader);

    // We must use a new class loader since we don't do the static initialization properly above.
    let mut class_loader = ClassLoader::new();
    class_loader.set_paths(vec!["./tests/java_lib/", "./tests/"]);

    let mut native = Native::new();
    native.register_method("vadeen/test/Assertion", "assertEquals", java_assert_equals);

    let mut vm = VirtualMachine::default();
    for (class_name, test_methods) in classes {
        for method in &test_methods {
            vm.run(&mut class_loader, &mut native, &class_name, method, vec![]);
        }
    }
}

fn find_test_classes(class_loader: &mut ClassLoader) -> Vec<(String, Vec<String>)> {
    let java_files: Vec<String> = read_dir("./tests/java_tests")
        .unwrap()
        .filter_map(Result::ok)
        .map(|f| f.path().to_str().unwrap_or("").to_owned())
        .filter(|f| f.ends_with("Tests.java"))
        .collect();

    let mut result = Vec::new();
    for file in java_files {
        let class_file = file.replace(".java", ".class");
        if !Path::new(&class_file).exists() {
            panic!(
                "Did not find {}, please compile the java files before running test.",
                class_file
            );
        }

        let class_name = class_file.replace("./tests/", "").replace(".class", "");
        let (class, _) = class_loader
            .resolve(&class_name)
            .expect("Could not load class");
        let test_methods: Vec<String> = class
            .methods
            .iter()
            .map(|m| m.name.clone())
            .filter(|s| s.starts_with("test_"))
            .collect();

        result.push((class_name, test_methods))
    }

    return result;
}
