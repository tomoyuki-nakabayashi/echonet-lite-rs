#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod error;
mod ser;
mod de;

pub mod el_packet;
pub use error::{Error, ErrorKind, Result};
