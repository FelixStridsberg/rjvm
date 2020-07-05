extern crate rjvm;

use std::fs::read;
use std::io::BufReader;

use rjvm::io::class::ClassReader;
use rjvm::vm::VirtualMachine;
use rjvm::vm::Value::{Int, Long};

#[test]
fn invoke_static_simple_no_args() {
    let file = read("./tests/test_data/Simple.class").unwrap();
    let mut reader = ClassReader::new(BufReader::new(&file[..]));

    reader.verify_meta().unwrap();
    let constants = reader.read_constant_pool().unwrap();
    let class = reader.read_class(&constants).unwrap();

    // public static void no_args()
    let method = class.find_public_static_method("no_args").unwrap();
    let vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&constants, method, vec![]);
    assert_eq!(return_value, Some(Int(1)));
}

#[test]
fn invoke_static_simple_add() {
    let file = read("./tests/test_data/Simple.class").unwrap();
    let mut reader = ClassReader::new(BufReader::new(&file[..]));

    reader.verify_meta().unwrap();
    let constants = reader.read_constant_pool().unwrap();
    let class = reader.read_class(&constants).unwrap();

    // public static int add(int a, int b)
    let method = class.find_public_static_method("add").unwrap();
    let vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&constants, method, vec![Int(1), Int(5)]);
    assert_eq!(return_value, Some(Int(6)));
}

#[test]
fn invoke_static_simple_add_long() {
    let file = read("./tests/test_data/Simple.class").unwrap();
    let mut reader = ClassReader::new(BufReader::new(&file[..]));

    reader.verify_meta().unwrap();
    let constants = reader.read_constant_pool().unwrap();
    let class = reader.read_class(&constants).unwrap();

    // public static int add(int a, int b)
    let method = class.find_public_static_method("add_long").unwrap();
    let vm = VirtualMachine::new();
    let return_value = vm.invoke_static_method(&constants, method, vec![Long(1), Long(5)]);
    assert_eq!(return_value, Some(Long(6)));
}
