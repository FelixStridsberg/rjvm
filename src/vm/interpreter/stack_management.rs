use crate::vm::Frame;

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
    use crate::class::constant::ConstantPool;
    use crate::vm::Frame;
    use crate::vm::interpreter::interpret;
    use crate::class::code::Instruction;
    use crate::vm::Value::*;
    use crate::class::code::Opcode::*;

    #[test]
    fn stack_management() {
        let constants = ConstantPool::new(2);
        let mut frame = Frame::new(10, 10, &constants);
        frame.operand_stack_depth = 100;

        frame.operand_stack = vec![Int(32)];
        interpret(&mut frame, &vec![Instruction::new(Pop, vec![])]);
        assert_eq!(frame.operand_stack, vec![]);

        frame.operand_stack = vec![Int(32), Int(32), Long(32)];
        interpret(
            &mut frame,
            &vec![
                Instruction::new(Pop2, vec![]),
                Instruction::new(Pop2, vec![]),
            ],
        );
        assert_eq!(frame.operand_stack, vec![]);


        frame.operand_stack = vec!(Int(21));
        interpret(&mut frame, &vec![Instruction::new(Dup, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(21), Int(21)]);

        frame.operand_stack = vec![Long(21)];
        interpret(&mut frame, &vec![Instruction::new(Dup2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(21), Long(21)]);

        frame.operand_stack = vec![Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(2), Int(1), Int(2), Int(1)]);

        frame.operand_stack = vec![Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(DupX1, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(1), Int(2), Int(1)]);

        frame.operand_stack = vec![Int(3), Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X1, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(2), Int(1), Int(3), Int(2), Int(1)]);

        frame.operand_stack = vec![Long(2), Long(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X1, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(1), Long(2), Long(1)]);

        frame.operand_stack = vec![Int(3), Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(DupX2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(1), Int(3), Int(2), Int(1)]);

        frame.operand_stack = vec![Long(2), Long(1)];
        interpret(&mut frame, &vec![Instruction::new(DupX2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(1), Long(2), Long(1)]);



        frame.operand_stack = vec![Int(4), Int(3), Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(2), Int(1), Int(4), Int(3), Int(2), Int(1)]);

        frame.operand_stack = vec![Int(3), Int(2), Long(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(1), Int(3), Int(2), Long(1)]);

        frame.operand_stack = vec![Long(3), Int(2), Int(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Int(2), Int(1), Long(3), Int(2), Int(1)]);

        frame.operand_stack = vec![Long(2), Long(1)];
        interpret(&mut frame, &vec![Instruction::new(Dup2X2, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(1), Long(2), Long(1)]);


        frame.operand_stack = vec![Long(2), Long(1)];
        interpret(&mut frame, &vec![Instruction::new(Swap, vec![])]);
        assert_eq!(frame.operand_stack, vec![Long(1), Long(2)]);
    }
}