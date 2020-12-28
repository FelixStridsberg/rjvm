use crate::vm::data_type::ReturnAddressType;
use crate::vm::data_type::Value;
use crate::vm::data_type::Value::ReturnAddress;
use crate::vm::frame::Frame;

pub fn if_equals(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() == 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_not_equals(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() != 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_less_than(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() < 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_less_than_inclusive(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() <= 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_greater_than(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() > 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_greater_than_inclusive(frame: &mut Frame, operands: &[u8]) {
    if frame.pop_operand().expect_int() >= 0 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_null(frame: &mut Frame, operands: &[u8]) {
    if matches!(frame.pop_operand(), Value::Null) {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_non_null(frame: &mut Frame, operands: &[u8]) {
    if !matches!(frame.pop_operand(), Value::Null) {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_equals(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 == value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_not_equals(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 != value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_less_than(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 < value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_less_than_inclusive(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 <= value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_greater_than(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 > value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_int_greater_than_inclusive(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_int();
    let value2 = frame.pop_operand().expect_int();
    if value1 >= value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_reference_equals(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_reference();
    let value2 = frame.pop_operand().expect_reference();
    if value1 == value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn if_reference_not_equals(frame: &mut Frame, operands: &[u8]) {
    let value1 = frame.pop_operand().expect_reference();
    let value2 = frame.pop_operand().expect_reference();
    if value1 != value2 {
        frame.pc_offset(offset(operands));
    }
}

pub fn goto(frame: &mut Frame, operands: &[u8]) {
    frame.pc_offset(offset(operands));
}

pub fn goto_wide(frame: &mut Frame, operands: &[u8]) {
    frame.pc_offset_wide(offset_wide(operands));
}

pub fn jump_subroutine(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(ReturnAddress(frame.pc as ReturnAddressType));
    frame.pc_offset(offset(operands));
}

pub fn jump_subroutine_wide(frame: &mut Frame, operands: &[u8]) {
    frame.push_operand(ReturnAddress(frame.pc as ReturnAddressType));
    frame.pc_offset_wide(offset_wide(operands));
}

pub fn return_from_subroutine(frame: &mut Frame, operands: &[u8]) {
    let index = operands[0] as u16;
    let offset = frame.get_local(index) as i16;
    frame.pc_offset(offset);
}

fn offset(bytes: &[u8]) -> i16 {
    (bytes[0] as i16) << 8 | bytes[1] as i16
}

fn offset_wide(bytes: &[u8]) -> i32 {
    (bytes[0] as i32) << 24 | (bytes[1] as i32) << 16 | (bytes[2] as i32) << 8 | bytes[3] as i32
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
        );
    }

    #[test]
    fn ifnull_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Null],
            instruction: IfNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnull_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(10)],
            instruction: IfNull; [0x00, 0x05],
            final_pc: 4,
        );
    }

    #[test]
    fn ifnonnull_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(10)],
            instruction: IfNonNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnonnull_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Null],
            instruction: IfNonNull; [0x00, 0x05],
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
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
            final_pc: 4,
        );
    }

    #[test]
    fn ifacmpeq_success() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(1), Reference(1)],
            instruction: IfAcmpEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifacmpeq_fail() {
        test_instruction!(
            start_pc: 4,
            start_stack: [Reference(10), Reference(0)],
            instruction: IfAcmpEq; [0x00, 0x05],
            final_pc: 4,
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
            start_locals: {4 => 96},
            instruction: Ret; [0x04],
            final_pc: 100,
        );
    }
}
