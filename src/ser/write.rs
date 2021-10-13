use crate::io::io;
use io::Result;

pub trait WriteBytesExt: io::Write {
    #[inline]
    fn write_u8(&mut self, n: u8) -> Result<()> {
        self.write_all(&[n])
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> Result<()> {
        self.write_all(&[n as u8])
    }

    #[inline]
    fn write_u16(&mut self, n: u16) -> Result<()> {
        let buf = u16::to_be_bytes(n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i16(&mut self, n: i16) -> Result<()> {
        let buf = i16::to_be_bytes(n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_u32(&mut self, n: u32) -> Result<()> {
        let buf = u32::to_be_bytes(n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i32(&mut self, n: i32) -> Result<()> {
        let buf = i32::to_be_bytes(n);
        self.write_all(&buf)
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt for W {}
