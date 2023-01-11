//! Use std::io from `std` when available, otherwise use core2 as io.

pub use self::imp::{Error, ErrorKind, Read, Result, Write};

#[cfg(not(feature = "std"))]
use core2 as imp;

#[cfg(feature = "std")]
use crate::lib::io as imp;
