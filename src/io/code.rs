use crate::class::code::Opcode::*;
use crate::class::code::{Instruction, Opcode};
use crate::error::Result;
use crate::io::ReadBytesExt;
use std::io::BufRead;

pub struct CodeReader<'r, R: BufRead> {
    reader: &'r mut R,
}

impl<'r, R: BufRead> CodeReader<'r, R> {
    pub fn new(reader: &'r mut R) -> CodeReader<'r, R> {
        CodeReader { reader }
    }

    pub fn read_code(&mut self) -> Result<Vec<Instruction>> {
        let mut byte_count = self.reader.read_u2()?;
        let mut code = Vec::new();

        loop {
            let (opcode, argc) = self.read_opcode()?;
            let operands = self.reader.read_bytes(argc as usize)?;
            code.push(Instruction::new(opcode, operands));

            byte_count -= 1 + argc as u16;
            if byte_count == 0 {
                break;
            }
        }

        Ok(code)
    }

    fn read_opcode(&mut self) -> Result<(Opcode, u8)> {
        Ok(match self.reader.read_u1()? {
            // 0x32 => Aaload,
            // 0x53 => Aastore,
            // 0x01 => AconstNull,
            // 0x19 => Aload,
            // 0x2a => Aload0,
            // 0x2b => Aload1,
            // 0x2c => Aload2,
            // 0x2d => Aload3,
            // 0xbd => Anewarray,
            // 0xb0 => Areturn,
            // 0xbe => Arraylength,
            // 0x3a => Astore,
            // 0x4b => Astore0,
            // 0x4c => Astore1,
            // 0x4d => Astore2,
            // 0x4e => Astore3,
            // 0xbf => Athrow,
            // 0x33 => Baload,
            // 0x54 => Bastore,
            // 0x10 => Bipush,
            // 0x34 => Caload,
            // 0x55 => Castore,
            // 0xc0 => Checkcast,
            // 0x90 => D2f,
            // 0x8e => D2i,
            // 0x8f => D2l,
            // 0x63 => Dadd,
            // 0x31 => Daload,
            // 0x52 => Dastore,
            // 0x98 => Dcmpg,
            // 0x97 => Dcmpl,
            // 0x0e => Dconst0,
            // 0x0f => Dconst1,
            // 0x6f => Ddiv,
            // 0x18 => Dload,
            // 0x26 => Dload0,
            // 0x27 => Dload1,
            // 0x28 => Dload2,
            // 0x29 => Dload3,
            // 0x6b => Dmul,
            // 0x77 => Dneg,
            // 0x73 => Drem,
            // 0xaf => Dreturn,
            // 0x39 => Dstore,
            // 0x47 => Dstore0,
            // 0x48 => Dstore1,
            // 0x49 => Dstore2,
            // 0x4a => Dstore3,
            // 0x67 => Dsub,
            // 0x59 => Dup,
            // 0x5a => DupX1,
            // 0x5b => DupX2,
            // 0x5c => Dup2,
            // 0x5d => Dup2X1,
            // 0x5e => Dup2X2,
            // 0x8d => F2d,
            // 0x8b => F2i,
            // 0x8c => F2l,
            // 0x62 => Fadd,
            // 0x30 => Faload,
            // 0x51 => Fastore,
            // 0x96 => Fcmpg,
            // 0x95 => Fcmpl,
            // 0x0b => Fconst0,
            // 0x0c => Fconst1,
            // 0x0d => Fconst2,
            // 0x6e => Fdiv,
            // 0x17 => Fload,
            // 0x22 => Fload0,
            // 0x23 => Fload1,
            // 0x24 => Fload2,
            // 0x25 => Fload3,
            // 0x6a => Fmul,
            // 0x76 => Fneg,
            // 0x72 => Frem,
            // 0xae => Freturn,
            // 0x38 => Fstore,
            // 0x43 => Fstore0,
            // 0x44 => Fstore1,
            // 0x45 => Fstore2,
            // 0x46 => Fstore3,
            // 0x66 => Fsub,
            // 0xb4 => Getfield,
            // 0xb2 => Getstatic,
            // 0xa7 => Goto,
            // 0xc8 => GotoW,
            // 0x91 => I2b,
            // 0x92 => I2c,
            // 0x87 => I2d,
            // 0x86 => I2f,
            // 0x85 => I2l,
            // 0x93 => I2s,
            // 0x60 => Iadd,
            // 0x2e => Iaload,
            // 0x7e => Iand,
            // 0x4f => Iastore,
            // 0x02 => IconstM1,
            0x03 => (Iconst0, 0u8),
            // 0x04 => Iconst1,
            // 0x05 => Iconst2,
            // 0x06 => Iconst3,
            // 0x07 => Iconst4,
            // 0x08 => Iconst5,
            // 0x6c => Idiv,
            // 0xa5 => IfAcmpEq,
            // 0xa6 => IfAcmpNe,
            // 0x9f => IfIcmpEq,
            // 0xa0 => IfIcmpNe,
            // 0xa1 => IfIcmpLt,
            // 0xa2 => IfIcmpGe,
            // 0xa3 => IfIcmpGt,
            // 0xa4 => IfIcmpLe,
            // 0x99 => IfEq,
            // 0x9a => IfNe,
            // 0x9b => IfLt,
            // 0x9c => IfGe,
            // 0x9d => IfGt,
            // 0x9e => IfLe,
            // 0xc7 => IfNonNull,
            // 0xc6 => IfNull,
            0x84 => (Iinc, 2),
            // 0x15 => Iload,
            // 0x1a => Iload0,
            // 0x1b => Iload1,
            // 0x1c => Iload2,
            // 0x1d => Iload3,
            // 0x68 => Imul,
            // 0x74 => Ineg,
            // 0xc1 => Instanceof,
            // 0xba => Invokedynamic,
            // 0xb9 => Invokeinterface,
            // 0xb7 => Invokespecial,
            // 0xb8 => Invokestatic,
            // 0xb6 => Invokevirtual,
            // 0x80 => Ior,
            // 0x70 => Irem,
            // 0xac => Ireturn,
            // 0x78 => Ishl,
            // 0x7a => Ishr,
            // 0x36 => Istore,
            // 0x3b => Istore0,
            0x3c => (Istore1, 0),
            // 0x3d => Istore2,
            // 0x3e => Istore3,
            // 0x64 => Isub,
            // 0x7c => Iushr,
            // 0x82 => Ixor,
            // 0xa8 => Jsr,
            // 0xc9 => JsrW,
            // 0x8a => L2d,
            // 0x89 => L2f,
            // 0x88 => L2i,
            // 0x61 => Ladd,
            // 0x2f => Laload,
            // 0x7f => Land,
            // 0x50 => Lastore,
            // 0x94 => Lcmp,
            // 0x09 => Lconst0,
            // 0x0a => Lconst1,
            // 0x12 => Ldc,
            // 0x13 => LdcW,
            // 0x14 => Ldc2W,
            // 0x6d => Ldiv,
            // 0x16 => Lload,
            // 0x1e => Lload0,
            // 0x1f => Lload1,
            // 0x20 => Lload2,
            // 0x21 => Lload3,
            // 0x69 => Lmul,
            // 0x75 => Lneg,
            // 0xab => LookupSwitch,
            // 0x81 => Lor,
            // 0x71 => Lrem,
            // 0xad => Lreturn,
            // 0x79 => Lshl,
            // 0x7b => Lshr,
            // 0x37 => Lstore,
            // 0x3f => Lstore0,
            // 0x40 => Lstore1,
            // 0x41 => Lstore2,
            // 0x42 => Lstore3,
            // 0x65 => Lsub,
            // 0x7d => Lushr,
            // 0x83 => Lxor,
            // 0xc2 => MonitorEnter,
            // 0xc3 => MonitorExit,
            // 0xc5 => Multianewarray,
            // 0xbb => New,
            // 0xbc => NewArray,
            // 0x00 => Nop,
            // 0x57 => Pop,
            // 0x58 => Pop2,
            // 0xb5 => PutField,
            // 0xb3 => PutStatic,
            // 0xa9 => Ret,
            0xb1 => (Return, 0),
            // 0x35 => Saload,
            // 0x56 => Sastore,
            // 0x11 => Sipush,
            // 0x5f => Swap,
            // 0xaa => TableSwitch,
            // 0xc4 => Wide,
            // 0xfe => ImpDep1,
            // 0xff => ImpDep2,
            // 0xca => BreakPoint
            x => panic!("Unknown opcode {:x?}", x),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::class::code::Instruction;
    use crate::class::code::Opcode::*;
    use crate::io::code::CodeReader;
    use std::io::Cursor;

    #[test]
    fn read_instructions() {
        let mut data = Cursor::new(vec![
            0x00, 0x06, // Length
            0x03, // iconst_0
            0x3c, // istore_1
            0x84, 0x01, 0x01, // iinc 1, 1
            0xb1, // return
        ]);

        let mut reader = CodeReader::new(&mut data);
        let instructions = reader.read_code().unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new(Iconst0, vec![]),
                Instruction::new(Istore1, vec![]),
                Instruction::new(Iinc, vec![1, 1]),
                Instruction::new(Return, vec![]),
            ]
        );
    }
}
