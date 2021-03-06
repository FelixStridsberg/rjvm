use crate::vm::frame::Frame;

pub fn pop_operand(frame: &mut Frame) {
    frame.pop_operand();
}

pub fn pop_operand_long(frame: &mut Frame) {
    if frame.pop_operand().get_category() == 1 {
        frame.pop_operand();
    }
}

pub fn duplicate_operand(frame: &mut Frame) {
    let value = frame.pop_operand();
    frame.push_operand(value.clone());
    frame.push_operand(value);
}

pub fn duplicate_operand_long(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    if value1.get_category() == 2 {
        frame.push_operand(value1.clone());
        frame.push_operand(value1);
    } else {
        let value2 = frame.pop_operand();
        frame.push_operand(value2.clone());
        frame.push_operand(value1.clone());
        frame.push_operand(value2);
        frame.push_operand(value1);
    }
}

pub fn duplicate_operand_back1(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    let value2 = frame.pop_operand();
    frame.push_operand(value1.clone());
    frame.push_operand(value2);
    frame.push_operand(value1);
}

pub fn duplicate_operand_long_back1(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    let value2 = frame.pop_operand();

    if value1.get_category() == 2 {
        frame.push_operand(value1.clone());
        frame.push_operand(value2);
        frame.push_operand(value1);
    } else {
        let value3 = frame.pop_operand();
        frame.push_operand(value2.clone());
        frame.push_operand(value1.clone());
        frame.push_operand(value3);
        frame.push_operand(value2);
        frame.push_operand(value1);
    }
}

pub fn duplicate_operand_back2(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    let value2 = frame.pop_operand();

    if value1.get_category() == 2 {
        frame.push_operand(value1.clone());
        frame.push_operand(value2);
        frame.push_operand(value1);
    } else {
        let value3 = frame.pop_operand();
        frame.push_operand(value1.clone());
        frame.push_operand(value3);
        frame.push_operand(value2);
        frame.push_operand(value1);
    }
}

pub fn duplicate_operand_long_back2(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    let value2 = frame.pop_operand();

    if value1.get_category() == 2 && value2.get_category() == 2 {
        frame.push_operand(value1.clone());
        frame.push_operand(value2);
        frame.push_operand(value1);
        return;
    }

    let value3 = frame.pop_operand();
    if value1.get_category() == 1 && value2.get_category() == 1 && value3.get_category() == 2 {
        frame.push_operand(value2.clone());
        frame.push_operand(value1.clone());
        frame.push_operand(value3);
        frame.push_operand(value2);
        frame.push_operand(value1);
        return;
    }

    if value1.get_category() == 2 && value2.get_category() == 1 && value3.get_category() == 1 {
        frame.push_operand(value1.clone());
        frame.push_operand(value3);
        frame.push_operand(value2);
        frame.push_operand(value1);
        return;
    }

    let value4 = frame.pop_operand();
    frame.push_operand(value2.clone());
    frame.push_operand(value1.clone());
    frame.push_operand(value4);
    frame.push_operand(value3);
    frame.push_operand(value2);
    frame.push_operand(value1);
}

pub fn swap_operand(frame: &mut Frame) {
    let value1 = frame.pop_operand();
    let value2 = frame.pop_operand();
    frame.push_operand(value1);
    frame.push_operand(value2);
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::*;
    use crate::vm::data_type::Value::*;

    #[test]
    fn pop() {
        test_instruction!(
            start_stack: [Int(32)],
            instruction: Pop,
            final_stack: [],
        );
    }

    #[test]
    fn pop2() {
        test_instruction!(
            start_stack: [Int(32), Int(32)],
            instruction: Pop2,
            final_stack: [],
        );
    }

    #[test]
    fn pop2_long() {
        test_instruction!(
            start_stack: [Double(32.0)],
            instruction: Pop2,
            final_stack: [],
        );
    }

    #[test]
    fn dup() {
        test_instruction!(
            start_stack: [Int(21)],
            instruction: Dup,
            final_stack: [Int(21), Int(21)],
        );
    }

    #[test]
    fn dup2() {
        test_instruction!(
            start_stack: [Long(21)],
            instruction: Dup2,
            final_stack: [Long(21), Long(21)],
        );
    }

    #[test]
    fn dup2_short() {
        test_instruction!(
            start_stack: [Int(2), Int(1)],
            instruction: Dup2,
            final_stack: [Int(2), Int(1), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup_x1() {
        test_instruction!(
            start_stack: [Int(2), Int(1)],
            instruction: DupX1,
            final_stack: [Int(1), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup2_x1() {
        test_instruction!(
            start_stack: [Int(3), Int(2), Int(1)],
            instruction: Dup2X1,
            final_stack: [Int(2), Int(1), Int(3), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup2_x1_long() {
        test_instruction!(
            start_stack: [Long(2), Long(1)],
            instruction: Dup2X1,
            final_stack: [Long(1), Long(2), Long(1)],
        );
    }

    #[test]
    fn dup_x2() {
        test_instruction!(
            start_stack: [Int(3), Int(2), Int(1)],
            instruction: DupX2,
            final_stack: [Int(1), Int(3), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup_x2_long() {
        test_instruction!(
            start_stack: [Long(2), Long(1)],
            instruction: DupX2,
            final_stack: [Long(1), Long(2), Long(1)],
        );
    }

    #[test]
    fn dup2_x2_short_short() {
        test_instruction!(
            start_stack: [Int(4), Int(3), Int(2), Int(1)],
            instruction: Dup2X2,
            final_stack: [Int(2), Int(1), Int(4), Int(3), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup2_x2_short_long() {
        test_instruction!(
            start_stack: [Int(3), Int(2), Long(1)],
            instruction: Dup2X2,
            final_stack: [Long(1), Int(3), Int(2), Long(1)],
        );
    }

    #[test]
    fn dup2_x2_long_short() {
        test_instruction!(
            start_stack: [Long(3), Int(2), Int(1)],
            instruction: Dup2X2,
            final_stack: [Int(2), Int(1), Long(3), Int(2), Int(1)],
        );
    }

    #[test]
    fn dup2_x2_long_long() {
        test_instruction!(
            start_stack: [Long(2), Long(1)],
            instruction: Dup2X2,
            final_stack: [Long(1), Long(2), Long(1)],
        );
    }

    #[test]
    fn swap() {
        test_instruction!(
            start_stack: [Long(2), Long(1)],
            instruction: Swap,
            final_stack: [Long(1), Long(2)],
        );
    }
}
