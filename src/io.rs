//! Use std::io from `std` when available, otherwise use bare_io as io.

pub use self::imp::{Error, ErrorKind, Read, Result, Write};

#[cfg(not(feature = "std"))]
use bare_io as imp;

#[cfg(feature = "std")]
use crate::lib::io as imp;
