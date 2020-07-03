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
    use crate::class::constant::ConstantPool;
    use crate::vm::Frame;
    use crate::vm::interpreter::interpret;
    use crate::class::code::Instruction;
    use crate::vm::Value::*;
    use crate::class::code::Opcode::*;

    #[test]
    fn conversion() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        // Widening conversions
        frame.push_operand(Int(100));
        interpret(&mut frame, &vec![Instruction::new(I2l, vec![])]);
        assert_eq!(frame.pop_operand(), Long(100));

        frame.push_operand(Int(100));
        interpret(&mut frame, &vec![Instruction::new(I2f, vec![])]);
        assert_eq!(frame.pop_operand(), Float(100.0));

        frame.push_operand(Int(100));
        interpret(&mut frame, &vec![Instruction::new(I2d, vec![])]);
        assert_eq!(frame.pop_operand(), Double(100.0));

        frame.push_operand(Long(1337));
        interpret(&mut frame, &vec![Instruction::new(L2f, vec![])]);
        assert_eq!(frame.pop_operand(), Float(1337.0));

        frame.push_operand(Long(13372));
        interpret(&mut frame, &vec![Instruction::new(L2d, vec![])]);
        assert_eq!(frame.pop_operand(), Double(13372.0));

        frame.push_operand(Float(1000.0));
        interpret(&mut frame, &vec![Instruction::new(F2d, vec![])]);
        assert_eq!(frame.pop_operand(), Double(1000.0));

        // Narrowing conversions
        frame.push_operand(Int(0x101));
        interpret(&mut frame, &vec![Instruction::new(I2b, vec![])]);
        assert_eq!(frame.pop_operand(), Byte(1));

        frame.push_operand(Int(65));
        interpret(&mut frame, &vec![Instruction::new(I2c, vec![])]);
        assert_eq!(frame.pop_operand(), Char('A'));

        frame.push_operand(Int(65));
        interpret(&mut frame, &vec![Instruction::new(I2s, vec![])]);
        assert_eq!(frame.pop_operand(), Short(65));

        frame.push_operand(Long(62));
        interpret(&mut frame, &vec![Instruction::new(L2i, vec![])]);
        assert_eq!(frame.pop_operand(), Int(62));

        frame.push_operand(Float(1234.58));
        interpret(&mut frame, &vec![Instruction::new(F2i, vec![])]);
        assert_eq!(frame.pop_operand(), Int(1234));

        frame.push_operand(Float(1234.58));
        interpret(&mut frame, &vec![Instruction::new(F2l, vec![])]);
        assert_eq!(frame.pop_operand(), Long(1234));

        frame.push_operand(Double(1234.58));
        interpret(&mut frame, &vec![Instruction::new(D2i, vec![])]);
        assert_eq!(frame.pop_operand(), Int(1234));

        frame.push_operand(Double(1234.58));
        interpret(&mut frame, &vec![Instruction::new(D2l, vec![])]);
        assert_eq!(frame.pop_operand(), Long(1234));

        frame.push_operand(Double(1234.58));
        interpret(&mut frame, &vec![Instruction::new(D2f, vec![])]);
        assert_eq!(frame.pop_operand(), Float(1234.58));
    }
}
