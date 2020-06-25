extern crate rjvm;

use std::fs::read;
use std::io::BufReader;

use rjvm::io::class::ClassReader;

#[test]
fn test_simple() {
    let file = read("./tests/test_data/Simple.class").unwrap();
    let mut reader = ClassReader::new(BufReader::new(&file[..]));

    reader.verify_meta().unwrap();
    let constants = reader.read_constant_pool().unwrap();
    let class = reader.read_class(&constants).unwrap();

    println!("Constants: {:#?}", constants);
    println!("Class: {:#?}", class);
    panic!("Panic to print, no real integration test yet. See above.");
}
