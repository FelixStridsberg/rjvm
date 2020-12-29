use crate::binary::bytes_to_i32;
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
        let byte_count = self.reader.read_u4()?;
        let mut byte_pos = 0;
        let mut code = Vec::with_capacity(byte_count as usize);

        loop {
            let (opcode, argc) = self.read_opcode()?;
            let (mut instructions, byte_len) = match opcode {
                LookupSwitch => self.read_lookup_switch(opcode, byte_pos + 1)?,
                TableSwitch => self.read_table_switch(opcode, byte_pos + 1)?,
                _ => self.read_static_width_instruction(opcode, argc)?,
            };
            code.append(&mut instructions);

            byte_pos += byte_len;
            if byte_count == byte_pos {
                break;
            }
        }
        Ok(code)
    }

    fn read_static_width_instruction(
        &mut self,
        opcode: Opcode,
        argc: u8,
    ) -> Result<(Vec<Instruction>, u32)> {
        let mut code = Vec::new();

        let operands = self.reader.read_bytes(argc as usize)?;
        code.push(Instruction::new(opcode, operands));

        // Must add spacers to keep the indexes correct.
        for _ in 0..argc {
            code.push(Instruction::operation_spacer());
        }

        Ok((code, 1 + argc as u32))
    }

    fn read_lookup_switch(
        &mut self,
        opcode: Opcode,
        byte_pos: u32,
    ) -> Result<(Vec<Instruction>, u32)> {
        let mut operands = Vec::new();
        let pad = (4 - byte_pos % 4) % 4;

        self.reader.read_bytes(pad as usize)?; // Skip padding

        let mut default_jump_bytes = self.reader.read_bytes(4)?;
        let mut num_pairs_bytes = self.reader.read_bytes(4)?;

        let num_pairs = bytes_to_i32(&num_pairs_bytes[..]);

        let mut match_offset_pairs = self.reader.read_bytes(num_pairs as usize * 8)?;

        operands.append(&mut default_jump_bytes);
        operands.append(&mut num_pairs_bytes);
        operands.append(&mut match_offset_pairs);

        let byte_len = 1 + operands.len() as u32 + pad;

        let mut code = Vec::new();
        code.push(Instruction::new_with_pad(opcode, operands, pad as u8));

        // Must add spacers to keep the indexes correct.
        for _ in 0..byte_len {
            code.push(Instruction::operation_spacer());
        }

        Ok((code, byte_len))
    }

    fn read_table_switch(
        &mut self,
        opcode: Opcode,
        byte_pos: u32,
    ) -> Result<(Vec<Instruction>, u32)> {
        let mut operands = Vec::new();
        let pad = (4 - byte_pos % 4) % 4;

        self.reader.read_bytes(pad as usize)?; // Skip padding

        let mut default_jump_bytes = self.reader.read_bytes(4)?;
        let mut low_byte_bytes = self.reader.read_bytes(4)?;
        let mut high_byte_bytes = self.reader.read_bytes(4)?;

        let high = bytes_to_i32(&high_byte_bytes[..]);
        let low = bytes_to_i32(&low_byte_bytes[..]);

        let len = high - low + 1;
        let mut jump_offset_bytes = self.reader.read_bytes(len as usize * 4)?;

        operands.append(&mut default_jump_bytes);
        operands.append(&mut low_byte_bytes);
        operands.append(&mut high_byte_bytes);
        operands.append(&mut jump_offset_bytes);

        let byte_len = 1 + operands.len() as u32 + pad;

        let mut code = Vec::new();
        code.push(Instruction::new_with_pad(opcode, operands, pad as u8));

        // Must add spacers to keep the indexes correct.
        for _ in 0..byte_len {
            code.push(Instruction::operation_spacer());
        }

        Ok((code, byte_len))
    }

    fn read_opcode(&mut self) -> Result<(Opcode, u8)> {
        Ok(match self.reader.read_u1()? {
            0x32 => (AaLoad, 0),
            0x53 => (AaStore, 0),
            0x01 => (AConstNull, 0),
            0x19 => (ALoad, 1),
            0x2a => (ALoad0, 0),
            0x2b => (ALoad1, 0),
            0x2c => (ALoad2, 0),
            0x2d => (ALoad3, 0),
            0xbd => (ANewArray, 2),
            0xb0 => (AReturn, 0),
            0xbe => (ArrayLength, 0),
            0x3a => (AStore, 1),
            0x4b => (AStore0, 0),
            0x4c => (AStore1, 0),
            0x4d => (AStore2, 0),
            0x4e => (AStore3, 0),
            0xbf => (AThrow, 0),
            0x33 => (BaLoad, 0),
            0x54 => (BaStore, 0),
            0x10 => (BiPush, 1),
            0x34 => (CaLoad, 0),
            0x55 => (CaStore, 0),
            0xc0 => (CheckCast, 2),
            0x90 => (D2f, 0),
            0x8e => (D2i, 0),
            0x8f => (D2l, 0),
            0x63 => (DAdd, 0),
            0x31 => (DaLoad, 0),
            0x52 => (DaStore, 0),
            0x98 => (DCmpg, 0),
            0x97 => (DCmpl, 0),
            0x0e => (DConst0, 0),
            0x0f => (DConst1, 0),
            0x6f => (DDiv, 0),
            0x18 => (DLoad, 1),
            0x26 => (DLoad0, 0),
            0x27 => (DLoad1, 0),
            0x28 => (DLoad2, 0),
            0x29 => (DLoad3, 0),
            0x6b => (DMul, 0),
            0x77 => (DNeg, 0),
            0x73 => (DRem, 0),
            0xaf => (DReturn, 0),
            0x39 => (DStore, 1),
            0x47 => (DStore0, 0),
            0x48 => (DStore1, 0),
            0x49 => (DStore2, 0),
            0x4a => (DStore3, 0),
            0x67 => (DSub, 0),
            0x59 => (Dup, 0),
            0x5a => (DupX1, 0),
            0x5b => (DupX2, 0),
            0x5c => (Dup2, 0),
            0x5d => (Dup2X1, 0),
            0x5e => (Dup2X2, 0),
            0x8d => (F2d, 0),
            0x8b => (F2i, 0),
            0x8c => (F2l, 0),
            0x62 => (FAdd, 0),
            0x30 => (FaLoad, 0),
            0x51 => (FaStore, 0),
            0x96 => (FCmpg, 0),
            0x95 => (FCmpl, 0),
            0x0b => (FConst0, 0),
            0x0c => (FConst1, 0),
            0x0d => (FConst2, 0),
            0x6e => (FDiv, 0),
            0x17 => (FLoad, 1),
            0x22 => (FLoad0, 0),
            0x23 => (FLoad1, 0),
            0x24 => (FLoad2, 0),
            0x25 => (FLoad3, 0),
            0x6a => (FMul, 0),
            0x76 => (FNeg, 0),
            0x72 => (FRem, 0),
            0xae => (FReturn, 0),
            0x38 => (FStore, 1),
            0x43 => (FStore0, 0),
            0x44 => (FStore1, 0),
            0x45 => (FStore2, 0),
            0x46 => (FStore3, 0),
            0x66 => (FSub, 0),
            0xb4 => (GetField, 2),
            0xb2 => (GetStatic, 2),
            0xa7 => (Goto, 2),
            0xc8 => (GotoW, 4),
            0x91 => (I2b, 0),
            0x92 => (I2c, 0),
            0x87 => (I2d, 0),
            0x86 => (I2f, 0),
            0x85 => (I2l, 0),
            0x93 => (I2s, 0),
            0x60 => (IAdd, 0),
            0x2e => (IaLoad, 0),
            0x7e => (IAnd, 0),
            0x4f => (IaStore, 0),
            0x02 => (IConstM1, 0),
            0x03 => (IConst0, 0),
            0x04 => (IConst1, 0),
            0x05 => (IConst2, 0),
            0x06 => (IConst3, 0),
            0x07 => (IConst4, 0),
            0x08 => (IConst5, 0),
            0x6c => (IDiv, 0),
            0xa5 => (IfAcmpEq, 2),
            0xa6 => (IfAcmpNe, 2),
            0x9f => (IfIcmpEq, 2),
            0xa0 => (IfIcmpNe, 2),
            0xa1 => (IfIcmpLt, 2),
            0xa2 => (IfIcmpGe, 2),
            0xa3 => (IfIcmpGt, 2),
            0xa4 => (IfIcmpLe, 2),
            0x99 => (IfEq, 2),
            0x9a => (IfNe, 2),
            0x9b => (IfLt, 2),
            0x9c => (IfGe, 2),
            0x9d => (IfGt, 2),
            0x9e => (IfLe, 2),
            0xc7 => (IfNonNull, 2),
            0xc6 => (IfNull, 2),
            0x84 => (IInc, 2),
            0x15 => (ILoad, 1),
            0x1a => (ILoad0, 0),
            0x1b => (ILoad1, 0),
            0x1c => (ILoad2, 0),
            0x1d => (ILoad3, 0),
            0x68 => (IMul, 0),
            0x74 => (INeg, 0),
            0xc1 => (Instanceof, 2),
            0xba => (InvokeDynamic, 4),
            0xb9 => (InvokeInterface, 4),
            0xb7 => (InvokeSpecial, 2),
            0xb8 => (InvokeStatic, 2),
            0xb6 => (InvokeVirtual, 2),
            0x80 => (IOr, 0),
            0x70 => (IRem, 0),
            0xac => (IReturn, 0),
            0x78 => (IShl, 0),
            0x7a => (IShr, 0),
            0x36 => (IStore, 1),
            0x3b => (IStore0, 0),
            0x3c => (IStore1, 0),
            0x3d => (IStore2, 0),
            0x3e => (IStore3, 0),
            0x64 => (Isub, 0),
            0x7c => (IUshr, 0),
            0x82 => (Ixor, 0),
            0xa8 => (Jsr, 2),
            0xc9 => (JsrW, 4),
            0x8a => (L2d, 0),
            0x89 => (L2f, 0),
            0x88 => (L2i, 0),
            0x61 => (LAdd, 0),
            0x2f => (LaLoad, 0),
            0x7f => (LAnd, 0),
            0x50 => (LaStore, 0),
            0x94 => (LCmp, 0),
            0x09 => (LConst0, 0),
            0x0a => (LConst1, 0),
            0x12 => (Ldc, 1),
            0x13 => (LdcW, 2),
            0x14 => (Ldc2W, 2),
            0x6d => (LDiv, 0),
            0x16 => (LLoad, 1),
            0x1e => (LLoad0, 0),
            0x1f => (LLoad1, 0),
            0x20 => (LLoad2, 0),
            0x21 => (LLoad3, 0),
            0x69 => (LMul, 0),
            0x75 => (LNeg, 0),
            0xab => (LookupSwitch, 0), // Variable width
            0x81 => (LOr, 0),
            0x71 => (LRem, 0),
            0xad => (LReturn, 0),
            0x79 => (LShl, 0),
            0x7b => (LShr, 0),
            0x37 => (LStore, 1),
            0x3f => (LStore0, 0),
            0x40 => (LStore1, 0),
            0x41 => (LStore2, 0),
            0x42 => (LStore3, 0),
            0x65 => (LSub, 0),
            0x7d => (LUshr, 0),
            0x83 => (LXor, 0),
            0xc2 => (MonitorEnter, 0),
            0xc3 => (MonitorExit, 0),
            0xc5 => (MultiANewArray, 3),
            0xbb => (New, 2),
            0xbc => (NewArray, 1),
            0x00 => (Nop, 0),
            0x57 => (Pop, 0),
            0x58 => (Pop2, 0),
            0xb5 => (PutField, 2),
            0xb3 => (PutStatic, 2),
            0xa9 => (Ret, 1),
            0xb1 => (Return, 0),
            0x35 => (SaLoad, 0),
            0x56 => (SaStore, 0),
            0x11 => (SiPush, 2),
            0x5f => (Swap, 0),
            0xaa => (TableSwitch, 0),
            // 0xc4 => (Wide, opcode specific length),
            0xca => (BreakPoint, 0),
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
            0x00, 0x00, 0x00, 0x06, // Length
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
                Instruction::new(IConst0, vec![]),
                Instruction::new(IStore1, vec![]),
                Instruction::new(IInc, vec![1, 1]),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::new(Return, vec![]),
            ]
        );
    }

    #[test]
    fn read_lookup_switch() {
        let mut data = Cursor::new(vec![
            0x00, 0x00, 0x00, 0x14, // Length
            0xab, // Opcode
            0x00, 0x00, 0x00, // Padding
            0x00, 0x00, 0x00, 0x01, // Default
            0x00, 0x00, 0x00, 0x01, // npairs
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, // match offset pair
        ]);

        let mut reader = CodeReader::new(&mut data);
        let instructions = reader.read_code().unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new_with_pad(
                    LookupSwitch,
                    vec![
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
                        0x00, 0x00, 0x00, 0x02
                    ],
                    3
                ),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
            ]
        );
    }

    #[test]
    fn read_table_switch() {
        let mut data = Cursor::new(vec![
            0x00, 0x00, 0x00, 0x14, // Length
            0xaa, // Opcode
            0x00, 0x00, 0x00, // Padding
            0x00, 0x00, 0x00, 0x01, // Default
            0x00, 0x00, 0x00, 0x01, // low
            0x00, 0x00, 0x00, 0x01, // high
            0x00, 0x00, 0x00, 0x03, // Jump offset
        ]);

        let mut reader = CodeReader::new(&mut data);
        let instructions = reader.read_code().unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::new_with_pad(
                    TableSwitch,
                    vec![
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x00, 0x03
                    ],
                    3
                ),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
                Instruction::operation_spacer(),
            ]
        );
    }
}
