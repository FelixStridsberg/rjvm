#[macro_use]
mod test_macro;

#[macro_use]
mod arithmetic;

#[macro_use]
mod object_creation_and_manipulation;

#[macro_use]
mod conversion;

#[macro_use]
mod control_transfer;

#[macro_use]
mod load_and_store;

mod stack_management;

use crate::class::code::Instruction;
use crate::class::code::Opcode::*;
use crate::error::Result;
use crate::vm::data_type::Value::{Double, Float, Int, Long, Null, Reference, ReturnAddress};
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
use crate::vm::interpreter::InterpretResult::{Command, Jump, Normal};
use crate::vm::VMCommand;
use crate::vm::VMCommand::{
    VMAllocateReferenceArray, VMException, VMGetField, VMGetStatic, VMInvokeInterface,
    VMInvokeSpecial, VMInvokeStatic, VMInvokeVirtual, VMNative, VMPutField, VMPutStatic, VMReturn,
};

macro_rules! jump (
    ($ins:expr) => {{
            $ins;
            return Ok(Jump);
    }}
);

macro_rules! vm_command (
    ($ins:expr) => {{
        return Ok(Command($ins));
    }}
);

enum InterpretResult {
    Normal,
    Jump,
    Command(VMCommand),
}

pub(super) fn interpret_frame(frame: &mut Frame, heap: &mut Heap) -> Result<VMCommand> {
    loop {
        if frame.code.is_none() {
            return Ok(VMNative());
        }

        let instruction = &frame.code.as_ref().unwrap().clone().instructions[frame.pc as usize];

        debug!("-------------");
        debug!("{}", frame);
        debug!("[I] {}", instruction);

        let result = interpret_instruction(frame, heap, instruction)?;

        if let Command(vm_command) = result {
            return Ok(vm_command);
        }

        if !matches!(result, Jump) {
            frame.pc_next();
        }
    }
}

fn interpret_instruction(
    frame: &mut Frame,
    heap: &mut Heap,
    instruction: &Instruction,
) -> Result<InterpretResult> {
    match &instruction.opcode {
        Nop => {}

        // Load and store:
        ILoad => load!(frame, instruction, Int(_)),
        ILoad0 => load!(frame, Int(_), 0),
        ILoad1 => load!(frame, Int(_), 1),
        ILoad2 => load!(frame, Int(_), 2),
        ILoad3 => load!(frame, Int(_), 3),

        LLoad => load!(frame, instruction, Long(_)),
        LLoad0 => load!(frame, Long(_), 0),
        LLoad1 => load!(frame, Long(_), 1),
        LLoad2 => load!(frame, Long(_), 2),
        LLoad3 => load!(frame, Long(_), 3),

        FLoad => load!(frame, instruction, Float(_)),
        FLoad0 => load!(frame, Float(_), 0),
        FLoad1 => load!(frame, Float(_), 1),
        FLoad2 => load!(frame, Float(_), 2),
        FLoad3 => load!(frame, Float(_), 3),

        DLoad => load!(frame, instruction, Double(_)),
        DLoad0 => load!(frame, Double(_), 0),
        DLoad1 => load!(frame, Double(_), 1),
        DLoad2 => load!(frame, Double(_), 2),
        DLoad3 => load!(frame, Double(_), 3),

        ALoad => load!(frame, instruction, Reference(_) | Null),
        ALoad0 => load!(frame, Reference(_) | Null, 0),
        ALoad1 => load!(frame, Reference(_) | Null, 1),
        ALoad2 => load!(frame, Reference(_) | Null, 2),
        ALoad3 => load!(frame, Reference(_) | Null, 3),

        IStore => store!(frame, instruction, Int(_)),
        IStore0 => store!(frame, Int(_), 0),
        IStore1 => store!(frame, Int(_), 1),
        IStore2 => store!(frame, Int(_), 2),
        IStore3 => store!(frame, Int(_), 3),

        LStore => store!(frame, instruction, Long(_)),
        LStore0 => store!(frame, Long(_), 0),
        LStore1 => store!(frame, Long(_), 1),
        LStore2 => store!(frame, Long(_), 2),
        LStore3 => store!(frame, Long(_), 3),

        FStore => store!(frame, instruction, Float(_)),
        FStore0 => store!(frame, Float(_), 0),
        FStore1 => store!(frame, Float(_), 1),
        FStore2 => store!(frame, Float(_), 2),
        FStore3 => store!(frame, Float(_), 3),

        DStore => store!(frame, instruction, Double(_)),
        DStore0 => store!(frame, Double(_), 0),
        DStore1 => store!(frame, Double(_), 1),
        DStore2 => store!(frame, Double(_), 2),
        DStore3 => store!(frame, Double(_), 3),

        AStore => store!(frame, instruction, Reference(_) | ReturnAddress(_) | Null),
        AStore0 => store!(frame, Reference(_) | ReturnAddress(_) | Null, 0),
        AStore1 => store!(frame, Reference(_) | ReturnAddress(_) | Null, 1),
        AStore2 => store!(frame, Reference(_) | ReturnAddress(_) | Null, 2),
        AStore3 => store!(frame, Reference(_) | ReturnAddress(_) | Null, 3),

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

        LShl => arithmetic_long!(frame, |l, r| ((l as i64) << (r as i32 & 0x1f)) as LongType),
        LShr => arithmetic_long!(frame, |l, r| ((l as i64) >> (r as i32 & 0x1f)) as LongType),
        LUshr => arithmetic_long!(frame, |l, r| ((l as u64) >> (r as u32 & 0x1f)) as LongType),

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
        ANewArray => vm_command!(VMAllocateReferenceArray(reference(&instruction.operands,))),
        // Multianewarray => TODO
        GetField => vm_command!(VMGetField(reference(&instruction.operands))),
        PutField => vm_command!(VMPutField(reference(&instruction.operands))),
        GetStatic => vm_command!(VMGetStatic(reference(&instruction.operands))),
        PutStatic => vm_command!(VMPutStatic(reference(&instruction.operands))),

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
        IfEq => jump!(if_cmp_zero!(frame, instruction, ==)),
        IfNe => jump!(if_cmp_zero!(frame, instruction, !=)),
        IfLt => jump!(if_cmp_zero!(frame, instruction, <)),
        IfLe => jump!(if_cmp_zero!(frame, instruction, <=)),
        IfGt => jump!(if_cmp_zero!(frame, instruction, >)),
        IfGe => jump!(if_cmp_zero!(frame, instruction, >=)),

        IfNull => jump!(if_null(frame, &instruction.operands)),
        IfNonNull => jump!(if_non_null(frame, &instruction.operands)),

        IfIcmpEq => jump!(if_cmp_operands!(frame, instruction, Int, ==)),
        IfIcmpNe => jump!(if_cmp_operands!(frame, instruction, Int, !=)),
        IfIcmpLt => jump!(if_cmp_operands!(frame, instruction, Int, <)),
        IfIcmpLe => jump!(if_cmp_operands!(frame, instruction, Int, <=)),
        IfIcmpGt => jump!(if_cmp_operands!(frame, instruction, Int, >)),
        IfIcmpGe => jump!(if_cmp_operands!(frame, instruction, Int, >=)),
        IfAcmpEq => jump!(if_cmp_operands!(frame, instruction, Reference, ==)),
        IfAcmpNe => jump!(if_cmp_operands!(frame, instruction, Reference, !=)),

        TableSwitch => table_switch(frame, &instruction.operands),
        LookupSwitch => lookup_switch(frame, &instruction.operands),

        Goto => jump!(goto(frame, &instruction.operands)),
        GotoW => jump!(goto_wide(frame, &instruction.operands)),
        Jsr => jump!(jump_subroutine(frame, &instruction.operands)),
        JsrW => jump!(jump_subroutine_wide(frame, &instruction.operands)),
        Ret => jump!(return_from_subroutine(frame, &instruction.operands)),

        // Method invocation and return
        InvokeVirtual => vm_command!(VMInvokeVirtual(reference(&instruction.operands))),
        InvokeInterface => vm_command!(VMInvokeInterface(reference(&instruction.operands))),
        InvokeSpecial => vm_command!(VMInvokeSpecial(reference(&instruction.operands))),
        InvokeStatic => vm_command!(VMInvokeStatic(reference(&instruction.operands))),
        // Invokedynamic => TODO
        Return => vm_command!(VMReturn(Null)),
        IReturn => vm_command!(VMReturn(Int(frame.pop_operand().expect_int_like()))),
        LReturn => vm_command!(VMReturn(Long(frame.pop_operand().expect_long()))),
        FReturn => vm_command!(VMReturn(Float(frame.pop_operand().expect_float()))),
        DReturn => vm_command!(VMReturn(Double(frame.pop_operand().expect_double()))),
        AReturn => {
            return Ok(Command(VMReturn(
                frame
                    .pop_operand()
                    .expect_nullable_reference()
                    .map_or(Null, |r| Reference(r)),
            )))
        }

        // Throwing exceptions:
        AThrow => vm_command!(VMException()),

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

    Ok(Normal)
}

fn reference(operands: &[u8]) -> u16 {
    (operands[0] as u16) << 8 | operands[1] as u16
}
