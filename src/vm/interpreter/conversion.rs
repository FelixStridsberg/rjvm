use crate::vm::data_type::Value::*;
use crate::vm::data_type::{DoubleType, FloatType, IntType, LongType, ShortType};
use crate::vm::frame::Frame;

pub fn int_to_long(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Long(int as LongType));
}

pub fn int_to_float(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Float(int as FloatType));
}

pub fn int_to_double(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Double(int as DoubleType));
}

pub fn long_to_float(frame: &mut Frame) {
    let long: LongType = frame.pop_operand().expect_long();
    frame.push_operand(Float(long as FloatType));
}

pub fn long_to_double(frame: &mut Frame) {
    let long: LongType = frame.pop_operand().expect_long();
    frame.push_operand(Double(long as DoubleType));
}

pub fn float_to_double(frame: &mut Frame) {
    let float: FloatType = frame.pop_operand().expect_float();
    frame.push_operand(Double(float as DoubleType));
}

pub fn int_to_byte(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Int(int & 0xff));
}

pub fn int_to_char(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Char(int as u8 as char));
}

pub fn int_to_short(frame: &mut Frame) {
    let int: IntType = frame.pop_operand().expect_int();
    frame.push_operand(Short(int as ShortType));
}

pub fn long_to_int(frame: &mut Frame) {
    let long: LongType = frame.pop_operand().expect_long();
    frame.push_operand(Int(long as IntType));
}

pub fn float_to_int(frame: &mut Frame) {
    let float: FloatType = frame.pop_operand().expect_float();
    frame.push_operand(Int(float as IntType));
}

pub fn float_to_long(frame: &mut Frame) {
    let float: FloatType = frame.pop_operand().expect_float();
    frame.push_operand(Long(float as LongType));
}

pub fn double_to_int(frame: &mut Frame) {
    let double: DoubleType = frame.pop_operand().expect_double();
    frame.push_operand(Int(double as IntType));
}

pub fn double_to_long(frame: &mut Frame) {
    let double: DoubleType = frame.pop_operand().expect_double();
    frame.push_operand(Long(double as LongType));
}

pub fn double_to_float(frame: &mut Frame) {
    let double: DoubleType = frame.pop_operand().expect_double();
    frame.push_operand(Float(double as FloatType));
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::*;
    use crate::vm::data_type::Value::*;

    #[test]
    fn i2l() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2l,
            final_stack: [Long(100)],
        );
    }

    #[test]
    fn i2f() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2f,
            final_stack: [Float(100.0)],
        );
    }

    #[test]
    fn i2d() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2d,
            final_stack: [Double(100.0)],
        );
    }

    #[test]
    fn l2f() {
        test_instruction!(
            start_stack: [Long(1337)],
            instruction: L2f,
            final_stack: [Float(1337.0)],
        );
    }

    #[test]
    fn l2d() {
        test_instruction!(
            start_stack: [Long(13372)],
            instruction: L2d,
            final_stack: [Double(13372.0)],
        );
    }

    #[test]
    fn f2d() {
        test_instruction!(
            start_stack: [Float(1000.0)],
            instruction: F2d,
            final_stack: [Double(1000.0)],
        );
    }

    #[test]
    fn i2b() {
        test_instruction!(
            start_stack: [Int(0x101)],
            instruction: I2b,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn i2c() {
        test_instruction!(
            start_stack: [Int(65)],
            instruction: I2c,
            final_stack: [Char('A')],
        );
    }

    #[test]
    fn i2s() {
        test_instruction!(
            start_stack: [Int(65)],
            instruction: I2s,
            final_stack: [Short(65)],
        );
    }

    #[test]
    fn l2i() {
        test_instruction!(
            start_stack: [Long(62)],
            instruction: L2i,
            final_stack: [Int(62)],
        );
    }

    #[test]
    fn f2i() {
        test_instruction!(
            start_stack: [Float(1234.58)],
            instruction: F2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2i() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2l() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2l,
            final_stack: [Long(1234)],
        );
    }

    #[test]
    fn d2f() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2f,
            final_stack: [Float(1234.58)],
        );
    }
}
