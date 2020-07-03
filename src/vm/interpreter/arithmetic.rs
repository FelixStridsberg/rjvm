//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.3

use crate::vm::Frame;
use crate::vm::Value::{Double, Float, Int, Long};

pub fn add_int(frame: &mut Frame) {
    let value = frame.pop_operand_int() + frame.pop_operand_int();
    frame.push_operand(Int(value));
}

pub fn add_long(frame: &mut Frame) {
    let value = frame.pop_operand_long() + frame.pop_operand_long();
    frame.push_operand(Long(value));
}

pub fn add_float(frame: &mut Frame) {
    let value = frame.pop_operand_float() + frame.pop_operand_float();
    frame.push_operand(Float(value));
}

pub fn add_double(frame: &mut Frame) {
    let value = frame.pop_operand_double() + frame.pop_operand_double();
    frame.push_operand(Double(value));
}

pub fn sub_int(frame: &mut Frame) {
    let value = frame.pop_operand_int() - frame.pop_operand_int();
    frame.push_operand(Int(value));
}

pub fn sub_long(frame: &mut Frame) {
    let value = frame.pop_operand_long() - frame.pop_operand_long();
    frame.push_operand(Long(value));
}

pub fn sub_float(frame: &mut Frame) {
    let value = frame.pop_operand_float() - frame.pop_operand_float();
    frame.push_operand(Float(value));
}

pub fn sub_double(frame: &mut Frame) {
    let value = frame.pop_operand_double() - frame.pop_operand_double();
    frame.push_operand(Double(value));
}

pub fn mul_int(frame: &mut Frame) {
    let value = frame.pop_operand_int() * frame.pop_operand_int();
    frame.push_operand(Int(value));
}

pub fn mul_long(frame: &mut Frame) {
    let value = frame.pop_operand_long() * frame.pop_operand_long();
    frame.push_operand(Long(value));
}

pub fn mul_float(frame: &mut Frame) {
    let value = frame.pop_operand_float() * frame.pop_operand_float();
    frame.push_operand(Float(value));
}

pub fn mul_double(frame: &mut Frame) {
    let value = frame.pop_operand_double() * frame.pop_operand_double();
    frame.push_operand(Double(value));
}

pub fn div_int(frame: &mut Frame) {
    let value = frame.pop_operand_int() / frame.pop_operand_int();
    frame.push_operand(Int(value));
}

pub fn div_long(frame: &mut Frame) {
    let value = frame.pop_operand_long() / frame.pop_operand_long();
    frame.push_operand(Long(value));
}

pub fn div_float(frame: &mut Frame) {
    let value = frame.pop_operand_float() / frame.pop_operand_float();
    frame.push_operand(Float(value));
}

pub fn div_double(frame: &mut Frame) {
    let value = frame.pop_operand_double() / frame.pop_operand_double();
    frame.push_operand(Double(value));
}

pub fn rem_int(frame: &mut Frame) {
    let value = frame.pop_operand_int() % frame.pop_operand_int();
    frame.push_operand(Int(value));
}

pub fn rem_long(frame: &mut Frame) {
    let value = frame.pop_operand_long() % frame.pop_operand_long();
    frame.push_operand(Long(value));
}

pub fn rem_float(frame: &mut Frame) {
    let value = frame.pop_operand_float() % frame.pop_operand_float();
    frame.push_operand(Float(value));
}

pub fn rem_double(frame: &mut Frame) {
    let value = frame.pop_operand_double() % frame.pop_operand_double();
    frame.push_operand(Double(value));
}

pub fn neg_int(frame: &mut Frame) {
    let value = frame.pop_operand_int();
    frame.push_operand(Int(-value));
}

pub fn neg_long(frame: &mut Frame) {
    let value = frame.pop_operand_long();
    frame.push_operand(Long(-value));
}

pub fn neg_float(frame: &mut Frame) {
    let value = frame.pop_operand_float();
    frame.push_operand(Float(-value));
}

pub fn neg_double(frame: &mut Frame) {
    let value = frame.pop_operand_double();
    frame.push_operand(Double(-value));
}

pub fn int_shift_left(frame: &mut Frame) {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    frame.push_operand(Int(value1 << (value2 & 0x1f)));
}

pub fn int_shift_right(frame: &mut Frame) {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    frame.push_operand(Int(value1 >> (value2 & 0x1f)));
}

pub fn int_logical_shift_right(frame: &mut Frame) {
    let value1 = frame.pop_operand_int() as u32;
    let value2 = frame.pop_operand_int() as u32;
    frame.push_operand(Int((value1 >> (value2 & 0x1f)) as i32));
}

pub fn long_shift_left(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
    frame.push_operand(Long(value1 << (value2 & 0x1f)));
}

pub fn long_shift_right(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
    frame.push_operand(Long(value1 >> (value2 & 0x1f)));
}

pub fn long_logical_shift_right(frame: &mut Frame) {
    let value1 = frame.pop_operand_long() as u64;
    let value2 = frame.pop_operand_long() as u64;
    frame.push_operand(Long((value1 >> (value2 & 0x1f)) as i64));
}

pub fn int_bitwise_or(frame: &mut Frame) {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    frame.push_operand(Int(value1 | value2));
}

pub fn long_bitwise_or(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
    frame.push_operand(Long(value1 | value2));
}

pub fn int_bitwise_and(frame: &mut Frame) {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    frame.push_operand(Int(value1 & value2));
}

pub fn long_bitwise_and(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
    frame.push_operand(Long(value1 & value2));
}

pub fn int_bitwise_exclusive_or(frame: &mut Frame) {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    frame.push_operand(Int(value1 ^ value2));
}

pub fn long_bitwise_exclusive_or(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
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
    let value1 = frame.pop_operand_double();
    let value2 = frame.pop_operand_double();
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn double_compare_l(frame: &mut Frame) {
    let value1 = frame.pop_operand_double();
    let value2 = frame.pop_operand_double();
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn float_compare_g(frame: &mut Frame) {
    let value1 = frame.pop_operand_float();
    let value2 = frame.pop_operand_float();
    frame.push_operand(Int(compare(value1, value2)))
}

// TODO NaN
pub fn float_compare_l(frame: &mut Frame) {
    let value1 = frame.pop_operand_float();
    let value2 = frame.pop_operand_float();
    frame.push_operand(Int(compare(value1, value2)))
}

pub fn long_compare(frame: &mut Frame) {
    let value1 = frame.pop_operand_long();
    let value2 = frame.pop_operand_long();
    frame.push_operand(Int(compare(value1, value2)))
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
