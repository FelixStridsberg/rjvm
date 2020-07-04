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

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret;
    use crate::vm::Frame;
    use crate::vm::Value::*;

    #[test]
    fn i2l() {
        test_command!(
            start_stack: [Int(100)],
            command: I2l,
            final_stack: [Long(100)],
        );
    }

    #[test]
    fn i2f() {
        test_command!(
            start_stack: [Int(100)],
            command: I2f,
            final_stack: [Float(100.0)],
        );
    }

    #[test]
    fn i2d() {
        test_command!(
            start_stack: [Int(100)],
            command: I2d,
            final_stack: [Double(100.0)],
        );
    }

    #[test]
    fn l2f() {
        test_command!(
            start_stack: [Long(1337)],
            command: L2f,
            final_stack: [Float(1337.0)],
        );
    }

    #[test]
    fn l2d() {
        test_command!(
            start_stack: [Long(13372)],
            command: L2d,
            final_stack: [Double(13372.0)],
        );
    }

    #[test]
    fn f2d() {
        test_command!(
            start_stack: [Float(1000.0)],
            command: F2d,
            final_stack: [Double(1000.0)],
        );
    }

    #[test]
    fn i2b() {
        test_command!(
            start_stack: [Int(0x101)],
            command: I2b,
            final_stack: [Byte(1)],
        );
    }

    #[test]
    fn i2c() {
        test_command!(
            start_stack: [Int(65)],
            command: I2c,
            final_stack: [Char('A')],
        );
    }

    #[test]
    fn i2s() {
        test_command!(
            start_stack: [Int(65)],
            command: I2s,
            final_stack: [Short(65)],
        );
    }

    #[test]
    fn l2i() {
        test_command!(
            start_stack: [Long(62)],
            command: L2i,
            final_stack: [Int(62)],
        );
    }

    #[test]
    fn f2i() {
        test_command!(
            start_stack: [Float(1234.58)],
            command: F2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2i() {
        test_command!(
            start_stack: [Double(1234.58)],
            command: D2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2l() {
        test_command!(
            start_stack: [Double(1234.58)],
            command: D2l,
            final_stack: [Long(1234)],
        );
    }

    #[test]
    fn d2f() {
        test_command!(
            start_stack: [Double(1234.58)],
            command: D2f,
            final_stack: [Float(1234.58)],
        );
    }
}
