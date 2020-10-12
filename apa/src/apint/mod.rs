use core::num::NonZeroUsize;
use core::ptr::NonNull;

use crate::limb::Limb;
use crate::mem;

mod convert;

/// An arbitrary-precision integer.
pub struct ApInt {
    /// The number of limbs used to store data.
    len: NonZeroUsize,
    /// The data holding the bits of the integer.
    data: ApIntData,
}

enum ApIntStorage<'a> {
    Stack(Limb),
    Heap(&'a NonNull<Limb>),
}

enum ApIntStorageMut<'a> {
    Stack(&'a mut Limb),
    Heap(&'a mut NonNull<Limb>),
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
    const fn from_limb(value: Limb) -> ApInt {
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
    fn with_capacity(capacity: NonZeroUsize) -> ApInt {
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

    // TODO: Replace with actual API.

    /// Returns a storage accessor for the limb data.
    fn storage(&self) -> ApIntStorage {
        match self.len.get() {
            // SAFETY: The len is non-zero.
            0 => unsafe { core::hint::unreachable_unchecked() },
            // SAFETY: A len of 1 guarantees that value is a valid limb.
            1 => ApIntStorage::Stack(unsafe { self.data.value }),
            // SAFETY: A len greater than 1 guarantees that p_value is a valid pointer.
            _ => ApIntStorage::Heap(unsafe { &self.data.p_value }),
        }
    }

    /// Returns a mutable storage accessor for the limb data.
    fn storage_mut(&mut self) -> ApIntStorageMut {
        match self.len.get() {
            // SAFETY: The len is non-zero.
            0 => unsafe { core::hint::unreachable_unchecked() },
            // SAFETY: A len of 1 guarantees that value is a valid limb.
            1 => ApIntStorageMut::Stack(unsafe { &mut self.data.value }),
            // SAFETY: A len greater than 1 guarantees that p_value is a valid pointer.
            _ => ApIntStorageMut::Heap(unsafe { &mut self.data.p_value }),
        }
    }

    // TODO: Add proper limb accessor/iterator.

    /// Returns the limb at the given index.
    pub(crate) unsafe fn limb(&self, index: usize) -> Limb {
        match self.storage() {
            ApIntStorage::Stack(limb) => limb,
            ApIntStorage::Heap(ptr) => *ptr.as_ptr().add(index),
        }
    }

    /// Returns a mutable reference to the limb at the given index.
    pub(crate) unsafe fn limb_mut(&mut self, index: usize) -> &mut Limb {
        match self.storage_mut() {
            ApIntStorageMut::Stack(limb) => limb,
            ApIntStorageMut::Heap(ptr) => &mut *ptr.as_ptr().add(index),
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
