#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod error;

pub mod el_packet;
pub mod de_echonet_lite;
pub use error::{Error, ErrorKind, Result};
