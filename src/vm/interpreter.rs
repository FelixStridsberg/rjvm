#[macro_use]
mod test;

mod arithmetic;
mod control_transfer;
mod conversion;
mod load_and_store;
mod method_invocation_and_return;
mod object_creation_and_manipulation;
mod stack_management;

use crate::class::code::Instruction;
use crate::class::code::Opcode::*;
use crate::error::Result;
use crate::vm::data_type::Value::{Double, Float, Int, Long, Null, Reference};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;
use crate::vm::interpreter::arithmetic::*;
use crate::vm::interpreter::control_transfer::*;
use crate::vm::interpreter::conversion::*;
use crate::vm::interpreter::load_and_store::*;
use crate::vm::interpreter::object_creation_and_manipulation::*;
use crate::vm::interpreter::stack_management::*;
use crate::vm::stack::Stack;
use crate::vm::Command;
use crate::vm::Command::{
    VMException, VMGetField, VMGetStatic, VMInvokeSpecial, VMInvokeStatic, VMInvokeVirtual,
    VMPutField, VMPutStatic, VMReturn,
};

pub(super) fn interpret_frame(frame: &mut Frame, heap: &mut Heap) -> Result<Command> {
    loop {
        let instruction = &frame.code.clone().instructions[frame.pc as usize];
        if let Some(vm_command) = interpret_instruction(frame, heap, instruction)? {
            return Ok(vm_command);
        }

        frame.pc_next();
    }
}

fn interpret_instruction(
    frame: &mut Frame,
    heap: &mut Heap,
    instruction: &Instruction,
) -> Result<Option<Command>> {
    let mut command = None;

    match &instruction.opcode {
        // Load and store:
        Iload => load_int(frame, &instruction.operands),
        Iload0 => load_int_n(frame, 0),
        Iload1 => load_int_n(frame, 1),
        Iload2 => load_int_n(frame, 2),
        Iload3 => load_int_n(frame, 3),

        Lload => load_long(frame, &instruction.operands),
        Lload0 => load_long_n(frame, 0),
        Lload1 => load_long_n(frame, 1),
        Lload2 => load_long_n(frame, 2),
        Lload3 => load_long_n(frame, 3),

        Fload => load_float(frame, &instruction.operands),
        Fload0 => load_float_n(frame, 0),
        Fload1 => load_float_n(frame, 1),
        Fload2 => load_float_n(frame, 2),
        Fload3 => load_float_n(frame, 3),

        Dload => load_double(frame, &instruction.operands),
        Dload0 => load_double_n(frame, 0),
        Dload1 => load_double_n(frame, 1),
        Dload2 => load_double_n(frame, 2),
        Dload3 => load_double_n(frame, 3),

        Aload => load_reference(frame, &instruction.operands),
        Aload0 => load_reference_n(frame, 0),
        Aload1 => load_reference_n(frame, 1),
        Aload2 => load_reference_n(frame, 2),
        Aload3 => load_reference_n(frame, 3),

        Istore => store_int(frame, &instruction.operands)?,
        Istore0 => store_int_n(frame, 0)?,
        Istore1 => store_int_n(frame, 1)?,
        Istore2 => store_int_n(frame, 2)?,
        Istore3 => store_int_n(frame, 3)?,

        Lstore => store_long(frame, &instruction.operands)?,
        Lstore0 => store_long_n(frame, 0)?,
        Lstore1 => store_long_n(frame, 1)?,
        Lstore2 => store_long_n(frame, 2)?,
        Lstore3 => store_long_n(frame, 3)?,

        Fstore => store_float(frame, &instruction.operands)?,
        Fstore0 => store_float_n(frame, 0)?,
        Fstore1 => store_float_n(frame, 1)?,
        Fstore2 => store_float_n(frame, 2)?,
        Fstore3 => store_float_n(frame, 3)?,

        Dstore => store_double(frame, &instruction.operands)?,
        Dstore0 => store_double_n(frame, 0)?,
        Dstore1 => store_double_n(frame, 1)?,
        Dstore2 => store_double_n(frame, 2)?,
        Dstore3 => store_double_n(frame, 3)?,

        Astore => store_reference(frame, &instruction.operands)?,
        Astore0 => store_reference_n(frame, 0)?,
        Astore1 => store_reference_n(frame, 1)?,
        Astore2 => store_reference_n(frame, 2)?,
        Astore3 => store_reference_n(frame, 3)?,

        Bipush => push_byte(frame, &instruction.operands),
        Sipush => push_short(frame, &instruction.operands),
        Ldc => push_constant(frame, &instruction.operands)?,
        LdcW => push_constant_wide(frame, &instruction.operands)?,
        Ldc2W => push_constant_long(frame, &instruction.operands)?,
        AconstNull => push_null(frame),

        IconstM1 => frame.push_operand(Int(-1)),
        Iconst0 => frame.push_operand(Int(0)),
        Iconst1 => frame.push_operand(Int(1)),
        Iconst2 => frame.push_operand(Int(2)),
        Iconst3 => frame.push_operand(Int(3)),
        Iconst4 => frame.push_operand(Int(4)),
        Iconst5 => frame.push_operand(Int(5)),

        Lconst0 => frame.push_operand(Long(0)),
        Lconst1 => frame.push_operand(Long(1)),

        Fconst0 => frame.push_operand(Float(0.0)),
        Fconst1 => frame.push_operand(Float(1.0)),
        Fconst2 => frame.push_operand(Float(2.0)),

        Dconst0 => frame.push_operand(Double(0.0)),
        Dconst1 => frame.push_operand(Double(1.0)),

        Wide => unimplemented!("wide not implemented."),

        // Arithmetic:
        Iadd => add_int(frame),
        Ladd => add_long(frame),
        Fadd => add_float(frame),
        Dadd => add_double(frame),

        Isub => sub_int(frame),
        Lsub => sub_long(frame),
        Fsub => sub_float(frame),
        Dsub => sub_double(frame),

        Imul => mul_int(frame),
        Lmul => mul_long(frame),
        Fmul => mul_float(frame),
        Dmul => mul_double(frame),

        Idiv => div_int(frame),
        Ldiv => div_long(frame),
        Fdiv => div_float(frame),
        Ddiv => div_double(frame),

        Irem => rem_int(frame),
        Lrem => rem_long(frame),
        Frem => rem_float(frame),
        Drem => rem_double(frame),

        Ineg => neg_int(frame),
        Lneg => neg_long(frame),
        Fneg => neg_float(frame),
        Dneg => neg_double(frame),

        Ishl => int_shift_left(frame),
        Ishr => int_shift_right(frame),
        Iushr => int_logical_shift_right(frame),

        Lshl => long_shift_left(frame),
        Lshr => long_shift_right(frame),
        Lushr => long_logical_shift_right(frame),

        Ior => int_bitwise_or(frame),
        Lor => long_bitwise_or(frame),

        Iand => int_bitwise_and(frame),
        Land => long_bitwise_and(frame),

        Ixor => int_bitwise_exclusive_or(frame),
        Lxor => long_bitwise_exclusive_or(frame),

        Iinc => int_increase(frame, &instruction.operands),

        Dcmpg => double_compare_g(frame),
        Dcmpl => double_compare_l(frame),

        Fcmpg => float_compare_g(frame),
        Fcmpl => float_compare_l(frame),

        Lcmp => long_compare(frame),

        // Conversion:
        I2l => int_to_long(frame),
        I2f => int_to_float(frame),
        I2d => int_to_double(frame),
        L2f => long_to_float(frame),
        L2d => long_to_double(frame),
        F2d => float_to_double(frame),

        I2b => int_to_byte(frame),
        I2c => int_to_char(frame),
        I2s => int_to_short(frame),
        L2i => long_to_int(frame),
        F2i => float_to_int(frame),
        F2l => float_to_long(frame),
        D2i => double_to_int(frame),
        D2l => double_to_long(frame),
        D2f => double_to_float(frame),

        // Object creation and manipulation:
        // TODO
        NewArray => new_array(frame, heap, &instruction.operands)?,
        ArrayLength => array_length(frame, heap)?,
        New => new_object(frame, heap, &instruction.operands),
        Iastore => int_array_store(frame, heap),
        Iaload => int_array_load(frame, heap),
        PutField => command = Some(VMPutField(reference(&instruction.operands))),
        GetField => command = Some(VMGetField(reference(&instruction.operands))),
        PutStatic => command = Some(VMPutStatic(reference(&instruction.operands))),
        Getstatic => command = Some(VMGetStatic(reference(&instruction.operands))),

        // Operand stack management:
        Pop => pop_operand(frame),
        Pop2 => pop_operand_long(frame),
        Dup => duplicate_operand(frame),
        Dup2 => duplicate_operand_long(frame),
        DupX1 => duplicate_operand_back1(frame),
        Dup2X1 => duplicate_operand_long_back1(frame),
        DupX2 => duplicate_operand_back2(frame),
        Dup2X2 => duplicate_operand_long_back2(frame),
        Swap => swap_operand(frame),

        // Control transfer:
        IfEq => {
            if_equals(frame, &instruction.operands);
            return Ok(None);
        }
        IfNe => {
            if_not_equals(frame, &instruction.operands);
            return Ok(None);
        }
        IfLt => {
            if_less_than(frame, &instruction.operands);
            return Ok(None);
        }
        IfLe => {
            if_less_than_inclusive(frame, &instruction.operands);
            return Ok(None);
        }
        IfGt => {
            if_greater_than(frame, &instruction.operands);
            return Ok(None);
        }
        IfGe => {
            if_greater_than_inclusive(frame, &instruction.operands);
            return Ok(None);
        }
        IfNull => {
            if_null(frame, &instruction.operands);
            return Ok(None);
        }
        IfNonNull => {
            if_non_null(frame, &instruction.operands);
            return Ok(None);
        }

        IfIcmpEq => {
            if_int_equals(frame, &instruction.operands);
            return Ok(None);
        }
        IfIcmpNe => {
            if_int_not_equals(frame, &instruction.operands);
            return Ok(None);
        }
        IfIcmpLt => {
            if_int_less_than(frame, &instruction.operands);
            return Ok(None);
        }
        IfIcmpLe => {
            if_int_less_than_inclusive(frame, &instruction.operands);
            return Ok(None);
        }
        IfIcmpGt => {
            if_int_greater_than(frame, &instruction.operands);
            return Ok(None);
        }
        IfIcmpGe => {
            if_int_greater_than_inclusive(frame, &instruction.operands);
            return Ok(None);
        }

        IfAcmpEq => {
            if_reference_equals(frame, &instruction.operands);
            return Ok(None);
        }
        IfAcmpNe => {
            if_reference_not_equals(frame, &instruction.operands);
            return Ok(None);
        }
        TableSwitch => panic!("TableSwitch not implemented"),
        LookupSwitch => {
            lookup_switch(frame, &instruction.operands);
            return Ok(None);
        }
        Goto => {
            goto(frame, &instruction.operands);
            return Ok(None);
        }
        GotoW => {
            goto_wide(frame, &instruction.operands);
            return Ok(None);
        }
        Jsr => {
            jump_subroutine(frame, &instruction.operands);
            return Ok(None);
        }
        JsrW => {
            jump_subroutine_wide(frame, &instruction.operands);
            return Ok(None);
        }
        Ret => {
            return_from_subroutine(frame, &instruction.operands);
            return Ok(None);
        }

        // Method invocation and return
        // TODO
        Return => command = Some(VMReturn(Null)),
        Ireturn => command = Some(VMReturn(Int(frame.pop_operand().expect_int()))),
        Lreturn => command = Some(VMReturn(Long(frame.pop_operand().expect_long()))),
        Freturn => command = Some(VMReturn(Float(frame.pop_operand().expect_float()))),
        Dreturn => command = Some(VMReturn(Double(frame.pop_operand().expect_double()))),
        Areturn => command = Some(VMReturn(Reference(frame.pop_operand().expect_reference()))),

        Invokespecial => command = Some(VMInvokeSpecial(reference(&instruction.operands))),
        Invokestatic => command = Some(VMInvokeStatic(reference(&instruction.operands))),
        Invokevirtual => command = Some(VMInvokeVirtual(reference(&instruction.operands))),

        // Throwing exceptions:
        Athrow => command = Some(VMException()),

        OperationSpacer => panic!("Tried to parse operation as instruction in {}", frame),
        _ => unimplemented!(
            "Opcode {:?} is not implemented in interpreter",
            instruction.opcode
        ),
    }

    Ok(command)
}

fn reference(operands: &[u8]) -> u16 {
    (operands[0] as u16) << 8 | operands[1] as u16
}
