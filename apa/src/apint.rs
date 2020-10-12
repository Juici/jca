use core::num::NonZeroUsize;
use core::ptr::NonNull;

use crate::limb::Limb;
use crate::mem;

/// An arbitrary-precision integer.
pub struct ApInt {
    /// The number of limbs used to store data.
    len: NonZeroUsize,
    /// The data holding the bits of the integer.
    data: ApIntData,
}

/// A single stack allocated limb or pointer to heap allocated limbs.
union ApIntData {
    /// Inlined storage for values able to be stored within a single machine word.
    value: Limb,
    /// Heap allocated storage for values unable to be stored within a single machine word.
    p_value: NonNull<Limb>,
}

// `ApInt` can safely be sent across thread boundaries, since it does not own
// aliasing memory and has no reference counting mechanism.
unsafe impl Send for ApInt {}
// `ApInt` can safely be shared between threads, since it does not own
// aliasing memory and has no mutable internal state.
unsafe impl Sync for ApInt {}

impl ApInt {
    /// Represents an `ApInt` with value `0`.
    pub const ZERO: ApInt = ApInt::from_limb(Limb::ZERO);
    /// Represents an `ApInt` with value `1`.
    pub const ONE: ApInt = ApInt::from_limb(Limb::ONE);

    /// Creates an `ApInt` with a single limb.
    pub(crate) const fn from_limb(value: Limb) -> ApInt {
        ApInt {
            // SAFETY: 1 is guaranteed to be non-zero.
            len: unsafe { NonZeroUsize::new_unchecked(1) },
            data: ApIntData { value },
        }
    }

    /// Creates an `ApInt` with space allocated for the given capacity.
    ///
    /// Data is zeroed.
    ///
    /// # Safety
    ///
    /// Calling this function with a capacity of `1` will result in undefined
    /// behaviour.
    pub(crate) fn with_capacity(capacity: NonZeroUsize) -> ApInt {
        // Sanity check when testing. Since this is an internal function we
        // should be able to guarantee it is never called with a capacity of 1.
        debug_assert!(
            capacity.get() > 1,
            "allocating `ApInt` with capacity 1 is not supported"
        );

        let p_value = mem::alloc_limbs(capacity);
        ApInt {
            len: capacity,
            data: ApIntData { p_value },
        }
    }

    /// Returns the number of limbs used to represent the integer.
    #[inline]
    pub(crate) fn len(&self) -> NonZeroUsize {
        self.len
    }

    /// Returns the limb at the given offset.
    pub(crate) unsafe fn limb(&self, offset: usize) -> Limb {
        match self.len.get() {
            1 => self.data.value,
            _ => *self.data.p_value.as_ptr().add(offset),
        }
    }

    /// Returns a mutable reference to the limb at the given offset.
    pub(crate) unsafe fn limb_mut(&mut self, offset: usize) -> &mut Limb {
        match self.len.get() {
            1 => &mut self.data.value,
            _ => &mut *self.data.p_value.as_ptr().add(offset),
        }
    }
}

impl Drop for ApInt {
    fn drop(&mut self) {
        if self.len.get() > 1 {
            // SAFETY: `p_value` is guaranteed to be a valid pointer, since `len` > 1.
            let p_value = unsafe { self.data.p_value };
            mem::dealloc_limbs(p_value, self.len);
        }
    }
}
