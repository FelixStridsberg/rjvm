/// Contains implementation of all instructions under:
/// https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.2
use crate::vm::Frame;
use crate::vm::Value::{Double, Float, Int, Long, Reference};

pub fn load_int(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    load_int_n(frame, index);
}

pub fn load_int_n(frame: &mut Frame, index: u16) {
    let int = frame.get_local(index) as i32;
    frame.push_operand(Int(int));
}

pub fn load_long(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    load_long_n(frame, index);
}

pub fn load_long_n(frame: &mut Frame, index: u16) {
    let i1 = frame.get_local(index) as i64;
    let i2 = frame.get_local(index + 1) as i64;
    frame.push_operand(Long(i1 << 32 | i2));
}

pub fn load_float(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    load_float_n(frame, index);
}

pub fn load_float_n(frame: &mut Frame, index: u16) {
    let bits = frame.get_local(index);
    frame.push_operand(Float(f32::from_bits(bits)));
}

pub fn load_double(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    load_double_n(frame, index);
}

pub fn load_double_n(frame: &mut Frame, index: u16) {
    let i1 = frame.get_local(index) as u64;
    let i2 = frame.get_local(index + 1) as u64;
    let bits = i1 << 32 | i2;
    frame.push_operand(Double(f64::from_bits(bits)));
}

pub fn load_reference(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    load_reference_n(frame, index);
}

pub fn load_reference_n(frame: &mut Frame, index: u16) {
    let int = frame.get_local(index) as i32;
    frame.push_operand(Reference(int));
}

pub fn store_int(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn store_int_n(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Int(value) = operand {
        frame.set_local(index, value as u32);
    } else {
        panic!(
            "istoreX expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_long(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn store_long_n(_frame: &mut Frame, _index: u16) {
    unimplemented!();
}

pub fn store_float(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn store_float_n(_frame: &mut Frame, _index: u16) {
    unimplemented!();
}

pub fn store_double(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn store_double_n(_frame: &mut Frame, _index: u16) {
    unimplemented!();
}

pub fn store_reference(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn store_reference_n(_frame: &mut Frame, _index: u16) {
    unimplemented!();
}

pub fn push_byte(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn push_short(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn push_constant(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn push_constant_wide(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn push_constant_long(_frame: &mut Frame, _operands: &Vec<u8>) {
    unimplemented!();
}

pub fn push_null(_frame: &mut Frame) {
    unimplemented!();
}
