use crate::class::constant::Constant;
/// Contains implementation of all instructions under:
/// https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.2
use crate::vm::Frame;
use crate::vm::Value::*;

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

pub fn store_int(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    store_int_n(frame, index);
}

pub fn store_int_n(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Int(value) = operand {
        frame.set_local(index, value as u32);
    } else {
        panic!(
            "istore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_long(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    store_long_n(frame, index);
}

pub fn store_long_n(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Long(value) = operand {
        frame.set_local_long(index, value as u64);
    } else {
        panic!(
            "lstore_<n> expected a long value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_float(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    store_float_n(frame, index);
}

pub fn store_float_n(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Float(value) = operand {
        frame.set_local(index, value.to_bits());
    } else {
        panic!(
            "fstore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_double(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    store_double_n(frame, index);
}

pub fn store_double_n(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Double(value) = operand {
        frame.set_local_long(index, value.to_bits());
    } else {
        panic!(
            "fstore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_reference(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    store_reference_n(frame, index);
}

pub fn store_reference_n(frame: &mut Frame, index: u16) {
    let value = match frame.pop_operand() {
        Reference(r) => r,
        ReturnAddress(r) => r,
        operand => panic!(
            "astore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        ),
    };
    frame.set_local(index, value as u32);
}

pub fn push_byte(frame: &mut Frame, operands: &Vec<u8>) {
    frame.push_operand(Int(operands[0] as i32));
}

pub fn push_short(frame: &mut Frame, operands: &Vec<u8>) {
    let b1 = operands[0] as i16;
    let b2 = operands[1] as i16;
    frame.push_operand(Short((b1 << 8) | b2));
}

pub fn push_constant(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    push_constant_index(frame, index);
}

pub fn push_constant_wide(frame: &mut Frame, operands: &Vec<u8>) {
    let index_b1 = operands[0] as u16;
    let index_b2 = operands[1] as u16;
    push_constant_index(frame, (index_b1 << 2) | index_b2);
}

fn push_constant_index(frame: &mut Frame, index: u16) {
    match frame.constant_pool.get(index) {
        Constant::Integer(i) => frame.push_operand(Int(*i)),
        Constant::Float(f) => frame.push_operand(Float(*f)),
        // TODO reference and reference resolution
        constant => panic!("ldc not implemented for constant {:?}", constant),
    }
}

pub fn push_constant_long(frame: &mut Frame, operands: &Vec<u8>) {
    let index_b1 = operands[0] as u16;
    let index_b2 = operands[1] as u16;
    let index = (index_b1 << 2) | index_b2;

    match frame.constant_pool.get(index) {
        Constant::Long(l) => frame.push_operand(Long(*l)),
        Constant::Double(d) => frame.push_operand(Double(*d)),
        // TODO reference and reference resolution
        constant => panic!("ldc2w not implemented for constant {:?}", constant),
    }
}

pub fn push_null(_frame: &mut Frame) {
    unimplemented!();
}
