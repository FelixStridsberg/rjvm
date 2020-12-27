use bitflags::_core::fmt::Formatter;
use core::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<u8>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u8>) -> Self {
        Instruction { opcode, operands }
    }

    pub fn operation_spacer() -> Self {
        Self::new(Opcode::OperationSpacer, vec![])
    }

    pub fn size(&self) -> u32 {
        1 + self.operands.len() as u32
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:<15}{:?}", format!("{:?}", self.opcode), self.operands)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Opcode {
    Aaload,
    Aastore,
    AconstNull,
    Aload,
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray,
    Areturn,
    Arraylength,
    Astore,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush,
    Caload,
    Castore,
    Checkcast,
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload,
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore,
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload,
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore,
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    GetField,
    Getstatic,
    Goto,
    GotoW,
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpEq,
    IfAcmpNe,
    IfIcmpEq,
    IfIcmpNe,
    IfIcmpLt,
    IfIcmpGe,
    IfIcmpGt,
    IfIcmpLe,
    IfEq,
    IfNe,
    IfLt,
    IfGe,
    IfGt,
    IfLe,
    IfNonNull,
    IfNull,
    Iinc,
    Iload,
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Instanceof,
    Invokedynamic,
    Invokeinterface,
    Invokespecial,
    Invokestatic,
    Invokevirtual,
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore,
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr,
    JsrW,
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc,
    LdcW,
    Ldc2W,
    Ldiv,
    Lload,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    LookupSwitch,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore,
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    MonitorEnter,
    MonitorExit,
    Multianewarray,
    New,
    NewArray,
    Nop,
    Pop,
    Pop2,
    PutField,
    PutStatic,
    Ret,
    Return,
    Saload,
    Sastore,
    Sipush,
    Swap,
    TableSwitch,
    Wide,
    ImpDep2,
    BreakPoint,
    OperationSpacer, // Used to mark operations in code array
}
