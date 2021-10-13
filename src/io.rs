//! Use std::io from `std` when available, otherwise use bare_io as io.

#[cfg(not(feature = "std"))]
pub use bare_io as io;

#[cfg(feature = "std")]
pub use crate::lib::io;
