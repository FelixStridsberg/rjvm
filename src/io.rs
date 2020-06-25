pub mod class;
mod attribute;

use crate::error::Result;

trait ReadBytesExt: std::io::Read {
    #[inline]
    fn read_u1(&mut self) -> Result<u8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    #[inline]
    fn read_u2(&mut self) -> Result<u16> {
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)?;
        Ok((bytes[0] as u16) << 8 | bytes[1] as u16)
    }

    #[inline]
    fn read_u4(&mut self) -> Result<u32> {
        let u16_0 = self.read_u2()? as u32;
        let u16_1 = self.read_u2()? as u32;
        Ok(u16_0 << 16 | u16_1)
    }
}

impl<R: std::io::Read + ?Sized> ReadBytesExt for R {}
