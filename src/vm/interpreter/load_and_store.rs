//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.2

use crate::class::constant::Constant;
use crate::error::Result;
use crate::vm::data_type::Value::*;
use crate::vm::data_type::{DoubleType, FloatType, IntType, LongType, ReferenceType};
use crate::vm::frame::Frame;

pub fn load_int(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    load_int_n(frame, index);
}

pub fn load_int_n(frame: &mut Frame, index: u16) {
    let int = frame.get_local(index) as IntType;
    frame.push_operand(Int(int));
}

pub fn load_long(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    load_long_n(frame, index);
}

pub fn load_long_n(frame: &mut Frame, index: u16) {
    let i1 = frame.get_local(index) as LongType;
    let i2 = frame.get_local(index + 1) as LongType;
    frame.push_operand(Long(i1 << 32 | i2));
}

pub fn load_float(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    load_float_n(frame, index);
}

pub fn load_float_n(frame: &mut Frame, index: u16) {
    let bits = frame.get_local(index);
    frame.push_operand(Float(FloatType::from_bits(bits)));
}

pub fn load_double(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    load_double_n(frame, index);
}

pub fn load_double_n(frame: &mut Frame, index: u16) {
    let i1 = frame.get_local(index) as u64;
    let i2 = frame.get_local(index + 1) as u64;
    let bits = i1 << 32 | i2;
    frame.push_operand(Double(DoubleType::from_bits(bits)));
}

pub fn load_reference(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    load_reference_n(frame, index);
}

pub fn load_reference_n(frame: &mut Frame, index: u16) {
    let int = frame.get_local(index);
    frame.push_operand(Reference(int as ReferenceType));
}

pub fn store_int(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    store_int_n(frame, index)
}

pub fn store_int_n(frame: &mut Frame, index: u16) -> Result<()> {
    let operand = frame.pop_operand();
    if let Int(value) = operand {
        frame.set_local(index, value as u32);
        Ok(())
    } else {
        runtime_error!(
            "istore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_long(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    store_long_n(frame, index)
}

pub fn store_long_n(frame: &mut Frame, index: u16) -> Result<()> {
    let operand = frame.pop_operand();
    if let Long(value) = operand {
        frame.set_local_long(index, value as u64);
        Ok(())
    } else {
        runtime_error!(
            "lstore_<n> expected a long value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_float(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    store_float_n(frame, index)
}

pub fn store_float_n(frame: &mut Frame, index: u16) -> Result<()> {
    let operand = frame.pop_operand();
    if let Float(value) = operand {
        frame.set_local(index, value.to_bits());
        Ok(())
    } else {
        runtime_error!(
            "fstore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_double(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    store_double_n(frame, index)
}

pub fn store_double_n(frame: &mut Frame, index: u16) -> Result<()> {
    let operand = frame.pop_operand();
    if let Double(value) = operand {
        frame.set_local_long(index, value.to_bits());
        Ok(())
    } else {
        runtime_error!(
            "fstore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        );
    }
}

pub fn store_reference(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    store_reference_n(frame, index)
}

pub fn store_reference_n(frame: &mut Frame, index: u16) -> Result<()> {
    let value = match frame.pop_operand() {
        Reference(r) => r,
        ReturnAddress(r) => r,
        operand => runtime_error!(
            "astore_<n> expected an int value on top of the stack. Got {:?}",
            operand
        ),
    };
    frame.set_local(index, value as u32);
    Ok(())
}

pub fn push_byte(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(Int(operands[0] as i32));
}

pub fn push_short(frame: &mut Frame, operands: &[u8]) {
    let b1 = operands[0] as i16;
    let b2 = operands[1] as i16;
    frame.push_operand(Short((b1 << 8) | b2));
}

pub fn push_constant(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index = operands[0] as u16;
    push_constant_index(frame, index)
}

pub fn push_constant_wide(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index_b1 = operands[0] as u16;
    let index_b2 = operands[1] as u16;
    push_constant_index(frame, (index_b1 << 2) | index_b2)
}

fn push_constant_index(frame: &mut Frame, index: u16) -> Result<()> {
    let value = match frame.class.constants.get(index) {
        Constant::Integer(i) => Int(*i),
        Constant::Float(f) => Float(*f),
        // TODO reference and reference resolution
        constant => runtime_error!("ldc not implemented for constant {:?}", constant),
    };

    frame.push_operand(value);

    Ok(())
}

pub fn push_constant_long(frame: &mut Frame, operands: &[u8]) -> Result<()> {
    let index_b1 = operands[0] as u16;
    let index_b2 = operands[1] as u16;
    let index = (index_b1 << 2) | index_b2;

    let value = match frame.class.constants.get(index) {
        Constant::Long(l) => Long(*l),
        Constant::Double(d) => Double(*d),
        // TODO reference and reference resolution
        constant => runtime_error!("ldc2w not implemented for constant {:?}", constant),
    };

    frame.push_operand(value);

    Ok(())
}

pub fn push_null(frame: &mut Frame) {
    frame.push_operand(Null);
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::*;
    use crate::class::constant::Constant;
    use crate::vm::data_type::Value::*;

    #[test]
    fn iload() {
        test_instruction!(
            start_locals: { 5 => 6 },
            instruction: ILoad; [0x05],
            final_stack: [Int(6)],
        );
    }

    #[test]
    fn iload0() {
        test_instruction!(
            start_locals: { 0 => 1 },
            instruction: ILoad0,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn iload1() {
        test_instruction!(
            start_locals: { 1 => 2 },
            instruction: ILoad1,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn iload2() {
        test_instruction!(
            start_locals: { 2 => 3 },
            instruction: ILoad2,
            final_stack: [Int(3)],
        );
    }

    #[test]
    fn iload3() {
        test_instruction!(
            start_locals: { 3 => 4 },
            instruction: ILoad3,
            final_stack: [Int(4)],
        );
    }

    #[test]
    fn lload() {
        test_instruction!(
            start_locals_long: { 4 => 8 },
            instruction: LLoad; [0x04],
            final_stack: [Long(8)],
        );
    }

    #[test]
    fn lload0() {
        test_instruction!(
            start_locals_long: { 0 => 1 },
            instruction: LLoad0,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn lload1() {
        test_instruction!(
            start_locals_long: { 1 => 2 },
            instruction: LLoad1,
            final_stack: [Long(2)],
        );
    }

    #[test]
    fn lload2() {
        test_instruction!(
            start_locals_long: { 2 => 3 },
            instruction: LLoad2,
            final_stack: [Long(3)],
        );
    }

    #[test]
    fn lload3() {
        test_instruction!(
            start_locals_long: { 3 => 4 },
            instruction: LLoad3,
            final_stack: [Long(4)],
        );
    }

    #[test]
    fn fload() {
        test_instruction!(
            start_locals: { 5 => 6.8_f32.to_bits() },
            instruction: FLoad; [0x05],
            final_stack: [Float(6.8)],
        );
    }

    #[test]
    fn fload0() {
        test_instruction!(
            start_locals: { 0 => 1.2_f32.to_bits() },
            instruction: FLoad0,
            final_stack: [Float(1.2)],
        );
    }

    #[test]
    fn fload1() {
        test_instruction!(
            start_locals: { 1 => 2.3_f32.to_bits() },
            instruction: FLoad1,
            final_stack: [Float(2.3)],
        );
    }

    #[test]
    fn fload2() {
        test_instruction!(
            start_locals: { 2 => 3.4_f32.to_bits() },
            instruction: FLoad2,
            final_stack: [Float(3.4)],
        );
    }

    #[test]
    fn fload3() {
        test_instruction!(
            start_locals: { 3 => 4.5_f32.to_bits() },
            instruction: FLoad3,
            final_stack: [Float(4.5)],
        );
    }

    #[test]
    fn dload() {
        test_instruction!(
            start_locals_long: { 4 => 5.6_f64.to_bits() },
            instruction: DLoad; [0x04],
            final_stack: [Double(5.6)],
        );
    }

    #[test]
    fn dload0() {
        test_instruction!(
            start_locals_long: { 0 => 1.2_f64.to_bits() },
            instruction: DLoad0,
            final_stack: [Double(1.2)],
        );
    }

    #[test]
    fn dload1() {
        test_instruction!(
            start_locals_long: { 1 => 2.3_f64.to_bits() },
            instruction: DLoad1,
            final_stack: [Double(2.3)],
        );
    }

    #[test]
    fn dload2() {
        test_instruction!(
            start_locals_long: { 2 => 3.4_f64.to_bits() },
            instruction: DLoad2,
            final_stack: [Double(3.4)],
        );
    }

    #[test]
    fn dload3() {
        test_instruction!(
            start_locals_long: { 3 => 4.5_f64.to_bits() },
            instruction: DLoad3,
            final_stack: [Double(4.5)],
        );
    }

    #[test]
    fn aload() {
        test_instruction!(
            start_locals: { 5 => 6 },
            instruction: ALoad; [0x05],
            final_stack: [Reference(6)],
        );
    }

    #[test]
    fn aload0() {
        test_instruction!(
            start_locals: { 0 => 1 },
            instruction: ALoad0,
            final_stack: [Reference(1)],
        );
    }

    #[test]
    fn aload1() {
        test_instruction!(
            start_locals: { 1 => 2 },
            instruction: ALoad1,
            final_stack: [Reference(2)],
        );
    }

    #[test]
    fn aload2() {
        test_instruction!(
            start_locals: { 2 => 3 },
            instruction: ALoad2,
            final_stack: [Reference(3)],
        );
    }

    #[test]
    fn aload3() {
        test_instruction!(
            start_locals: { 3 => 4 },
            instruction: ALoad3,
            final_stack: [Reference(4)],
        );
    }

    #[test]
    fn istore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Int(10)],
            instruction: IStore; [0x05],
            final_locals: { 5 => 10 },
        );
    }

    #[test]
    fn istore0() {
        test_instruction!(
            start_stack: [Int(1)],
            instruction: IStore0,
            final_locals: { 0 => 1 },
        );
    }

    #[test]
    fn istore1() {
        test_instruction!(
            start_stack: [Int(2)],
            instruction: IStore1,
            final_locals: { 1 => 2 },
        );
    }

    #[test]
    fn istore2() {
        test_instruction!(
            start_stack: [Int(3)],
            instruction: IStore2,
            final_locals: { 2 => 3 },
        );
    }

    #[test]
    fn istore3() {
        test_instruction!(
            start_stack: [Int(4)],
            instruction: IStore3,
            final_locals: { 3 => 4 },
        );
    }

    #[test]
    fn lstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Long(10)],
            instruction: LStore; [0x05],
            final_locals_long: { 5 => 10 },
        );
    }

    #[test]
    fn lstore0() {
        test_instruction!(
            start_stack: [Long(1)],
            instruction: LStore0,
            final_locals_long: { 0 => 1 },
        );
    }

    #[test]
    fn lstore1() {
        test_instruction!(
            start_stack: [Long(2)],
            instruction: LStore1,
            final_locals_long: { 1 => 2 },
        );
    }

    #[test]
    fn lstore2() {
        test_instruction!(
            start_stack: [Long(3)],
            instruction: LStore2,
            final_locals_long: { 2 => 3 },
        );
    }

    #[test]
    fn lstore3() {
        test_instruction!(
            start_stack: [Long(4)],
            instruction: LStore3,
            final_locals_long: { 3 => 4 },
        );
    }

    #[test]
    fn fstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Float(5.1)],
            instruction: FStore; [0x05],
            final_locals: { 5 => 5.1_f32.to_bits() },
        );
    }

    #[test]
    fn fstore0() {
        test_instruction!(
            start_stack: [Float(1.1)],
            instruction: FStore0,
            final_locals: { 0 => 1.1_f32.to_bits() },
        );
    }

    #[test]
    fn fstore1() {
        test_instruction!(
            start_stack: [Float(2.2)],
            instruction: FStore1,
            final_locals: { 1 => 2.2_f32.to_bits() },
        );
    }

    #[test]
    fn fstore2() {
        test_instruction!(
            start_stack: [Float(3.3)],
            instruction: FStore2,
            final_locals: { 2 => 3.3_f32.to_bits() },
        );
    }

    #[test]
    fn fstore3() {
        test_instruction!(
            start_stack: [Float(4.4)],
            instruction: FStore3,
            final_locals: { 3 => 4.4_f32.to_bits() },
        );
    }

    #[test]
    fn dstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Double(5.1)],
            instruction: DStore; [0x05],
            final_locals_long: { 5 => 5.1_f64.to_bits() },
        );
    }

    #[test]
    fn dstore0() {
        test_instruction!(
            start_stack: [Double(1.1)],
            instruction: DStore0,
            final_locals_long: { 0 => 1.1_f64.to_bits() },
        );
    }

    #[test]
    fn dstore1() {
        test_instruction!(
            start_stack: [Double(2.2)],
            instruction: DStore1,
            final_locals_long: { 1 => 2.2_f64.to_bits() },
        );
    }

    #[test]
    fn dstore2() {
        test_instruction!(
            start_stack: [Double(3.3)],
            instruction: DStore2,
            final_locals_long: { 2 => 3.3_f64.to_bits() },
        );
    }

    #[test]
    fn dstore3() {
        test_instruction!(
            start_stack: [Double(4.4)],
            instruction: DStore3,
            final_locals_long: { 3 => 4.4_f64.to_bits() },
        );
    }

    #[test]
    fn astore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), ReturnAddress(10)],
            instruction: AStore; [0x05],
            final_locals: { 5 => 10 },
        );
    }

    #[test]
    fn astore0() {
        test_instruction!(
            start_stack: [Reference(1)],
            instruction: AStore0,
            final_locals: { 0 => 1 },
        );
    }

    #[test]
    fn astore1() {
        test_instruction!(
            start_stack: [Reference(2)],
            instruction: AStore1,
            final_locals: { 1 => 2 },
        );
    }

    #[test]
    fn astore2() {
        test_instruction!(
            start_stack: [Reference(3)],
            instruction: AStore2,
            final_locals: { 2 => 3 },
        );
    }

    #[test]
    fn astore3() {
        test_instruction!(
            start_stack: [Reference(4)],
            instruction: AStore3,
            final_locals: { 3 => 4 },
        );
    }

    #[test]
    fn bipush() {
        test_instruction!(
            instruction: BiPush; [0x05],
            final_stack: [Int(5)],
        );
    }

    #[test]
    fn sipush() {
        test_instruction!(
            instruction: SiPush; [0x01, 0x10],
            final_stack: [Short(272)],
        );
    }

    #[test]
    fn ldc_int() {
        test_instruction!(
            constants: [Constant::Integer(10)],
            instruction: Ldc; [0x01],
            final_stack: [Int(10)],
        );
    }

    #[test]
    fn ldc_float() {
        test_instruction!(
            constants: [Constant::Float(10.1)],
            instruction: Ldc; [0x01],
            final_stack: [Float(10.1)],
        );
    }

    #[test]
    fn ldc_w_int() {
        test_instruction!(
            constants: [Constant::Integer(10)],
            instruction: LdcW; [0x00, 0x01],
            final_stack: [Int(10)],
        );
    }

    #[test]
    fn ldc_w_float() {
        test_instruction!(
            constants: [Constant::Float(10.1)],
            instruction: LdcW; [0x00, 0x01],
            final_stack: [Float(10.1)],
        );
    }

    #[test]
    fn ldc2_w_long() {
        test_instruction!(
            constants: [Constant::Long(10)],
            instruction: Ldc2W; [0x00, 0x01],
            final_stack: [Long(10)],
        );
    }

    #[test]
    fn ldc2_w_double() {
        test_instruction!(
            constants: [Constant::Double(10.1)],
            instruction: Ldc2W; [0x00, 0x01],
            final_stack: [Double(10.1)],
        );
    }
}
