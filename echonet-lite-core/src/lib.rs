//! This crate is pure Rust ECHONET Lite implementation including
//! - serde implementation of ECHONET Lite packet
//! - detailed property configurations of ECHONET Device objects (WIP)
//!
//! but not included
//! - transport layer (usually, UDP with IPv4/IPv6) implementation
//! - specific ECHONET Lite object behavior

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

/// All the types we need from `std`, `core`, and `alloc` crates.
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::clone::{self, Clone};
    pub use self::core::convert::{self, From, Into};
    pub use self::core::default::{self, Default};
    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::result::{self, Result};
    pub use self::core::{ops, str};

    #[cfg(not(feature = "std"))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString};

    #[cfg(not(feature = "std"))]
    pub use alloc::vec::{self, Vec};
    #[cfg(feature = "std")]
    pub use std::vec::{self, Vec};

    #[cfg(not(feature = "std"))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(feature = "std")]
    pub use std::error;

    #[cfg(feature = "std")]
    pub use std::io;
}

mod el_packet;
mod error;
mod io;

mod de;
mod ser;
pub use de::deserialize;
pub use ser::serialize;
pub mod object;
pub mod prelude;
pub use el_packet::*;
pub use error::{Error, ErrorKind, Result};
