use crate::binary::*;
use crate::vm::data_type::ReturnAddressType;
use crate::vm::data_type::Value::{Reference, ReturnAddress};
use crate::vm::frame::Frame;

#[macro_export]
macro_rules! if_cmp_zero (
    ($frame:ident, $instruction:ident, $op:tt) => {{
        if $frame.pop_operand().expect_int() $op 0 {
            $frame.pc_offset(crate::binary::bytes_to_i16(&$instruction.operands));
        } else {
            $frame.pc_next();
        }
    }};
    ($frame:ident, $instruction:ident, $comp:path) => {{
        if matches!($frame.pop_operand(), $comp) {
            $frame.pc_offset(crate::binary::bytes_to_i16(&$instruction.operands));
        } else {
            $frame.pc_next();
        }
    }}
);

#[macro_export]
macro_rules! if_cmp_operands (
    ($frame:ident, $instruction:ident, $type:path, $op:tt) => {{
        let value1 = expect_type!($frame.pop_operand(), $type);
        let value2 = expect_type!($frame.pop_operand(), $type);
        if value1 $op value2 {
            $frame.pc_offset(crate::binary::bytes_to_i16(&$instruction.operands));
        } else {
            $frame.pc_next();
        }
    }}
);

pub fn if_null(frame: &mut Frame, operands: &[u8]) {
    if matches!(frame.pop_operand(), Reference(None)) {
        frame.pc_offset(bytes_to_i16(operands));
    } else {
        frame.pc_next();
    }
}

pub fn if_non_null(frame: &mut Frame, operands: &[u8]) {
    if !matches!(frame.pop_operand(), Reference(None)) {
        frame.pc_offset(bytes_to_i16(operands));
    } else {
        frame.pc_next();
    }
}

pub fn table_switch(frame: &mut Frame, operands: &[u8]) {
    let index = frame.pop_operand().expect_int();
    let default_offset = bytes_to_i32(&operands[0..4]);
    let low = bytes_to_i32(&operands[4..8]);
    let high = bytes_to_i32(&operands[8..12]);

    if index < low || index > high {
        frame.pc_offset_wide(default_offset);
        return;
    }

    let offset_index = ((index - low) * 4 + 12) as usize;
    let offset = bytes_to_i32(&operands[offset_index..(offset_index + 8)]);

    frame.pc_offset_wide(offset);
}

pub fn lookup_switch(frame: &mut Frame, operands: &[u8]) {
    let key = frame.pop_operand().expect_int();
    let mut offset = bytes_to_i32(&operands[0..4]);
    let len = bytes_to_i32(&operands[4..8]);

    for i in 0..len {
        let o = (8 + i * 8) as usize;
        let lookup_match = bytes_to_i32(&operands[o..(o + 4)]);
        let lookup_offset = bytes_to_i32(&operands[(o + 4)..(o + 8)]);

        if key == lookup_match {
            offset = lookup_offset;
            break;
        }
    }

    frame.pc_offset_wide(offset);
}

pub fn goto(frame: &mut Frame, operands: &[u8]) {
    frame.pc_offset(bytes_to_i16(operands));
}

pub fn goto_wide(frame: &mut Frame, operands: &[u8]) {
    frame.pc_offset_wide(bytes_to_i32(operands));
}

pub fn jump_subroutine(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(ReturnAddress(frame.pc as ReturnAddressType));
    frame.pc_offset(bytes_to_i16(operands));
}

pub fn jump_subroutine_wide(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(ReturnAddress(frame.pc as ReturnAddressType));
    frame.pc_offset_wide(bytes_to_i32(operands));
}

pub fn return_from_subroutine(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    let offset = frame.get_local(index).expect_return_address();
    frame.pc_offset(offset as i16);
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::*;
    use crate::vm::data_type::Value::*;

    #[test]
    fn ifeq_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifeq_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1)],
            instruction: IfEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifne_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1)],
            instruction: IfNe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifne_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfNe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn iflt_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(-1)],
            instruction: IfLt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn iflt_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfLt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifle_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfLe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifle_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1)],
            instruction: IfLe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifgt_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1)],
            instruction: IfGt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifgt_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifge_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0)],
            instruction: IfGe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifge_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(-1)],
            instruction: IfGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifnull_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(None)],
            instruction: IfNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnull_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(Some(10))],
            instruction: IfNull; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifnonnull_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(Some(10))],
            instruction: IfNonNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnonnull_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(None)],
            instruction: IfNonNull; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpeq_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpeq_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(-1), Int(0)],
            instruction: IfIcmpEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpne_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0), Int(1)],
            instruction: IfIcmpNe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpne_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpNe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmplt_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(0)],
            instruction: IfIcmpLt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmplt_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpLt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmple_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpLe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmple_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(0), Int(1)],
            instruction: IfIcmpLe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpgt_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(2)],
            instruction: IfIcmpGt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpgt_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpge_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            instruction: IfIcmpGe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpge_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Int(2), Int(1)],
            instruction: IfIcmpGe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifacmpeq_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(Some(1)), Reference(Some(1))],
            instruction: IfAcmpEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifacmpeq_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(Some(10)), Reference(Some(0))],
            instruction: IfAcmpEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn goto() {
        test_instruction!(
            start_pc: 4,
            instruction: Goto; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn goto_w() {
        test_instruction!(
            start_pc: 4,
            instruction: GotoW; [0x00, 0x00, 0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn jsr() {
        test_instruction!(
            start_pc: 4,
            instruction: Jsr; [0x00, 0x05],
            final_pc: 9,
            final_stack: [ReturnAddress(4)],
        );
    }

    #[test]
    fn jsr_w() {
        test_instruction!(
            start_pc: 4,
            instruction: JsrW; [0x00, 0x00, 0x00, 0x07],
            final_pc: 11,
            final_stack: [ReturnAddress(4)],
        );
    }

    #[test]
    fn ret() {
        test_instruction!(
            start_pc: 4,
            start_locals: {4 => ReturnAddress(96)},
            instruction: Ret; [0x04],
            final_pc: 100,
        );
    }
}
