use crate::vm::{Frame, Value};
use crate::class::code::Instruction;
use crate::class::code::Opcode::*;
use crate::vm::Value::Int;
use crate::class::attribute::Code;

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
        IconstM1 => frame.push_operand(Int(-1)),
        Iconst0 => frame.push_operand(Int(0)),
        Iconst1 => frame.push_operand(Int(1)),
        Iconst2 => frame.push_operand(Int(2)),
        Iconst3 => frame.push_operand(Int(3)),
        Iconst4 => frame.push_operand(Int(4)),
        Iconst5 => frame.push_operand(Int(5)),

        Istore0 => store_int_variable(frame, 0),
        Istore1 => store_int_variable(frame, 1),
        Istore2 => store_int_variable(frame, 2),
        Istore3 => store_int_variable(frame, 3),

        Iload0 => load_int_variable(frame, 0),
        Iload1 => load_int_variable(frame, 1),
        Iload2 => load_int_variable(frame, 2),
        Iload3 => load_int_variable(frame, 3),

        Iinc => iinc(frame, &instruction.operands),

        Ireturn => return Some(frame.pop_operand()),

        _ => unimplemented!("Opcode {:?} is not implemented in interpreter", instruction.opcode)
    }

    None
}

fn store_int_variable(frame: &mut Frame, index: u16) {
    let operand = frame.pop_operand();
    if let Int(value) = operand {
        frame.set_local(index, value as u32);
    } else {
        panic!("istoreX expected an int value on top of the stack. Got {:?}", operand);
    }
}

fn load_int_variable(frame: &mut Frame, index: u16) {
    let int = frame.get_local(index) as i32;
    frame.push_operand(Int(int));
}

fn iinc(frame: &mut Frame, operands: &Vec<u8>) {
    let index = operands[0] as u16;
    let constant = operands[1] as u32;
    let value = frame.get_local(index);
    frame.set_local(index, value + constant);
}

#[cfg(test)]
mod test {
    use crate::class::attribute::Code;
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::{Iconst0, Istore0, Iinc, Iload0, Ireturn};
    use crate::vm::Frame;
    use crate::class::constant::ConstantPool;
    use crate::vm::interpreter::interpret;
    use crate::vm::Value::Int;

    #[test]
    fn inc_and_return() {
        /*
          Code:
            int i = 0;
            i++;
            return i;
         */
        let code = Code {
            max_stack: 1,
            max_locals: 1,
            attributes: vec![],
            instructions: vec![
                Instruction::new(Iconst0, vec![]),
                Instruction::new(Istore0, vec![]),
                Instruction::new(Iinc, vec![0x00, 0x01]),
                Instruction::new(Iload0, vec![]),
                Instruction::new(Ireturn, vec![]),
            ]
        };

        let constants = ConstantPool::new(0);
        let mut frame = Frame::new(1, 1, &constants);

        assert_eq!(interpret(&mut frame, &code.instructions), Some(Int(1)));
    }
}