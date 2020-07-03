use crate::vm::Frame;
use crate::vm::Value::*;

pub fn int_to_long(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Long(int as i64));
}

pub fn int_to_float(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Float(int as f32));
}

pub fn int_to_double(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Double(int as f64));
}

pub fn long_to_float(frame: &mut Frame) {
    let long = frame.pop_operand_long();
    frame.push_operand(Float(long as f32));
}

pub fn long_to_double(frame: &mut Frame) {
    let long = frame.pop_operand_long();
    frame.push_operand(Double(long as f64));
}

pub fn float_to_double(frame: &mut Frame) {
    let float = frame.pop_operand_float();
    frame.push_operand(Double(float as f64));
}

pub fn int_to_byte(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Byte(int as u8));
}

pub fn int_to_char(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Char(int as u8 as char));
}

pub fn int_to_short(frame: &mut Frame) {
    let int = frame.pop_operand_int();
    frame.push_operand(Short(int as i16));
}

pub fn long_to_int(frame: &mut Frame) {
    let long = frame.pop_operand_long();
    frame.push_operand(Int(long as i32));
}

pub fn float_to_int(frame: &mut Frame) {
    let float = frame.pop_operand_float();
    frame.push_operand(Int(float as i32));
}

pub fn float_to_long(frame: &mut Frame) {
    let float = frame.pop_operand_float();
    frame.push_operand(Long(float as i64));
}

pub fn double_to_int(frame: &mut Frame) {
    let double = frame.pop_operand_double();
    frame.push_operand(Int(double as i32));
}

pub fn double_to_long(frame: &mut Frame) {
    let double = frame.pop_operand_double();
    frame.push_operand(Long(double as i64));
}

pub fn double_to_float(frame: &mut Frame) {
    let double = frame.pop_operand_double();
    frame.push_operand(Float(double as f32));
}