use bitflags::_core::fmt::Formatter;
use core::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<u8>,
    pad: u8,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u8>) -> Self {
        Instruction {
            opcode,
            operands,
            pad: 0,
        }
    }

    pub fn new_with_pad(opcode: Opcode, operands: Vec<u8>, pad: u8) -> Self {
        let mut instruction = Self::new(opcode, operands);
        instruction.pad = pad;
        instruction
    }

    pub fn operation_spacer() -> Self {
        Self::new(Opcode::OperationSpacer, vec![])
    }

    pub fn size(&self) -> u16 {
        1 + self.operands.len() as u16 + self.pad as u16
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:<15}{:?}", format!("{:?}", self.opcode), self.operands)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Opcode {
    AaLoad,
    AaStore,
    AConstNull,
    ALoad,
    ALoad0,
    ALoad1,
    ALoad2,
    ALoad3,
    ANewArray,
    AReturn,
    ArrayLength,
    AStore,
    AStore0,
    AStore1,
    AStore2,
    AStore3,
    AThrow,
    BaLoad,
    BaStore,
    BiPush,
    CaLoad,
    CaStore,
    CheckCast,
    D2f,
    D2i,
    D2l,
    DAdd,
    DaLoad,
    DaStore,
    DCmpg,
    DCmpl,
    DConst0,
    DConst1,
    DDiv,
    DLoad,
    DLoad0,
    DLoad1,
    DLoad2,
    DLoad3,
    DMul,
    DNeg,
    DRem,
    DReturn,
    DStore,
    DStore0,
    DStore1,
    DStore2,
    DStore3,
    DSub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    FAdd,
    FaLoad,
    FaStore,
    FCmpg,
    FCmpl,
    FConst0,
    FConst1,
    FConst2,
    FDiv,
    FLoad,
    FLoad0,
    FLoad1,
    FLoad2,
    FLoad3,
    FMul,
    FNeg,
    FRem,
    FReturn,
    FStore,
    FStore0,
    FStore1,
    FStore2,
    FStore3,
    FSub,
    GetField,
    GetStatic,
    Goto,
    GotoW,
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    IAdd,
    IaLoad,
    IAnd,
    IaStore,
    IConstM1,
    IConst0,
    IConst1,
    IConst2,
    IConst3,
    IConst4,
    IConst5,
    IDiv,
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
    IInc,
    ILoad,
    ILoad0,
    ILoad1,
    ILoad2,
    ILoad3,
    IMul,
    INeg,
    Instanceof,
    InvokeDynamic,
    InvokeInterface,
    InvokeSpecial,
    InvokeStatic,
    InvokeVirtual,
    IOr,
    IRem,
    IReturn,
    IShl,
    IShr,
    IStore,
    IStore0,
    IStore1,
    IStore2,
    IStore3,
    ISub,
    IUshr,
    IXor,
    Jsr,
    JsrW,
    L2d,
    L2f,
    L2i,
    LAdd,
    LaLoad,
    LAnd,
    LaStore,
    LCmp,
    LConst0,
    LConst1,
    Ldc,
    LdcW,
    Ldc2W,
    LDiv,
    LLoad,
    LLoad0,
    LLoad1,
    LLoad2,
    LLoad3,
    LMul,
    LNeg,
    LookupSwitch,
    LOr,
    LRem,
    LReturn,
    LShl,
    LShr,
    LStore,
    LStore0,
    LStore1,
    LStore2,
    LStore3,
    LSub,
    LUshr,
    LXor,
    MonitorEnter,
    MonitorExit,
    MultiANewArray,
    New,
    NewArray,
    Nop,
    Pop,
    Pop2,
    PutField,
    PutStatic,
    Ret,
    Return,
    SaLoad,
    SaStore,
    SiPush,
    Swap,
    TableSwitch,
    Wide,
    ImpDep2,
    BreakPoint,
    OperationSpacer, // Used to mark operations in code array
}
