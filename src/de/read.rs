use bare_io as io;
use core::ptr::copy_nonoverlapping;
use io::Result;

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => {{
        assert!($size == ::core::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping($src.as_ptr(), &mut data as *mut $ty as *mut u8, $size);
        }
        data.$which()
    }};
}

pub trait ReadBytesExt: io::Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(read_num_bytes!(u16, 2, buf, to_be))
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(read_num_bytes!(i16, 2, buf, to_be))
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(read_num_bytes!(u32, 4, buf, to_be))
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(read_num_bytes!(i32, 4, buf, to_be))
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}
