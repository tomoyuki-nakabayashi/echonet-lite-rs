#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod de;
mod el_packet;
mod error;
mod object;
mod ser;

pub mod prelude;
pub use el_packet::*;
pub use error::{Error, ErrorKind, Result};
