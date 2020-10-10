use core::ptr::NonNull;

use crate::bitwidth::BitWidth;
use crate::limb::Limb;

/// An arbitrary-precision integer.
pub struct ApInt {
    /// The number of bits used in the data.
    len: BitWidth,
    /// The data holding the bits of the integer.
    data: ApIntData,
}

union ApIntData {
    /// Inlined storage for values able to be stored within a single machine word.
    value: Limb,
    /// Heap allocated storage for values unable to be stored within a single machine word.
    p_value: NonNull<Limb>,
}

/// `ApInt` can safely be sent across thread boundaries, since it does not own
/// aliasing memory and has no reference counting mechanism.
unsafe impl Send for ApInt {}
/// `ApInt` can safely be shared between threads, since it does not own
/// aliasing memory and has no mutable internal state.
unsafe impl Sync for ApInt {}
