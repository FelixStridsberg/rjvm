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

pub fn int_increase(frame: &mut Frame, operands: &[u8]) {
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
    use crate::vm::interpreter::interpret_instruction;
    use crate::vm::Frame;
    use crate::vm::Value::*;

    #[test]
    fn iadd() {
        test_command!(
            start_stack: [Int(1), Int(2)],
            command: Iadd,
            final_stack: [Int(3)],
        );
    }

    #[test]
    fn ladd() {
        test_command!(
            start_stack: [Long(3), Long(4)],
            command: Ladd,
            final_stack: [Long(7)],
        );
    }

    #[test]
    fn fadd() {
        test_command!(
            start_stack: [Float(1.0), Float(2.2)],
            command: Fadd,
            final_stack: [Float(3.2)],
        );
    }

    #[test]
    fn dadd() {
        test_command!(
            start_stack: [Double(3.1), Double(4.0)],
            command: Dadd,
            final_stack: [Double(7.1)],
        );
    }

    #[test]
    fn isub() {
        test_command!(
            start_stack: [Int(3), Int(2)],
            command: Isub,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn lsub() {
        test_command!(
            start_stack: [Long(2), Long(4)],
            command: Lsub,
            final_stack: [Long(2)],
        );
    }

    #[test]
    fn fsub() {
        test_command!(
            start_stack: [Float(1.0), Float(2.2)],
            command: Fsub,
            final_stack: [Float(1.2)],
        );
    }

    #[test]
    fn dsub() {
        test_command!(
            start_stack: [Double(3.0), Double(4.0)],
            command: Dsub,
            final_stack: [Double(1.0)],
        );
    }

    #[test]
    fn imul() {
        test_command!(
            start_stack: [Int(1), Int(2)],
            command: Imul,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn lmul() {
        test_command!(
            start_stack: [Long(3), Long(4)],
            command: Lmul,
            final_stack: [Long(12)],
        );
    }

    #[test]
    fn fmul() {
        test_command!(
            start_stack: [Float(1.0), Float(2.2)],
            command: Fmul,
            final_stack: [Float(2.2)],
        );
    }

    #[test]
    fn dmul() {
        test_command!(
            start_stack: [Double(3.1), Double(4.0)],
            command: Dmul,
            final_stack: [Double(12.4)],
        );
    }

    #[test]
    fn idiv() {
        test_command!(
            start_stack: [Int(2), Int(4)],
            command: Idiv,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn ldiv() {
        test_command!(
            start_stack: [Long(3), Long(4)],
            command: Ldiv,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn fdiv() {
        test_command!(
            start_stack: [Float(1.1), Float(2.2)],
            command: Fdiv,
            final_stack: [Float(2.0)],
        );
    }

    #[test]
    fn ddiv() {
        test_command!(
            start_stack: [Double(4.0), Double(3.1)],
            command: Ddiv,
            final_stack: [Double(0.775)],
        );
    }

    #[test]
    fn irem() {
        test_command!(
            start_stack: [Int(2), Int(4)],
            command: Irem,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn lrem() {
        test_command!(
            start_stack: [Long(3), Long(4)],
            command: Lrem,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn frem() {
        test_command!(
            start_stack: [Float(1.1), Float(2.2)],
            command: Frem,
            final_stack: [Float(0.0)],
        );
    }

    #[test]
    fn drem() {
        test_command!(
            start_stack: [Double(4.0), Double(3.1)],
            command: Drem,
            final_stack: [Double(3.1)],
        );
    }

    #[test]
    fn ineg() {
        test_command!(
            start_stack: [Int(2)],
            command: Ineg,
            final_stack: [Int(-2)],
        );
    }

    #[test]
    fn lneg() {
        test_command!(
            start_stack: [Long(3)],
            command: Lneg,
            final_stack: [Long(-3)],
        );
    }

    #[test]
    fn fneg() {
        test_command!(
            start_stack: [Float(1.1)],
            command: Fneg,
            final_stack: [Float(-1.1)],
        );
    }

    #[test]
    fn dneg() {
        test_command!(
            start_stack: [Double(4.0)],
            command: Dneg,
            final_stack: [Double(-4.0)],
        );
    }

    #[test]
    fn ishl() {
        test_command!(
            start_stack: [Int(1), Int(0x08)],
            command: Ishl,
            final_stack: [Int(0x10)],
        );
    }

    #[test]
    fn ishr() {
        test_command!(
            start_stack: [Int(2), Int(-0x01)],
            command: Ishr,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn iushr() {
        test_command!(
            start_stack: [Int(2), Int(-0x01)],
            command: Iushr,
            final_stack: [Int(1073741823)],
        );
    }

    #[test]
    fn lshl() {
        test_command!(
            start_stack: [Long(1), Long(0x08)],
            command: Lshl,
            final_stack: [Long(0x10)],
        );
    }

    #[test]
    fn lshr() {
        test_command!(
            start_stack: [Long(2), Long(-0x01)],
            command: Lshr,
            final_stack: [Long(-1)],
        );
    }

    #[test]
    fn lushr() {
        test_command!(
            start_stack: [Long(63), Long(-1)],
            command: Lushr,
            final_stack: [Long(8589934591)],
        );
    }

    #[test]
    fn ior() {
        test_command!(
            start_stack: [Int(0xf0), Int(0x0f)],
            command: Ior,
            final_stack: [Int(0xff)],
        );
    }

    #[test]
    fn lor() {
        test_command!(
            start_stack: [Long(0xf000), Long(0x0fff)],
            command: Lor,
            final_stack: [Long(0xffff)],
        );
    }

    #[test]
    fn iand() {
        test_command!(
            start_stack: [Int(0x30), Int(0xff)],
            command: Iand,
            final_stack: [Int(0x30)],
        );
    }

    #[test]
    fn land() {
        test_command!(
            start_stack: [Long(0xfc00), Long(0x0fff)],
            command: Land,
            final_stack: [Long(0x0c00)],
        );
    }

    #[test]
    fn ixor() {
        test_command!(
            start_stack: [Int(0x30), Int(0xff)],
            command: Ixor,
            final_stack: [Int(0xcf)],
        );
    }

    #[test]
    fn lxor() {
        test_command!(
            start_stack: [Long(0xfc00), Long(0x0fff)],
            command: Lxor,
            final_stack: [Long(0xf3ff)],
        );
    }

    #[test]
    fn iinc() {
        test_command!(
            start_locals: {1 => 0x0a},
            command: Iinc; [0x01, 0x06],
            final_locals: {1 => 0x10},
        );
    }

    #[test]
    fn dcmpg_lesser() {
        test_command!(
            start_stack: [Double(100.0), Double(-10.0)],
            command: Dcmpg,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn dcmpl_lesser() {
        test_command!(
            start_stack: [Double(100.0), Double(-10.0)],
            command: Dcmpl,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn dcmpg_equal() {
        test_command!(
            start_stack: [Double(10.0), Double(10.0)],
            command: Dcmpg,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn dcmpl_equal() {
        test_command!(
            start_stack: [Double(10.0), Double(10.0)],
            command: Dcmpl,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn dcmpg_greater() {
        test_command!(
            start_stack: [Double(10.0), Double(100.0)],
            command: Dcmpg,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn dcmpl_greater() {
        test_command!(
            start_stack: [Double(10.0), Double(100.0)],
            command: Dcmpl,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn fcmpg_lesser() {
        test_command!(
            start_stack: [Float(100.0), Float(-10.0)],
            command: Fcmpg,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn fcmpl_lesser() {
        test_command!(
            start_stack: [Float(100.0), Float(-10.0)],
            command: Fcmpl,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn fcmpg_equal() {
        test_command!(
            start_stack: [Float(10.0), Float(10.0)],
            command: Fcmpg,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn fcmpl_equal() {
        test_command!(
            start_stack: [Float(10.0), Float(10.0)],
            command: Fcmpl,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn fcmpg_greater() {
        test_command!(
            start_stack: [Float(10.0), Float(100.0)],
            command: Fcmpg,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn fcmpl_greater() {
        test_command!(
            start_stack: [Float(10.0), Float(100.0)],
            command: Fcmpl,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn lcmp_lesser() {
        test_command!(
            start_stack: [Long(10), Long(100)],
            command: Lcmp,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn lcmp_equal() {
        test_command!(
            start_stack: [Long(50), Long(50)],
            command: Lcmp,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn lcmp_greater() {
        test_command!(
            start_stack: [Long(100), Long(10)],
            command: Lcmp,
            final_stack: [Int(-1)],
        );
    }
}
