mod arithmetic;
mod conversion;
mod load_and_store;

use crate::class::code::Instruction;
use crate::class::code::Opcode::*;
use crate::vm::interpreter::arithmetic::*;
use crate::vm::interpreter::conversion::*;
use crate::vm::interpreter::load_and_store::*;
use crate::vm::Value::{Double, Float, Int, Long};
use crate::vm::{Frame, Value};

pub fn interpret(frame: &mut Frame, instructions: &Vec<Instruction>) -> Option<Value> {
    let mut ret = None;

    // TODO implement PC
    for i in instructions {
        ret = interpret_instruction(frame, i);
    }

    ret
}

fn interpret_instruction(frame: &mut Frame, instruction: &Instruction) -> Option<Value> {
    match &instruction.opcode {
        // Load and store:
        Iload => load_int(frame, &instruction.operands),
        Iload0 => load_int_n(frame, 0),
        Iload1 => load_int_n(frame, 1),
        Iload2 => load_int_n(frame, 2),
        Iload3 => load_int_n(frame, 3),

        Lload => load_long(frame, &instruction.operands),
        Lload0 => load_long_n(frame, 0),
        Lload1 => load_long_n(frame, 1),
        Lload2 => load_long_n(frame, 2),
        Lload3 => load_long_n(frame, 3),

        Fload => load_float(frame, &instruction.operands),
        Fload0 => load_float_n(frame, 0),
        Fload1 => load_float_n(frame, 1),
        Fload2 => load_float_n(frame, 2),
        Fload3 => load_float_n(frame, 3),

        Dload => load_double(frame, &instruction.operands),
        Dload0 => load_double_n(frame, 0),
        Dload1 => load_double_n(frame, 1),
        Dload2 => load_double_n(frame, 2),
        Dload3 => load_double_n(frame, 3),

        Aload => load_reference(frame, &instruction.operands),
        Aload0 => load_reference_n(frame, 0),
        Aload1 => load_reference_n(frame, 1),
        Aload2 => load_reference_n(frame, 2),
        Aload3 => load_reference_n(frame, 3),

        Istore => store_int(frame, &instruction.operands),
        Istore0 => store_int_n(frame, 0),
        Istore1 => store_int_n(frame, 1),
        Istore2 => store_int_n(frame, 2),
        Istore3 => store_int_n(frame, 3),

        Lstore => store_long(frame, &instruction.operands),
        Lstore0 => store_long_n(frame, 0),
        Lstore1 => store_long_n(frame, 1),
        Lstore2 => store_long_n(frame, 2),
        Lstore3 => store_long_n(frame, 3),

        Fstore => store_float(frame, &instruction.operands),
        Fstore0 => store_float_n(frame, 0),
        Fstore1 => store_float_n(frame, 1),
        Fstore2 => store_float_n(frame, 2),
        Fstore3 => store_float_n(frame, 3),

        Dstore => store_double(frame, &instruction.operands),
        Dstore0 => store_double_n(frame, 0),
        Dstore1 => store_double_n(frame, 1),
        Dstore2 => store_double_n(frame, 2),
        Dstore3 => store_double_n(frame, 3),

        Astore => store_reference(frame, &instruction.operands),
        Astore0 => store_reference_n(frame, 0),
        Astore1 => store_reference_n(frame, 1),
        Astore2 => store_reference_n(frame, 2),
        Astore3 => store_reference_n(frame, 3),

        Bipush => push_byte(frame, &instruction.operands),
        Sipush => push_short(frame, &instruction.operands),
        Ldc => push_constant(frame, &instruction.operands),
        LdcW => push_constant_wide(frame, &instruction.operands),
        Ldc2W => push_constant_long(frame, &instruction.operands),
        AconstNull => push_null(frame),

        IconstM1 => frame.push_operand(Int(-1)),
        Iconst0 => frame.push_operand(Int(0)),
        Iconst1 => frame.push_operand(Int(1)),
        Iconst2 => frame.push_operand(Int(2)),
        Iconst3 => frame.push_operand(Int(3)),
        Iconst4 => frame.push_operand(Int(4)),
        Iconst5 => frame.push_operand(Int(5)),

        Lconst0 => frame.push_operand(Long(0)),
        Lconst1 => frame.push_operand(Long(1)),

        Fconst0 => frame.push_operand(Float(0.0)),
        Fconst1 => frame.push_operand(Float(1.0)),
        Fconst2 => frame.push_operand(Float(2.0)),

        Dconst0 => frame.push_operand(Double(0.0)),
        Dconst1 => frame.push_operand(Double(1.0)),

        Wide => unimplemented!("wide not implemented."),

        // Arithmetic:
        Iadd => add_int(frame),
        Ladd => add_long(frame),
        Fadd => add_float(frame),
        Dadd => add_double(frame),

        Isub => sub_int(frame),
        Lsub => sub_long(frame),
        Fsub => sub_float(frame),
        Dsub => sub_double(frame),

        Imul => mul_int(frame),
        Lmul => mul_long(frame),
        Fmul => mul_float(frame),
        Dmul => mul_double(frame),

        Idiv => div_int(frame),
        Ldiv => div_long(frame),
        Fdiv => div_float(frame),
        Ddiv => div_double(frame),

        Irem => rem_int(frame),
        Lrem => rem_long(frame),
        Frem => rem_float(frame),
        Drem => rem_double(frame),

        Ineg => neg_int(frame),
        Lneg => neg_long(frame),
        Fneg => neg_float(frame),
        Dneg => neg_double(frame),

        Ishl => int_shift_left(frame),
        Ishr => int_shift_right(frame),
        Iushr => int_logical_shift_right(frame),

        Lshl => long_shift_left(frame),
        Lshr => long_shift_right(frame),
        Lushr => long_logical_shift_right(frame),

        Ior => int_bitwise_or(frame),
        Lor => long_bitwise_or(frame),

        Iand => int_bitwise_and(frame),
        Land => long_bitwise_and(frame),

        Ixor => int_bitwise_exclusive_or(frame),
        Lxor => long_bitwise_exclusive_or(frame),

        Iinc => int_increase(frame, &instruction.operands),

        Dcmpg => double_compare_g(frame),
        Dcmpl => double_compare_l(frame),

        Fcmpg => float_compare_g(frame),
        Fcmpl => float_compare_l(frame),

        Lcmp => long_compare(frame),

        // Conversion:
        I2l => int_to_long(frame),
        I2f => int_to_float(frame),
        I2d => int_to_double(frame),
        L2f => long_to_float(frame),
        F2d => long_to_double(frame),


        ///
        Ireturn => return Some(frame.pop_operand()),

        _ => unimplemented!(
            "Opcode {:?} is not implemented in interpreter",
            instruction.opcode
        ),
    }

    None
}

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::class::constant::Constant;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret;
    use crate::vm::Frame;
    use crate::vm::Value::{Double, Float, Int, Long, Reference, ReturnAddress, Short};

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

    #[test]
    fn conversion() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);

        frame.push_operand(Int(100));
        interpret(&mut frame, &vec![Instruction::new(I2l, vec![])]);

        assert_eq!(frame.pop_operand(), Long(100));
    }
}
