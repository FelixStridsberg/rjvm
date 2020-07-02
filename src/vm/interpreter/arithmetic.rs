//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.3

use crate::vm::Frame;
use crate::vm::Value::{Double, Float, Int, Long};

pub fn add_int(frame: &mut Frame) {
    let value = get_int(frame) + get_int(frame);
    frame.push_operand(Int(value));
}

pub fn add_long(frame: &mut Frame) {
    let value = get_long(frame) + get_long(frame);
    frame.push_operand(Long(value));
}

pub fn add_float(frame: &mut Frame) {
    let value = get_float(frame) + get_float(frame);
    frame.push_operand(Float(value));
}

pub fn add_double(frame: &mut Frame) {
    let value = get_double(frame) + get_double(frame);
    frame.push_operand(Double(value));
}

pub fn sub_int(frame: &mut Frame) {
    let value = get_int(frame) - get_int(frame);
    frame.push_operand(Int(value));
}

pub fn sub_long(frame: &mut Frame) {
    let value = get_long(frame) - get_long(frame);
    frame.push_operand(Long(value));
}

pub fn sub_float(frame: &mut Frame) {
    let value = get_float(frame) - get_float(frame);
    frame.push_operand(Float(value));
}

pub fn sub_double(frame: &mut Frame) {
    let value = get_double(frame) - get_double(frame);
    frame.push_operand(Double(value));
}

pub fn mul_int(frame: &mut Frame) {
    let value = get_int(frame) * get_int(frame);
    frame.push_operand(Int(value));
}

pub fn mul_long(frame: &mut Frame) {
    let value = get_long(frame) * get_long(frame);
    frame.push_operand(Long(value));
}

pub fn mul_float(frame: &mut Frame) {
    let value = get_float(frame) * get_float(frame);
    frame.push_operand(Float(value));
}

pub fn mul_double(frame: &mut Frame) {
    let value = get_double(frame) * get_double(frame);
    frame.push_operand(Double(value));
}

pub fn div_int(frame: &mut Frame) {
    let value = get_int(frame) / get_int(frame);
    frame.push_operand(Int(value));
}

pub fn div_long(frame: &mut Frame) {
    let value = get_long(frame) / get_long(frame);
    frame.push_operand(Long(value));
}

pub fn div_float(frame: &mut Frame) {
    let value = get_float(frame) / get_float(frame);
    frame.push_operand(Float(value));
}

pub fn div_double(frame: &mut Frame) {
    let value = get_double(frame) / get_double(frame);
    frame.push_operand(Double(value));
}

pub fn rem_int(frame: &mut Frame) {
    let value = get_int(frame) % get_int(frame);
    frame.push_operand(Int(value));
}

pub fn rem_long(frame: &mut Frame) {
    let value = get_long(frame) % get_long(frame);
    frame.push_operand(Long(value));
}

pub fn rem_float(frame: &mut Frame) {
    let value = get_float(frame) % get_float(frame);
    frame.push_operand(Float(value));
}

pub fn rem_double(frame: &mut Frame) {
    let value = get_double(frame) % get_double(frame);
    frame.push_operand(Double(value));
}

pub fn neg_int(frame: &mut Frame) {
    let value = get_int(frame);
    frame.push_operand(Int(-value));
}

pub fn neg_long(frame: &mut Frame) {
    let value = get_long(frame);
    frame.push_operand(Long(-value));
}

pub fn neg_float(frame: &mut Frame) {
    let value = get_float(frame);
    frame.push_operand(Float(-value));
}

pub fn neg_double(frame: &mut Frame) {
    let value = get_double(frame);
    frame.push_operand(Double(-value));
}

pub fn int_shift_left(frame: &mut Frame) {
    let value1 = get_int(frame);
    let value2 = get_int(frame);
    frame.push_operand(Int(value1 << (value2 & 0x1f)));
}

pub fn int_shift_right(frame: &mut Frame) {
    let value1 = get_int(frame);
    let value2 = get_int(frame);
    frame.push_operand(Int(value1 >> (value2 & 0x1f)));
}

pub fn int_logical_shift_right(frame: &mut Frame) {
    let value1 = get_int(frame) as u32;
    let value2 = get_int(frame) as u32;
    frame.push_operand(Int((value1 >> (value2 & 0x1f)) as i32));
}

pub fn long_shift_left(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Long(value1 << (value2 & 0x1f)));
}

pub fn long_shift_right(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Long(value1 >> (value2 & 0x1f)));
}

pub fn long_logical_shift_right(frame: &mut Frame) {
    let value1 = get_long(frame) as u64;
    let value2 = get_long(frame) as u64;
    frame.push_operand(Long((value1 >> (value2 & 0x1f)) as i64));
}

pub fn int_bitwise_or(frame: &mut Frame) {
    let value1 = get_int(frame);
    let value2 = get_int(frame);
    frame.push_operand(Int(value1 | value2));
}

pub fn long_bitwise_or(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Long(value1 | value2));
}

pub fn int_bitwise_and(frame: &mut Frame) {
    let value1 = get_int(frame);
    let value2 = get_int(frame);
    frame.push_operand(Int(value1 & value2));
}

pub fn long_bitwise_and(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Long(value1 & value2));
}

pub fn int_bitwise_exclusive_or(frame: &mut Frame) {
    let value1 = get_int(frame);
    let value2 = get_int(frame);
    frame.push_operand(Int(value1 ^ value2));
}

pub fn long_bitwise_exclusive_or(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Long(value1 ^ value2));
}

pub fn int_increase(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    let constant = operands[1] as u32;
    let value = frame.get_local(index);
    frame.set_local(index, value + constant);
}

// TODO NaN
pub fn double_compare_g(frame: &mut Frame) {
    let value1 = get_double(frame);
    let value2 = get_double(frame);
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn double_compare_l(frame: &mut Frame) {
    let value1 = get_double(frame);
    let value2 = get_double(frame);
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn float_compare_g(frame: &mut Frame) {
    let value1 = get_float(frame);
    let value2 = get_float(frame);
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn float_compare_l(frame: &mut Frame) {
    let value1 = get_float(frame);
    let value2 = get_float(frame);
    frame.push_operand(Int(compare(value1, value2)))
}

pub fn long_compare(frame: &mut Frame) {
    let value1 = get_long(frame);
    let value2 = get_long(frame);
    frame.push_operand(Int(compare(value1, value2)))
}

fn get_int(frame: &mut Frame) -> i32 {
    match frame.pop_operand() {
        Int(i) => i,
        op => panic!("Expected int to pop, found {:?}", op),
    }
}

fn get_long(frame: &mut Frame) -> i64 {
    match frame.pop_operand() {
        Long(l) => l,
        op => panic!("Expected long to pop, found {:?}", op),
    }
}

fn get_float(frame: &mut Frame) -> f32 {
    match frame.pop_operand() {
        Float(f) => f,
        op => panic!("Expected float to pop, found {:?}", op),
    }
}

fn get_double(frame: &mut Frame) -> f64 {
    match frame.pop_operand() {
        Double(d) => d,
        op => panic!("Expected double to pop, found {:?}", op),
    }
}

fn compare<O: PartialOrd>(value1: O, value2: O) -> i32 {
    if value1 == value2 {
        0
    } else if value1 > value2 {
        1
    } else {
        -1
    }
}
