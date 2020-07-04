use crate::vm::{Frame, Value};

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

fn offset(bytes: &[u8]) -> i32 {
    (bytes[0] as i32) << 8 | bytes[1] as i32
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
}
