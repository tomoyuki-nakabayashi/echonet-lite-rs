use bare_io as io;
use bare_io::Result;
use core::mem::transmute;
use core::ptr::copy_nonoverlapping;

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => {{
        assert!($size <= $dst.len());
        unsafe {
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    }};
}

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
        let mut buf = [0; 2];
        write_num_bytes!(u16, 2, n, buf, to_be);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i16(&mut self, n: i16) -> Result<()> {
        let mut buf = [0; 2];
        write_num_bytes!(i16, 2, n, buf, to_be);
        self.write_all(&buf)
    }

    #[inline]
    fn write_u32(&mut self, n: u32) -> Result<()> {
        let mut buf = [0; 4];
        write_num_bytes!(u32, 4, n, buf, to_be);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i32(&mut self, n: i32) -> Result<()> {
        let mut buf = [0; 4];
        write_num_bytes!(i32, 4, n, buf, to_be);
        self.write_all(&buf)
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt for W {}
