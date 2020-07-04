use crate::vm::{Frame, Value};
use crate::vm::Value::ReturnAddress;

pub fn if_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() == 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_not_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() != 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_less_than(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() < 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_less_than_inclusive(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() <= 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_greater_than(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() > 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_greater_than_inclusive(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    if frame.pop_operand_int() >= 0 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_null(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    match frame.pop_operand() {
        Value::Null => Some(offset(operands)),
        _ => None
    }
}

pub fn if_non_null(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    match frame.pop_operand() {
        Value::Null => None,
        _ => Some(offset(operands))
    }
}

pub fn if_int_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 == value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_int_not_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 != value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_int_less_than(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 < value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_int_less_than_inclusive(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 <= value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_int_greater_than(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 > value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_int_greater_than_inclusive(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_int();
    let value2 = frame.pop_operand_int();
    if value1 >= value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_reference_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_reference();
    let value2 = frame.pop_operand_reference();
    if value1 == value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn if_reference_not_equals(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let value1 = frame.pop_operand_reference();
    let value2 = frame.pop_operand_reference();
    if value1 != value2 {
        Some(offset(operands))
    } else {
        None
    }
}

pub fn goto(operands: &[u8]) -> Option<i32> {
    Some(offset(operands))
}

pub fn goto_wide(operands: &[u8]) -> Option<i32> {
    Some(offset_wide(operands))
}

pub fn jump_subroutine(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    frame.push_operand(ReturnAddress(frame.pc as i32 + 3));
    Some(offset(operands))
}

pub fn jump_subroutine_wide(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    frame.push_operand(ReturnAddress(frame.pc as i32 + 5));
    Some(offset_wide(operands))
}

pub fn return_from_subroutine(frame: &mut Frame, operands: &[u8]) -> Option<i32> {
    let index = operands[0] as u16;
    let offset = frame.get_local(index) as i32;
    Some(offset)
}


fn offset(bytes: &[u8]) -> i32 {
    (bytes[0] as i32) << 8 | bytes[1] as i32
}

fn offset_wide(bytes: &[u8]) -> i32 {
    (bytes[0] as i32) << 24 | (bytes[1] as i32) << 16 | (bytes[2] as i32) << 8 | bytes[3] as i32
}

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret_instruction;
    use crate::vm::Frame;
    use crate::vm::Value::*;

    #[test]
    fn ifeq_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifeq_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1)],
            command: IfEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifne_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1)],
            command: IfNe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifne_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfNe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn iflt_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(-1)],
            command: IfLt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn iflt_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfLt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifle_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfLe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifle_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1)],
            command: IfLe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifgt_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1)],
            command: IfGt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifgt_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifge_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0)],
            command: IfGe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifge_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(-1)],
            command: IfGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifnull_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Null],
            command: IfNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnull_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Reference(10)],
            command: IfNull; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifnonnull_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Reference(10)],
            command: IfNonNull; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifnonnull_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Null],
            command: IfNonNull; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpeq_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpeq_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(-1), Int(0)],
            command: IfIcmpEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpne_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0), Int(1)],
            command: IfIcmpNe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpne_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpNe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmplt_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(0)],
            command: IfIcmpLt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmplt_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpLt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmple_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpLe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmple_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(0), Int(1)],
            command: IfIcmpLe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpgt_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(2)],
            command: IfIcmpGt; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpgt_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpGt; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ificmpge_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(1), Int(1)],
            command: IfIcmpGe; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ificmpge_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Int(2), Int(1)],
            command: IfIcmpGe; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn ifacmpeq_success() {
        test_command!(
            start_pc: 4,
            start_stack: [Reference(1), Reference(1)],
            command: IfAcmpEq; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn ifacmpeq_fail() {
        test_command!(
            start_pc: 4,
            start_stack: [Reference(10), Reference(0)],
            command: IfAcmpEq; [0x00, 0x05],
            final_pc: 7,
        );
    }

    #[test]
    fn goto() {
        test_command!(
            start_pc: 4,
            command: Goto; [0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn goto_w() {
        test_command!(
            start_pc: 4,
            command: GotoW; [0x00, 0x00, 0x00, 0x05],
            final_pc: 9,
        );
    }

    #[test]
    fn jsr() {
        test_command!(
            start_pc: 4,
            command: Jsr; [0x00, 0x05],
            final_pc: 9,
            final_stack: [ReturnAddress(7)],
        );
    }

    #[test]
    fn jsr_w() {
        test_command!(
            start_pc: 4,
            command: JsrW; [0x00, 0x00, 0x00, 0x07],
            final_pc: 11,
            final_stack: [ReturnAddress(9)],
        );
    }

    #[test]
    fn ret() {
        test_command!(
            start_pc: 4,
            start_locals: {4 => 96},
            command: Ret; [0x04],
            final_pc: 100,
        );
    }
}
