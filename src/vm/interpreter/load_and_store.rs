//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.2

use crate::class::constant::Constant;
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

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::class::constant::Constant;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret;
    use crate::vm::Frame;
    use crate::vm::Value::*;

    #[test]
    fn iload() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local(0, 1);
        frame.set_local(1, 2);
        frame.set_local(2, 3);
        frame.set_local(3, 4);
        frame.set_local(5, 5);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Iload0, vec![]),
                Instruction::new(Iload1, vec![]),
                Instruction::new(Iload2, vec![]),
                Instruction::new(Iload3, vec![]),
                Instruction::new(Iload, vec![0x05]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![Int(1), Int(2), Int(3), Int(4), Int(5)]
        );
    }

    #[test]
    fn lload() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local_long(0, 1);
        frame.set_local_long(2, 2);
        frame.set_local_long(8, 9);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Lload0, vec![]),
                Instruction::new(Lload2, vec![]),
                Instruction::new(Lload, vec![0x08]),
            ],
        );

        frame.set_local_long(1, 4);
        frame.set_local_long(3, 5);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Lload1, vec![]),
                Instruction::new(Lload3, vec![]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![Long(1), Long(2), Long(9), Long(4), Long(5)]
        );
    }

    #[test]
    fn fload() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local(0, 1.2_f32.to_bits());
        frame.set_local(1, 2.3_f32.to_bits());
        frame.set_local(2, 3.4_f32.to_bits());
        frame.set_local(3, 4.5_f32.to_bits());
        frame.set_local(4, 5.6_f32.to_bits());

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Fload0, vec![]),
                Instruction::new(Fload1, vec![]),
                Instruction::new(Fload2, vec![]),
                Instruction::new(Fload3, vec![]),
                Instruction::new(Fload, vec![0x04]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![Float(1.2), Float(2.3), Float(3.4), Float(4.5), Float(5.6)]
        );
    }

    #[test]
    fn dload() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local_long(0, 1.2_f64.to_bits());
        frame.set_local_long(2, 2.3_f64.to_bits());
        frame.set_local_long(8, 5.6_f64.to_bits());

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Dload0, vec![]),
                Instruction::new(Dload2, vec![]),
                Instruction::new(Dload, vec![0x08]),
            ],
        );

        frame.set_local_long(1, 1.1_f64.to_bits());
        frame.set_local_long(3, 4.4_f64.to_bits());

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Dload1, vec![]),
                Instruction::new(Dload3, vec![]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![
                Double(1.2),
                Double(2.3),
                Double(5.6),
                Double(1.1),
                Double(4.4)
            ]
        );
    }

    #[test]
    fn aload() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.set_local(0, 1);
        frame.set_local(1, 2);
        frame.set_local(2, 3);
        frame.set_local(3, 4);
        frame.set_local(7, 9);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Aload0, vec![]),
                Instruction::new(Aload1, vec![]),
                Instruction::new(Aload2, vec![]),
                Instruction::new(Aload3, vec![]),
                Instruction::new(Aload, vec![0x07]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![
                Reference(1),
                Reference(2),
                Reference(3),
                Reference(4),
                Reference(9)
            ]
        );
    }

    #[test]
    fn istore() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.push_operand(Int(5));
        frame.push_operand(Int(4));
        frame.push_operand(Int(3));
        frame.push_operand(Int(2));
        frame.push_operand(Int(1));

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Istore0, vec![]),
                Instruction::new(Istore1, vec![]),
                Instruction::new(Istore2, vec![]),
                Instruction::new(Istore3, vec![]),
                Instruction::new(Istore, vec![0x05]),
            ],
        );

        assert_eq!(frame.get_local(0), 1);
        assert_eq!(frame.get_local(1), 2);
        assert_eq!(frame.get_local(2), 3);
        assert_eq!(frame.get_local(3), 4);
        assert_eq!(frame.get_local(5), 5);
    }

    #[test]
    fn lstore() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.push_operand(Long(5));
        frame.push_operand(Long(4));
        frame.push_operand(Long(3));
        frame.push_operand(Long(2));
        frame.push_operand(Long(1));

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Lstore0, vec![]),
                Instruction::new(Lstore2, vec![]),
                Instruction::new(Lstore, vec![0x05]),
            ],
        );

        assert_eq!(frame.get_local_long(0), 1);
        assert_eq!(frame.get_local_long(2), 2);
        assert_eq!(frame.get_local_long(5), 3);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Lstore1, vec![]),
                Instruction::new(Lstore3, vec![]),
            ],
        );

        assert_eq!(frame.get_local_long(1), 4);
        assert_eq!(frame.get_local_long(3), 5);
    }

    #[test]
    fn fstore() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.push_operand(Float(5.1));
        frame.push_operand(Float(4.1));
        frame.push_operand(Float(3.1));
        frame.push_operand(Float(2.1));
        frame.push_operand(Float(1.1));

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Fstore0, vec![]),
                Instruction::new(Fstore1, vec![]),
                Instruction::new(Fstore2, vec![]),
                Instruction::new(Fstore3, vec![]),
                Instruction::new(Fstore, vec![0x06]),
            ],
        );

        println!("F: {:?}", frame.local_variables);

        assert_eq!(f32::from_bits(frame.get_local(0)), 1.1);
        assert_eq!(f32::from_bits(frame.get_local(1)), 2.1);
        assert_eq!(f32::from_bits(frame.get_local(2)), 3.1);
        assert_eq!(f32::from_bits(frame.get_local(3)), 4.1);
        assert_eq!(f32::from_bits(frame.get_local(6)), 5.1);
    }

    #[test]
    fn dstore() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.push_operand(Double(5.2));
        frame.push_operand(Double(4.2));
        frame.push_operand(Double(3.2));
        frame.push_operand(Double(2.2));
        frame.push_operand(Double(1.2));

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Dstore0, vec![]),
                Instruction::new(Dstore2, vec![]),
                Instruction::new(Dstore, vec![0x05]),
            ],
        );

        assert_eq!(f64::from_bits(frame.get_local_long(0)), 1.2);
        assert_eq!(f64::from_bits(frame.get_local_long(2)), 2.2);
        assert_eq!(f64::from_bits(frame.get_local_long(5)), 3.2);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Dstore1, vec![]),
                Instruction::new(Dstore3, vec![]),
            ],
        );

        assert_eq!(f64::from_bits(frame.get_local_long(1)), 4.2);
        assert_eq!(f64::from_bits(frame.get_local_long(3)), 5.2);
    }

    #[test]
    fn astore() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);
        frame.push_operand(ReturnAddress(5));
        frame.push_operand(Reference(4));
        frame.push_operand(Reference(3));
        frame.push_operand(Reference(2));
        frame.push_operand(Reference(1));

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Astore0, vec![]),
                Instruction::new(Astore1, vec![]),
                Instruction::new(Astore2, vec![]),
                Instruction::new(Astore3, vec![]),
                Instruction::new(Astore, vec![0x05]),
            ],
        );

        assert_eq!(frame.get_local(0), 1);
        assert_eq!(frame.get_local(1), 2);
        assert_eq!(frame.get_local(2), 3);
        assert_eq!(frame.get_local(3), 4);
        assert_eq!(frame.get_local(5), 5);
    }

    #[test]
    fn ipush() {
        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(10, 10, &constants);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Bipush, vec![0x05]),
                Instruction::new(Sipush, vec![0x01, 0x10]),
            ],
        );

        assert_eq!(frame.operand_stack, vec![Int(5), Short(272)],)
    }

    #[test]
    fn ldc() {
        let mut constants = ConstantPool::new(2);
        constants.add(Constant::Integer(10));
        constants.add(Constant::Float(14.4));
        constants.add(Constant::Integer(12));
        constants.add(Constant::Float(14.2));
        constants.add(Constant::Long(12));
        constants.add(Constant::Double(47.42));

        let mut frame = Frame::new(10, 10, &constants);

        interpret(
            &mut frame,
            &vec![
                Instruction::new(Ldc, vec![0x01]),
                Instruction::new(Ldc, vec![0x02]),
                Instruction::new(LdcW, vec![0x00, 0x03]),
                Instruction::new(LdcW, vec![0x00, 0x04]),
                Instruction::new(Ldc2W, vec![0x00, 0x05]),
                Instruction::new(Ldc2W, vec![0x00, 0x07]),
            ],
        );

        assert_eq!(
            frame.operand_stack,
            vec![
                Int(10),
                Float(14.4),
                Int(12),
                Float(14.2),
                Long(12),
                Double(47.42)
            ],
        )
    }
}
