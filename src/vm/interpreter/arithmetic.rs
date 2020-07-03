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

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret;
    use crate::vm::Frame;
    use crate::vm::Value::*;

    #[test]
    fn add() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(1));
        frame.push_operand(Int(2));
        interpret(&mut frame, &vec![Instruction::new(Iadd, vec![])]);

        frame.push_operand(Long(3));
        frame.push_operand(Long(4));
        interpret(&mut frame, &vec![Instruction::new(Ladd, vec![])]);

        frame.push_operand(Float(1.0));
        frame.push_operand(Float(2.2));
        interpret(&mut frame, &vec![Instruction::new(Fadd, vec![])]);

        frame.push_operand(Double(3.1));
        frame.push_operand(Double(4.0));
        interpret(&mut frame, &vec![Instruction::new(Dadd, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(3), Long(7), Float(3.2), Double(7.1)]
        );
    }

    #[test]
    fn sub() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(3));
        frame.push_operand(Int(2));
        interpret(&mut frame, &vec![Instruction::new(Isub, vec![])]);

        frame.push_operand(Long(2));
        frame.push_operand(Long(4));
        interpret(&mut frame, &vec![Instruction::new(Lsub, vec![])]);

        frame.push_operand(Float(1.0));
        frame.push_operand(Float(2.2));
        interpret(&mut frame, &vec![Instruction::new(Fsub, vec![])]);

        frame.push_operand(Double(3.0));
        frame.push_operand(Double(4.0));
        interpret(&mut frame, &vec![Instruction::new(Dsub, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(-1), Long(2), Float(1.2), Double(1.0)]
        );
    }

    #[test]
    fn mul() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(1));
        frame.push_operand(Int(2));
        interpret(&mut frame, &vec![Instruction::new(Imul, vec![])]);

        frame.push_operand(Long(3));
        frame.push_operand(Long(4));
        interpret(&mut frame, &vec![Instruction::new(Lmul, vec![])]);

        frame.push_operand(Float(1.0));
        frame.push_operand(Float(2.2));
        interpret(&mut frame, &vec![Instruction::new(Fmul, vec![])]);

        frame.push_operand(Double(3.1));
        frame.push_operand(Double(4.0));
        interpret(&mut frame, &vec![Instruction::new(Dmul, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(2), Long(12), Float(2.2), Double(12.4)]
        );
    }

    #[test]
    fn div() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(2));
        frame.push_operand(Int(4));
        interpret(&mut frame, &vec![Instruction::new(Idiv, vec![])]);

        frame.push_operand(Long(3));
        frame.push_operand(Long(4));
        interpret(&mut frame, &vec![Instruction::new(Ldiv, vec![])]);

        frame.push_operand(Float(1.1));
        frame.push_operand(Float(2.2));
        interpret(&mut frame, &vec![Instruction::new(Fdiv, vec![])]);

        frame.push_operand(Double(4.0));
        frame.push_operand(Double(3.1));
        interpret(&mut frame, &vec![Instruction::new(Ddiv, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(2), Long(1), Float(2.0), Double(0.775)]
        );
    }

    #[test]
    fn rem() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(2));
        frame.push_operand(Int(4));
        interpret(&mut frame, &vec![Instruction::new(Irem, vec![])]);

        frame.push_operand(Long(3));
        frame.push_operand(Long(4));
        interpret(&mut frame, &vec![Instruction::new(Lrem, vec![])]);

        frame.push_operand(Float(1.1));
        frame.push_operand(Float(2.2));
        interpret(&mut frame, &vec![Instruction::new(Frem, vec![])]);

        frame.push_operand(Double(4.0));
        frame.push_operand(Double(3.1));
        interpret(&mut frame, &vec![Instruction::new(Drem, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(0), Long(1), Float(0.0), Double(3.1)]
        );
    }

    #[test]
    fn neg() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(2));
        interpret(&mut frame, &vec![Instruction::new(Ineg, vec![])]);

        frame.push_operand(Long(3));
        interpret(&mut frame, &vec![Instruction::new(Lneg, vec![])]);

        frame.push_operand(Float(1.1));
        interpret(&mut frame, &vec![Instruction::new(Fneg, vec![])]);

        frame.push_operand(Double(4.0));
        interpret(&mut frame, &vec![Instruction::new(Dneg, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![Int(-2), Long(-3), Float(-1.1), Double(-4.0)]
        );
    }

    #[test]
    fn shift() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(1));
        frame.push_operand(Int(0x08));
        interpret(&mut frame, &vec![Instruction::new(Ishl, vec![])]);

        frame.push_operand(Int(2));
        frame.push_operand(Int(-0x01));
        interpret(&mut frame, &vec![Instruction::new(Ishr, vec![])]);

        frame.push_operand(Int(2));
        frame.push_operand(Int(-0x01));
        interpret(&mut frame, &vec![Instruction::new(Iushr, vec![])]);

        frame.push_operand(Long(1));
        frame.push_operand(Long(0x08));
        interpret(&mut frame, &vec![Instruction::new(Lshl, vec![])]);

        frame.push_operand(Long(2));
        frame.push_operand(Long(-0x01));
        interpret(&mut frame, &vec![Instruction::new(Lshr, vec![])]);

        frame.push_operand(Long(63));
        frame.push_operand(Long(-1));
        interpret(&mut frame, &vec![Instruction::new(Lushr, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![
                Int(0x10),
                Int(-1),
                Int(1073741823),
                Long(0x10),
                Long(-1),
                Long(8589934591)
            ]
        );
    }

    #[test]
    fn bitwise() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(0xf0));
        frame.push_operand(Int(0x0f));
        interpret(&mut frame, &vec![Instruction::new(Ior, vec![])]);

        frame.push_operand(Long(0xf000));
        frame.push_operand(Long(0x0fff));
        interpret(&mut frame, &vec![Instruction::new(Lor, vec![])]);

        frame.push_operand(Int(0x30));
        frame.push_operand(Int(0xff));
        interpret(&mut frame, &vec![Instruction::new(Iand, vec![])]);

        frame.push_operand(Long(0xfc00));
        frame.push_operand(Long(0x0fff));
        interpret(&mut frame, &vec![Instruction::new(Land, vec![])]);

        frame.push_operand(Int(0x30));
        frame.push_operand(Int(0xff));
        interpret(&mut frame, &vec![Instruction::new(Ixor, vec![])]);

        frame.push_operand(Long(0xfc00));
        frame.push_operand(Long(0x0fff));
        interpret(&mut frame, &vec![Instruction::new(Lxor, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![
                Int(0xff),
                Long(0xffff),
                Int(0x30),
                Long(0x0c00),
                Int(0xcf),
                Long(0xf3ff)
            ],
        );
    }

    #[test]
    fn iinc() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local(1, 0x0a);

        interpret(&mut frame, &vec![Instruction::new(Iinc, vec![0x01, 0x06])]);

        assert_eq!(frame.get_local(1), 0x10);
    }

    #[test]
    fn cmp() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Double(100.0));
        frame.push_operand(Double(-10.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpg, vec![])]);

        frame.push_operand(Double(100.0));
        frame.push_operand(Double(-10.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpl, vec![])]);

        frame.push_operand(Double(10.0));
        frame.push_operand(Double(10.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpg, vec![])]);

        frame.push_operand(Double(10.0));
        frame.push_operand(Double(10.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpl, vec![])]);

        frame.push_operand(Double(-10.0));
        frame.push_operand(Double(100.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpg, vec![])]);

        frame.push_operand(Double(-10.0));
        frame.push_operand(Double(100.0));
        interpret(&mut frame, &vec![Instruction::new(Dcmpl, vec![])]);

        frame.push_operand(Float(100.0));
        frame.push_operand(Float(-10.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpg, vec![])]);

        frame.push_operand(Float(100.0));
        frame.push_operand(Float(-10.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpl, vec![])]);

        frame.push_operand(Float(10.0));
        frame.push_operand(Float(10.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpg, vec![])]);

        frame.push_operand(Float(10.0));
        frame.push_operand(Float(10.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpl, vec![])]);

        frame.push_operand(Float(-10.0));
        frame.push_operand(Float(100.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpg, vec![])]);

        frame.push_operand(Float(-10.0));
        frame.push_operand(Float(100.0));
        interpret(&mut frame, &vec![Instruction::new(Fcmpl, vec![])]);

        frame.push_operand(Long(10));
        frame.push_operand(Long(100));
        interpret(&mut frame, &vec![Instruction::new(Lcmp, vec![])]);

        frame.push_operand(Long(50));
        frame.push_operand(Long(50));
        interpret(&mut frame, &vec![Instruction::new(Lcmp, vec![])]);

        frame.push_operand(Long(100));
        frame.push_operand(Long(10));
        interpret(&mut frame, &vec![Instruction::new(Lcmp, vec![])]);

        assert_eq!(
            frame.operand_stack,
            vec![
                Int(-1),
                Int(-1),
                Int(0),
                Int(0),
                Int(1),
                Int(1),
                Int(-1),
                Int(-1),
                Int(0),
                Int(0),
                Int(1),
                Int(1),
                Int(1),
                Int(0),
                Int(-1),
            ],
        );
    }
}
