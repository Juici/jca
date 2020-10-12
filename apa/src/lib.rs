//! An arbitrary-precision arithmetic library.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

mod apint;
mod convert;
mod limb;
mod mem;

pub use crate::apint::ApInt;
