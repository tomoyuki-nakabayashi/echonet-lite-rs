//! This crate is pure Rust ECHONET Lite implementation including
//! - serde implementation of ECHONET Lite packet
//! - detailed property configurations of ECHONET Device objects (WIP)
//!
//! but not included
//! - transport layer (usually, UDP with IPv4/IPv6) implementation
//! - specific ECHONET Lite object behavior

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
