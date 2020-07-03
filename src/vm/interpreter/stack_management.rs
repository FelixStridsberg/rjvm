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
