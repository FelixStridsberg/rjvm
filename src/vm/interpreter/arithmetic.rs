//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.3

use crate::vm::data_type::Value::{Double, Float, Int, Long};
use crate::vm::data_type::*;
use crate::vm::frame::Frame;

macro_rules! pop2(
    ($type:path, $frame:expr) => {{
        let left = expect_type!($frame.pop_operand(), $type);
        let right = expect_type!($frame.pop_operand(), $type);
        (left, right)
    }}
);

pub fn add_int(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left + right));
}

pub fn add_long(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left + right));
}

pub fn add_float(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Float(left + right));
}

pub fn add_double(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Double(left + right));
}

pub fn sub_int(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left - right));
}

pub fn sub_long(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left - right));
}

pub fn sub_float(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Float(left - right));
}

pub fn sub_double(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Double(left - right));
}

pub fn mul_int(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left * right));
}

pub fn mul_long(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left * right));
}

pub fn mul_float(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Float(left * right));
}

pub fn mul_double(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Double(left * right));
}

pub fn div_int(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left / right));
}

pub fn div_long(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left / right));
}

pub fn div_float(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Float(left / right));
}

pub fn div_double(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Double(left / right));
}

pub fn rem_int(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left % right));
}

pub fn rem_long(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left % right));
}

pub fn rem_float(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Float(left % right));
}

pub fn rem_double(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Double(left % right));
}

pub fn neg_int(frame: &mut Frame) {
    let value = frame.pop_operand().expect_int();
    frame.push_operand(Int(-value));
}

pub fn neg_long(frame: &mut Frame) {
    let value = frame.pop_operand().expect_long();
    frame.push_operand(Long(-value));
}

pub fn neg_float(frame: &mut Frame) {
    let value = frame.pop_operand().expect_float();
    frame.push_operand(Float(-value));
}

pub fn neg_double(frame: &mut Frame) {
    let value = frame.pop_operand().expect_double();
    frame.push_operand(Double(-value));
}

pub fn int_shift_left(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int((left as i32) << (right as i32 & 0x1f) as IntType));
}

pub fn int_shift_right(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left as i32 >> (right as i32 & 0x1f) as IntType));
}

pub fn int_logical_shift_right(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int((left as u32 >> (right as u32 & 0x1f)) as IntType));
}

pub fn long_shift_left(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long((left as i64) << (right as i64 & 0x1f) as LongType));
}

pub fn long_shift_right(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left as i64 >> (right as i64 & 0x1f) as LongType));
}

pub fn long_logical_shift_right(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long((left as u64 >> (right as u64 & 0x1f)) as LongType));
}

pub fn int_bitwise_or(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left | right));
}

pub fn long_bitwise_or(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left | right));
}

pub fn int_bitwise_and(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left & right));
}

pub fn long_bitwise_and(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left & right));
}

pub fn int_bitwise_exclusive_or(frame: &mut Frame) {
    let (left, right) = pop2!(Int, frame);
    frame.push_operand(Int(left ^ right));
}

pub fn long_bitwise_exclusive_or(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Long(left ^ right));
}

pub fn int_increase(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    let constant = operands[1] as u32;
    let value = frame.get_local(index);
    frame.set_local(index, value + constant);
}

// TODO NaN
pub fn double_compare_g(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Int(compare(left, right)))
}

// TODO NaN
pub fn double_compare_l(frame: &mut Frame) {
    let (left, right) = pop2!(Double, frame);
    frame.push_operand(Int(compare(left, right)))
}

// TODO NaN
pub fn float_compare_g(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Int(compare(left, right)))
}

// TODO NaN
pub fn float_compare_l(frame: &mut Frame) {
    let (left, right) = pop2!(Float, frame);
    frame.push_operand(Int(compare(left, right)))
}

pub fn long_compare(frame: &mut Frame) {
    let (left, right) = pop2!(Long, frame);
    frame.push_operand(Int(compare(left, right)))
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
    use crate::class::code::Opcode::*;
    use crate::vm::data_type::Value::*;

    #[test]
    fn iadd() {
        test_instruction!(
            start_stack: [Int(1), Int(2)],
            instruction: Iadd,
            final_stack: [Int(3)],
        );
    }

    #[test]
    fn ladd() {
        test_instruction!(
            start_stack: [Long(3), Long(4)],
            instruction: Ladd,
            final_stack: [Long(7)],
        );
    }

    #[test]
    fn fadd() {
        test_instruction!(
            start_stack: [Float(1.0), Float(2.2)],
            instruction: Fadd,
            final_stack: [Float(3.2)],
        );
    }

    #[test]
    fn dadd() {
        test_instruction!(
            start_stack: [Double(3.1), Double(4.0)],
            instruction: Dadd,
            final_stack: [Double(7.1)],
        );
    }

    #[test]
    fn isub() {
        test_instruction!(
            start_stack: [Int(3), Int(2)],
            instruction: Isub,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn lsub() {
        test_instruction!(
            start_stack: [Long(2), Long(4)],
            instruction: Lsub,
            final_stack: [Long(2)],
        );
    }

    #[test]
    fn fsub() {
        test_instruction!(
            start_stack: [Float(1.0), Float(2.2)],
            instruction: Fsub,
            final_stack: [Float(1.2)],
        );
    }

    #[test]
    fn dsub() {
        test_instruction!(
            start_stack: [Double(3.0), Double(4.0)],
            instruction: Dsub,
            final_stack: [Double(1.0)],
        );
    }

    #[test]
    fn imul() {
        test_instruction!(
            start_stack: [Int(1), Int(2)],
            instruction: Imul,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn lmul() {
        test_instruction!(
            start_stack: [Long(3), Long(4)],
            instruction: Lmul,
            final_stack: [Long(12)],
        );
    }

    #[test]
    fn fmul() {
        test_instruction!(
            start_stack: [Float(1.0), Float(2.2)],
            instruction: Fmul,
            final_stack: [Float(2.2)],
        );
    }

    #[test]
    fn dmul() {
        test_instruction!(
            start_stack: [Double(3.1), Double(4.0)],
            instruction: Dmul,
            final_stack: [Double(12.4)],
        );
    }

    #[test]
    fn idiv() {
        test_instruction!(
            start_stack: [Int(2), Int(4)],
            instruction: Idiv,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn ldiv() {
        test_instruction!(
            start_stack: [Long(3), Long(4)],
            instruction: Ldiv,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn fdiv() {
        test_instruction!(
            start_stack: [Float(1.1), Float(2.2)],
            instruction: Fdiv,
            final_stack: [Float(2.0)],
        );
    }

    #[test]
    fn ddiv() {
        test_instruction!(
            start_stack: [Double(4.0), Double(3.1)],
            instruction: Ddiv,
            final_stack: [Double(0.775)],
        );
    }

    #[test]
    fn irem() {
        test_instruction!(
            start_stack: [Int(2), Int(4)],
            instruction: Irem,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn lrem() {
        test_instruction!(
            start_stack: [Long(3), Long(4)],
            instruction: Lrem,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn frem() {
        test_instruction!(
            start_stack: [Float(1.1), Float(2.2)],
            instruction: Frem,
            final_stack: [Float(0.0)],
        );
    }

    #[test]
    fn drem() {
        test_instruction!(
            start_stack: [Double(4.0), Double(3.1)],
            instruction: Drem,
            final_stack: [Double(3.1)],
        );
    }

    #[test]
    fn ineg() {
        test_instruction!(
            start_stack: [Int(2)],
            instruction: Ineg,
            final_stack: [Int(-2)],
        );
    }

    #[test]
    fn lneg() {
        test_instruction!(
            start_stack: [Long(3)],
            instruction: Lneg,
            final_stack: [Long(-3)],
        );
    }

    #[test]
    fn fneg() {
        test_instruction!(
            start_stack: [Float(1.1)],
            instruction: Fneg,
            final_stack: [Float(-1.1)],
        );
    }

    #[test]
    fn dneg() {
        test_instruction!(
            start_stack: [Double(4.0)],
            instruction: Dneg,
            final_stack: [Double(-4.0)],
        );
    }

    #[test]
    fn ishl() {
        test_instruction!(
            start_stack: [Int(1), Int(0x08)],
            instruction: Ishl,
            final_stack: [Int(0x10)],
        );
    }

    #[test]
    fn ishr() {
        test_instruction!(
            start_stack: [Int(2), Int(-0x01)],
            instruction: Ishr,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn iushr() {
        test_instruction!(
            start_stack: [Int(2), Int(-0x01)],
            instruction: Iushr,
            final_stack: [Int(1073741823)],
        );
    }

    #[test]
    fn lshl() {
        test_instruction!(
            start_stack: [Long(1), Long(0x08)],
            instruction: Lshl,
            final_stack: [Long(0x10)],
        );
    }

    #[test]
    fn lshr() {
        test_instruction!(
            start_stack: [Long(2), Long(-0x01)],
            instruction: Lshr,
            final_stack: [Long(-1)],
        );
    }

    #[test]
    fn lushr() {
        test_instruction!(
            start_stack: [Long(63), Long(-1)],
            instruction: Lushr,
            final_stack: [Long(8589934591)],
        );
    }

    #[test]
    fn ior() {
        test_instruction!(
            start_stack: [Int(0xf0), Int(0x0f)],
            instruction: Ior,
            final_stack: [Int(0xff)],
        );
    }

    #[test]
    fn lor() {
        test_instruction!(
            start_stack: [Long(0xf000), Long(0x0fff)],
            instruction: Lor,
            final_stack: [Long(0xffff)],
        );
    }

    #[test]
    fn iand() {
        test_instruction!(
            start_stack: [Int(0x30), Int(0xff)],
            instruction: Iand,
            final_stack: [Int(0x30)],
        );
    }

    #[test]
    fn land() {
        test_instruction!(
            start_stack: [Long(0xfc00), Long(0x0fff)],
            instruction: Land,
            final_stack: [Long(0x0c00)],
        );
    }

    #[test]
    fn ixor() {
        test_instruction!(
            start_stack: [Int(0x30), Int(0xff)],
            instruction: Ixor,
            final_stack: [Int(0xcf)],
        );
    }

    #[test]
    fn lxor() {
        test_instruction!(
            start_stack: [Long(0xfc00), Long(0x0fff)],
            instruction: Lxor,
            final_stack: [Long(0xf3ff)],
        );
    }

    #[test]
    fn iinc() {
        test_instruction!(
            start_locals: {1 => 0x0a},
            instruction: Iinc; [0x01, 0x06],
            final_locals: {1 => 0x10},
        );
    }

    #[test]
    fn dcmpg_lesser() {
        test_instruction!(
            start_stack: [Double(100.0), Double(-10.0)],
            instruction: Dcmpg,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn dcmpl_lesser() {
        test_instruction!(
            start_stack: [Double(100.0), Double(-10.0)],
            instruction: Dcmpl,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn dcmpg_equal() {
        test_instruction!(
            start_stack: [Double(10.0), Double(10.0)],
            instruction: Dcmpg,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn dcmpl_equal() {
        test_instruction!(
            start_stack: [Double(10.0), Double(10.0)],
            instruction: Dcmpl,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn dcmpg_greater() {
        test_instruction!(
            start_stack: [Double(10.0), Double(100.0)],
            instruction: Dcmpg,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn dcmpl_greater() {
        test_instruction!(
            start_stack: [Double(10.0), Double(100.0)],
            instruction: Dcmpl,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn fcmpg_lesser() {
        test_instruction!(
            start_stack: [Float(100.0), Float(-10.0)],
            instruction: Fcmpg,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn fcmpl_lesser() {
        test_instruction!(
            start_stack: [Float(100.0), Float(-10.0)],
            instruction: Fcmpl,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn fcmpg_equal() {
        test_instruction!(
            start_stack: [Float(10.0), Float(10.0)],
            instruction: Fcmpg,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn fcmpl_equal() {
        test_instruction!(
            start_stack: [Float(10.0), Float(10.0)],
            instruction: Fcmpl,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn fcmpg_greater() {
        test_instruction!(
            start_stack: [Float(10.0), Float(100.0)],
            instruction: Fcmpg,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn fcmpl_greater() {
        test_instruction!(
            start_stack: [Float(10.0), Float(100.0)],
            instruction: Fcmpl,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn lcmp_lesser() {
        test_instruction!(
            start_stack: [Long(10), Long(100)],
            instruction: Lcmp,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn lcmp_equal() {
        test_instruction!(
            start_stack: [Long(50), Long(50)],
            instruction: Lcmp,
            final_stack: [Int(0)],
        );
    }

    #[test]
    fn lcmp_greater() {
        test_instruction!(
            start_stack: [Long(100), Long(10)],
            instruction: Lcmp,
            final_stack: [Int(-1)],
        );
    }
}
