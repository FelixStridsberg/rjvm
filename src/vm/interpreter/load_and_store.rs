//! Contains implementation of all instructions under:
//! https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.2

use crate::class::constant::Constant;
use crate::error::Result;
use crate::vm::data_type::IntType;
use crate::vm::data_type::Value::*;
use crate::vm::frame::Frame;

#[macro_export]
macro_rules! load {
    ($frame:ident, $instruction:ident, $($type:pat)|+) => {{
        let index = $instruction.operands[0] as u16;
        load!($frame, $($type)|*, index);
    }};
    ($frame:ident, $($type:pat)|+, $index:expr) => {{
        let value = $frame.get_local($index);

        if !matches!(value, $($type)|*) {
            panic!(
                "*store_<n> expected a {} value on top of the stack. Got {:?}",
                stringify!($($type)|*), value);
        }

        $frame.push_operand(value);
    }};
}

#[macro_export]
macro_rules! store {
    ($frame:ident, $instruction:ident, $($type:pat)|+) => {{
        let index = $instruction.operands[0] as u16;
        store!($frame, $($type)|*, index);
    }};
    ($frame:ident, $($type:pat)|+, $index:expr) => {{
        let operand = $frame.pop_operand();

        if !matches!(operand, $($type)|*) {
            panic!(
                    "*store_<n> expected a {} value on top of the stack. Got {:?}",
                    stringify!($($type)|*), operand);
        }

        $frame.set_local($index, operand);
    }};
}

pub fn push_byte(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(Int(operands[0] as IntType));
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
        constant => return runtime_error!("ldc not implemented for constant {:?}", constant),
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
        constant => return runtime_error!("ldc2w not implemented for constant {:?}", constant),
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
            start_locals: { 5 => Int(6) },
            instruction: ILoad; [0x05],
            final_stack: [Int(6)],
        );
    }

    #[test]
    fn iload0() {
        test_instruction!(
            start_locals: { 0 => Int(1) },
            instruction: ILoad0,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn iload1() {
        test_instruction!(
            start_locals: { 1 => Int(2) },
            instruction: ILoad1,
            final_stack: [Int(2)],
        );
    }

    #[test]
    fn iload2() {
        test_instruction!(
            start_locals: { 2 => Int(3) },
            instruction: ILoad2,
            final_stack: [Int(3)],
        );
    }

    #[test]
    fn iload3() {
        test_instruction!(
            start_locals: { 3 => Int(4) },
            instruction: ILoad3,
            final_stack: [Int(4)],
        );
    }

    #[test]
    fn lload() {
        test_instruction!(
            start_locals_long: { 4 => Long(8) },
            instruction: LLoad; [0x04],
            final_stack: [Long(8)],
        );
    }

    #[test]
    fn lload0() {
        test_instruction!(
            start_locals_long: { 0 => Long(1) },
            instruction: LLoad0,
            final_stack: [Long(1)],
        );
    }

    #[test]
    fn lload1() {
        test_instruction!(
            start_locals_long: { 1 => Long(2) },
            instruction: LLoad1,
            final_stack: [Long(2)],
        );
    }

    #[test]
    fn lload2() {
        test_instruction!(
            start_locals_long: { 2 => Long(3) },
            instruction: LLoad2,
            final_stack: [Long(3)],
        );
    }

    #[test]
    fn lload3() {
        test_instruction!(
            start_locals_long: { 3 => Long(4) },
            instruction: LLoad3,
            final_stack: [Long(4)],
        );
    }

    #[test]
    fn fload() {
        test_instruction!(
            start_locals: { 5 => Float(6.8) },
            instruction: FLoad; [0x05],
            final_stack: [Float(6.8)],
        );
    }

    #[test]
    fn fload0() {
        test_instruction!(
            start_locals: { 0 => Float(1.2) },
            instruction: FLoad0,
            final_stack: [Float(1.2)],
        );
    }

    #[test]
    fn fload1() {
        test_instruction!(
            start_locals: { 1 => Float(2.3) },
            instruction: FLoad1,
            final_stack: [Float(2.3)],
        );
    }

    #[test]
    fn fload2() {
        test_instruction!(
            start_locals: { 2 => Float(3.4) },
            instruction: FLoad2,
            final_stack: [Float(3.4)],
        );
    }

    #[test]
    fn fload3() {
        test_instruction!(
            start_locals: { 3 => Float(4.5) },
            instruction: FLoad3,
            final_stack: [Float(4.5)],
        );
    }

    #[test]
    fn dload() {
        test_instruction!(
            start_locals_long: { 4 => Double(5.6) },
            instruction: DLoad; [0x04],
            final_stack: [Double(5.6)],
        );
    }

    #[test]
    fn dload0() {
        test_instruction!(
            start_locals_long: { 0 => Double(1.2) },
            instruction: DLoad0,
            final_stack: [Double(1.2)],
        );
    }

    #[test]
    fn dload1() {
        test_instruction!(
            start_locals_long: { 1 => Double(2.3) },
            instruction: DLoad1,
            final_stack: [Double(2.3)],
        );
    }

    #[test]
    fn dload2() {
        test_instruction!(
            start_locals_long: { 2 => Double(3.4) },
            instruction: DLoad2,
            final_stack: [Double(3.4)],
        );
    }

    #[test]
    fn dload3() {
        test_instruction!(
            start_locals_long: { 3 => Double(4.5) },
            instruction: DLoad3,
            final_stack: [Double(4.5)],
        );
    }

    #[test]
    fn aload() {
        test_instruction!(
            start_locals: { 5 => Reference(6) },
            instruction: ALoad; [0x05],
            final_stack: [Reference(6)],
        );
    }

    #[test]
    fn aload0() {
        test_instruction!(
            start_locals: { 0 => Reference(1) },
            instruction: ALoad0,
            final_stack: [Reference(1)],
        );
    }

    #[test]
    fn aload1() {
        test_instruction!(
            start_locals: { 1 => Reference(2) },
            instruction: ALoad1,
            final_stack: [Reference(2)],
        );
    }

    #[test]
    fn aload2() {
        test_instruction!(
            start_locals: { 2 => Reference(3) },
            instruction: ALoad2,
            final_stack: [Reference(3)],
        );
    }

    #[test]
    fn aload3() {
        test_instruction!(
            start_locals: { 3 => Reference(4) },
            instruction: ALoad3,
            final_stack: [Reference(4)],
        );
    }

    #[test]
    fn istore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Int(10)],
            instruction: IStore; [0x05],
            final_locals: { 5 => Int(10) },
        );
    }

    #[test]
    fn istore0() {
        test_instruction!(
            start_stack: [Int(1)],
            instruction: IStore0,
            final_locals: { 0 => Int(1) },
        );
    }

    #[test]
    fn istore1() {
        test_instruction!(
            start_stack: [Int(2)],
            instruction: IStore1,
            final_locals: { 1 => Int(2) },
        );
    }

    #[test]
    fn istore2() {
        test_instruction!(
            start_stack: [Int(3)],
            instruction: IStore2,
            final_locals: { 2 => Int(3) },
        );
    }

    #[test]
    fn istore3() {
        test_instruction!(
            start_stack: [Int(4)],
            instruction: IStore3,
            final_locals: { 3 => Int(4) },
        );
    }

    #[test]
    fn lstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Long(10)],
            instruction: LStore; [0x05],
            final_locals_long: { 5 => Long(10) },
        );
    }

    #[test]
    fn lstore0() {
        test_instruction!(
            start_stack: [Long(1)],
            instruction: LStore0,
            final_locals_long: { 0 => Long(1) },
        );
    }

    #[test]
    fn lstore1() {
        test_instruction!(
            start_stack: [Long(2)],
            instruction: LStore1,
            final_locals_long: { 1 => Long(2) },
        );
    }

    #[test]
    fn lstore2() {
        test_instruction!(
            start_stack: [Long(3)],
            instruction: LStore2,
            final_locals_long: { 2 => Long(3) },
        );
    }

    #[test]
    fn lstore3() {
        test_instruction!(
            start_stack: [Long(4)],
            instruction: LStore3,
            final_locals_long: { 3 => Long(4) },
        );
    }

    #[test]
    fn fstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Float(5.1)],
            instruction: FStore; [0x05],
            final_locals: { 5 => Float(5.1) },
        );
    }

    #[test]
    fn fstore0() {
        test_instruction!(
            start_stack: [Float(1.1)],
            instruction: FStore0,
            final_locals: { 0 => Float(1.1) },
        );
    }

    #[test]
    fn fstore1() {
        test_instruction!(
            start_stack: [Float(2.2)],
            instruction: FStore1,
            final_locals: { 1 => Float(2.2) },
        );
    }

    #[test]
    fn fstore2() {
        test_instruction!(
            start_stack: [Float(3.3)],
            instruction: FStore2,
            final_locals: { 2 => Float(3.3) },
        );
    }

    #[test]
    fn fstore3() {
        test_instruction!(
            start_stack: [Float(4.4)],
            instruction: FStore3,
            final_locals: { 3 => Float(4.4) },
        );
    }

    #[test]
    fn dstore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), Double(5.1)],
            instruction: DStore; [0x05],
            final_locals_long: { 5 => Double(5.1) },
        );
    }

    #[test]
    fn dstore0() {
        test_instruction!(
            start_stack: [Double(1.1)],
            instruction: DStore0,
            final_locals_long: { 0 => Double(1.1) },
        );
    }

    #[test]
    fn dstore1() {
        test_instruction!(
            start_stack: [Double(2.2)],
            instruction: DStore1,
            final_locals_long: { 1 => Double(2.2) },
        );
    }

    #[test]
    fn dstore2() {
        test_instruction!(
            start_stack: [Double(3.3)],
            instruction: DStore2,
            final_locals_long: { 2 => Double(3.3) },
        );
    }

    #[test]
    fn dstore3() {
        test_instruction!(
            start_stack: [Double(4.4)],
            instruction: DStore3,
            final_locals_long: { 3 => Double(4.4) },
        );
    }

    #[test]
    fn astore() {
        test_instruction!(
            start_stack: [Int(0), Int(0), Int(0), Int(0), Int(0), ReturnAddress(10)],
            instruction: AStore; [0x05],
            final_locals: { 5 => ReturnAddress(10) },
        );
    }

    #[test]
    fn astore0() {
        test_instruction!(
            start_stack: [Reference(1)],
            instruction: AStore0,
            final_locals: { 0 => Reference(1) },
        );
    }

    #[test]
    fn astore1() {
        test_instruction!(
            start_stack: [Reference(2)],
            instruction: AStore1,
            final_locals: { 1 => Reference(2) },
        );
    }

    #[test]
    fn astore2() {
        test_instruction!(
            start_stack: [Reference(3)],
            instruction: AStore2,
            final_locals: { 2 => Reference(3) },
        );
    }

    #[test]
    fn astore3() {
        test_instruction!(
            start_stack: [Reference(4)],
            instruction: AStore3,
            final_locals: { 3 => Reference(4) },
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
