//! An arbitrary-precision arithmetic library.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

mod alloc;
mod apint;
mod limb;
mod mem;

pub use crate::apint::ApInt;
