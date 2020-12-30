#[macro_use]
mod test;

#[macro_use]
mod arithmetic;

#[macro_use]
mod object_creation_and_manipulation;

#[macro_use]
mod conversion;

#[macro_use]
mod control_transfer;

mod load_and_store;
mod method_invocation_and_return;
mod stack_management;

use crate::class::code::Instruction;
use crate::class::code::Opcode::*;
use crate::error::Result;
use crate::vm::data_type::Value::{Double, Float, Int, Long, Null, Reference};
use crate::vm::data_type::{
    ByteType, CharType, DoubleType, FloatType, IntType, LongType, ShortType,
};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;
use crate::vm::heap::HeapObject::{
    ByteArray, CharArray, DoubleArray, FloatArray, IntArray, LongArray, ShortArray,
};
use crate::vm::interpreter::arithmetic::*;
use crate::vm::interpreter::control_transfer::*;
use crate::vm::interpreter::load_and_store::*;
use crate::vm::interpreter::object_creation_and_manipulation::*;
use crate::vm::interpreter::stack_management::*;
use crate::vm::Command;
use crate::vm::Command::{
    VMAllocateReferenceArray, VMException, VMGetField, VMGetStatic, VMInvokeSpecial,
    VMInvokeStatic, VMInvokeVirtual, VMPutField, VMPutStatic, VMReturn,
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
    match &instruction.opcode {
        Nop => {}

        // Load and store:
        ILoad => load_int(frame, &instruction.operands),
        ILoad0 => load_int_n(frame, 0),
        ILoad1 => load_int_n(frame, 1),
        ILoad2 => load_int_n(frame, 2),
        ILoad3 => load_int_n(frame, 3),

        LLoad => load_long(frame, &instruction.operands),
        LLoad0 => load_long_n(frame, 0),
        LLoad1 => load_long_n(frame, 1),
        LLoad2 => load_long_n(frame, 2),
        LLoad3 => load_long_n(frame, 3),

        FLoad => load_float(frame, &instruction.operands),
        FLoad0 => load_float_n(frame, 0),
        FLoad1 => load_float_n(frame, 1),
        FLoad2 => load_float_n(frame, 2),
        FLoad3 => load_float_n(frame, 3),

        DLoad => load_double(frame, &instruction.operands),
        DLoad0 => load_double_n(frame, 0),
        DLoad1 => load_double_n(frame, 1),
        DLoad2 => load_double_n(frame, 2),
        DLoad3 => load_double_n(frame, 3),

        ALoad => load_reference(frame, &instruction.operands),
        ALoad0 => load_reference_n(frame, 0),
        ALoad1 => load_reference_n(frame, 1),
        ALoad2 => load_reference_n(frame, 2),
        ALoad3 => load_reference_n(frame, 3),

        IStore => store_int(frame, &instruction.operands)?,
        IStore0 => store_int_n(frame, 0)?,
        IStore1 => store_int_n(frame, 1)?,
        IStore2 => store_int_n(frame, 2)?,
        IStore3 => store_int_n(frame, 3)?,

        LStore => store_long(frame, &instruction.operands)?,
        LStore0 => store_long_n(frame, 0)?,
        LStore1 => store_long_n(frame, 1)?,
        LStore2 => store_long_n(frame, 2)?,
        LStore3 => store_long_n(frame, 3)?,

        FStore => store_float(frame, &instruction.operands)?,
        FStore0 => store_float_n(frame, 0)?,
        FStore1 => store_float_n(frame, 1)?,
        FStore2 => store_float_n(frame, 2)?,
        FStore3 => store_float_n(frame, 3)?,

        DStore => store_double(frame, &instruction.operands)?,
        DStore0 => store_double_n(frame, 0)?,
        DStore1 => store_double_n(frame, 1)?,
        DStore2 => store_double_n(frame, 2)?,
        DStore3 => store_double_n(frame, 3)?,

        AStore => store_reference(frame, &instruction.operands)?,
        AStore0 => store_reference_n(frame, 0)?,
        AStore1 => store_reference_n(frame, 1)?,
        AStore2 => store_reference_n(frame, 2)?,
        AStore3 => store_reference_n(frame, 3)?,

        BiPush => push_byte(frame, &instruction.operands),
        SiPush => push_short(frame, &instruction.operands),
        Ldc => push_constant(frame, &instruction.operands)?,
        LdcW => push_constant_wide(frame, &instruction.operands)?,
        Ldc2W => push_constant_long(frame, &instruction.operands)?,
        AConstNull => push_null(frame),

        IConstM1 => frame.push_operand(Int(-1)),
        IConst0 => frame.push_operand(Int(0)),
        IConst1 => frame.push_operand(Int(1)),
        IConst2 => frame.push_operand(Int(2)),
        IConst3 => frame.push_operand(Int(3)),
        IConst4 => frame.push_operand(Int(4)),
        IConst5 => frame.push_operand(Int(5)),

        LConst0 => frame.push_operand(Long(0)),
        LConst1 => frame.push_operand(Long(1)),

        FConst0 => frame.push_operand(Float(0.0)),
        FConst1 => frame.push_operand(Float(1.0)),
        FConst2 => frame.push_operand(Float(2.0)),

        DConst0 => frame.push_operand(Double(0.0)),
        DConst1 => frame.push_operand(Double(1.0)),

        // Wide => TODO

        // Arithmetic:
        IAdd => arithmetic!(frame, Int, +),
        LAdd => arithmetic!(frame, Long, +),
        FAdd => arithmetic!(frame, Float, +),
        DAdd => arithmetic!(frame, Double, +),

        ISub => arithmetic!(frame, Int, -),
        LSub => arithmetic!(frame, Long, -),
        FSub => arithmetic!(frame, Float, -),
        DSub => arithmetic!(frame, Double, -),

        IMul => arithmetic!(frame, Int, *),
        LMul => arithmetic!(frame, Long, *),
        FMul => arithmetic!(frame, Float, *),
        DMul => arithmetic!(frame, Double, *),

        IDiv => arithmetic!(frame, Int, /),
        LDiv => arithmetic!(frame, Long, /),
        FDiv => arithmetic!(frame, Float, /),
        DDiv => arithmetic!(frame, Double, /),

        IRem => arithmetic!(frame, Int, %),
        LRem => arithmetic!(frame, Long, %),
        FRem => arithmetic!(frame, Float, %),
        DRem => arithmetic!(frame, Double, %),

        INeg => neg_int(frame),
        LNeg => neg_long(frame),
        FNeg => neg_float(frame),
        DNeg => neg_double(frame),

        IShl => arithmetic!(frame, Int, |l, r| ((l as i32) << (r as i32 & 0x1f))
            as IntType),
        IShr => arithmetic!(frame, Int, |l, r| ((l as i32) >> (r as i32 & 0x1f))
            as IntType),
        IUshr => arithmetic!(frame, Int, |l, r| ((l as u32) >> (r as u32 & 0x1f))
            as IntType),

        LShl => arithmetic!(frame, Long, |l, r| ((l as i64) << (r as i32 & 0x1f))
            as LongType),
        LShr => arithmetic!(frame, Long, |l, r| ((l as i64) >> (r as i32 & 0x1f))
            as LongType),
        LUshr => arithmetic!(frame, Long, |l, r| ((l as u64) >> (r as u32 & 0x1f))
            as LongType),

        IOr => arithmetic!(frame, Int, |),
        LOr => arithmetic!(frame, Long, |),

        IAnd => arithmetic!(frame, Int, &),
        LAnd => arithmetic!(frame, Long, &),

        IXor => arithmetic!(frame, Int, ^),
        LXor => arithmetic!(frame, Long, ^),

        IInc => int_increase(frame, &instruction.operands),

        DCmpg => double_compare_g(frame),
        DCmpl => double_compare_l(frame),

        FCmpg => float_compare_g(frame),
        FCmpl => float_compare_l(frame),

        LCmp => long_compare(frame),

        // Conversion:
        I2l => convert!(frame, Int, Long, [LongType]),
        I2f => convert!(frame, Int, Float, [FloatType]),
        I2d => convert!(frame, Int, Double, [DoubleType]),
        L2f => convert!(frame, Long, Float, [FloatType]),
        L2d => convert!(frame, Long, Double, [DoubleType]),
        F2d => convert!(frame, Float, Double, [DoubleType]),

        I2b => convert!(frame, Int, Int, [ByteType, IntType]),
        I2c => convert!(frame, Int, Int, [u8, CharType, IntType]),
        I2s => convert!(frame, Int, Int, [ShortType, IntType]),
        L2i => convert!(frame, Long, Int, [IntType]),
        F2i => convert!(frame, Float, Int, [IntType]),
        F2l => convert!(frame, Float, Long, [LongType]),
        D2i => convert!(frame, Double, Int, [IntType]),
        D2l => convert!(frame, Double, Long, [LongType]),
        D2f => convert!(frame, Double, Float, [FloatType]),

        // Object creation and manipulation:
        New => new_object(frame, heap, &instruction.operands),

        NewArray => new_array(frame, heap, &instruction.operands)?,
        ANewArray => {
            return Ok(Some(VMAllocateReferenceArray(reference(
                &instruction.operands,
            ))))
        }
        // Multianewarray => TODO
        GetField => return Ok(Some(VMGetField(reference(&instruction.operands)))),
        PutField => return Ok(Some(VMPutField(reference(&instruction.operands)))),
        GetStatic => return Ok(Some(VMGetStatic(reference(&instruction.operands)))),
        PutStatic => return Ok(Some(VMPutStatic(reference(&instruction.operands)))),

        BaLoad => array_load!(frame, heap, ByteArray, Int, IntType),
        CaLoad => array_load!(frame, heap, CharArray, Int, IntType),
        SaLoad => array_load!(frame, heap, ShortArray, Int, IntType),
        IaLoad => array_load!(frame, heap, IntArray, Int, IntType),
        LaLoad => array_load!(frame, heap, LongArray, Long, LongType),
        FaLoad => array_load!(frame, heap, FloatArray, Float, FloatType),
        DaLoad => array_load!(frame, heap, DoubleArray, Double, DoubleType),
        AaLoad => reference_array_load(frame, heap),
        BaStore => array_store!(frame, heap, ByteArray, Int, [ByteType]),
        CaStore => array_store!(frame, heap, CharArray, Int, [u8, CharType]),
        SaStore => array_store!(frame, heap, ShortArray, Int, [ShortType]),
        IaStore => array_store!(frame, heap, IntArray, Int, [IntType]),
        LaStore => array_store!(frame, heap, LongArray, Long, [LongType]),
        FaStore => array_store!(frame, heap, FloatArray, Float, [FloatType]),
        DaStore => array_store!(frame, heap, DoubleArray, Double, [DoubleType]),
        AaStore => reference_array_store(frame, heap),
        ArrayLength => array_length(frame, heap)?,

        // Checkcast => TODO
        // Instanceof => TODO

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
        IfEq => if_cmp_zero!(frame, instruction, ==),
        IfNe => if_cmp_zero!(frame, instruction, !=),
        IfLt => if_cmp_zero!(frame, instruction, <),
        IfLe => if_cmp_zero!(frame, instruction, <=),
        IfGt => if_cmp_zero!(frame, instruction, >),
        IfGe => if_cmp_zero!(frame, instruction, >=),

        IfNull => if_null(frame, &instruction.operands),
        IfNonNull => if_non_null(frame, &instruction.operands),

        IfIcmpEq => if_cmp_operands!(frame, instruction, Int, ==),
        IfIcmpNe => if_cmp_operands!(frame, instruction, Int, !=),
        IfIcmpLt => if_cmp_operands!(frame, instruction, Int, <),
        IfIcmpLe => if_cmp_operands!(frame, instruction, Int, <=),
        IfIcmpGt => if_cmp_operands!(frame, instruction, Int, >),
        IfIcmpGe => if_cmp_operands!(frame, instruction, Int, >=),
        IfAcmpEq => if_cmp_operands!(frame, instruction, Reference, ==),
        IfAcmpNe => if_cmp_operands!(frame, instruction, Reference, !=),

        TableSwitch => table_switch(frame, &instruction.operands),
        LookupSwitch => lookup_switch(frame, &instruction.operands),

        Goto => goto(frame, &instruction.operands),
        GotoW => goto_wide(frame, &instruction.operands),
        Jsr => jump_subroutine(frame, &instruction.operands),
        JsrW => jump_subroutine_wide(frame, &instruction.operands),
        Ret => return_from_subroutine(frame, &instruction.operands),

        // Method invocation and return
        InvokeVirtual => return Ok(Some(VMInvokeVirtual(reference(&instruction.operands)))),
        // Invokeinterface => TODO
        InvokeSpecial => return Ok(Some(VMInvokeSpecial(reference(&instruction.operands)))),
        InvokeStatic => return Ok(Some(VMInvokeStatic(reference(&instruction.operands)))),
        // Invokedynamic => TODO
        Return => return Ok(Some(VMReturn(Null))),
        IReturn => return Ok(Some(VMReturn(Int(frame.pop_operand().expect_int())))),
        LReturn => return Ok(Some(VMReturn(Long(frame.pop_operand().expect_long())))),
        FReturn => return Ok(Some(VMReturn(Float(frame.pop_operand().expect_float())))),
        DReturn => return Ok(Some(VMReturn(Double(frame.pop_operand().expect_double())))),
        AReturn => {
            return Ok(Some(VMReturn(Reference(
                frame.pop_operand().expect_reference(),
            ))))
        }

        // Throwing exceptions:
        AThrow => return Ok(Some(VMException())),

        // Implementation specific
        OperationSpacer => panic!("Tried to parse operation as instruction in {}", frame),
        ImpDep2 => eprintln!("ImpDep2 not implemented"),
        BreakPoint => eprintln!("Breakpoint not implemented"),

        _ => unimplemented!(
            "Opcode {:?} is not implemented in interpreter",
            instruction.opcode
        ),
        // Synchronization
        // MonitorEnter => TODO
        // MonitorExit => TODO
    }

    Ok(None)
}

fn reference(operands: &[u8]) -> u16 {
    (operands[0] as u16) << 8 | operands[1] as u16
}
