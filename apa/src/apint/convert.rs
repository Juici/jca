use core::num::NonZeroUsize;

use crate::apint::{ApInt, ApIntStorage};
use crate::limb::{Limb, LimbRepr};

/// Store limbs in native endian order to make primitive casts quicker.
#[inline]
fn limb_order(len: usize) -> impl Iterator<Item = usize> {
    let iter = 0..len;
    #[cfg(target_endian = "big")]
    let iter = iter.rev();
    iter
}

macro_rules! impl_from_prim {
    (unsigned: $($ty:ty),* $(,)?) => {
        $(
            impl core::convert::From<$ty> for ApInt {
                fn from(val: $ty) -> ApInt {
                    const SIZE_TY: usize = core::mem::size_of::<$ty>();
                    const SIZE_LIMB: usize = Limb::SIZE;

                    const BITS_TY: usize = SIZE_TY * 8;
                    const BITS_LIMB: usize = Limb::BITS;

                    const FITS: bool = SIZE_TY < SIZE_LIMB;

                    // The number of bits actually required to hold the value.
                    let bits_val = BITS_TY - (val.leading_zeros() as usize);

                    // Check if the value fits, or can be truncated to fit.
                    if FITS || bits_val < BITS_LIMB {
                        ApInt::from_limb(Limb(val as LimbRepr))
                    } else {
                        const MASK: $ty = !(0 as LimbRepr) as $ty;

                        // Equivalent to `ceil((bits_val + 1) / BITS_LIMB)`.
                        let capacity = (bits_val / BITS_LIMB) + 1;
                        // SAFETY: `factor + 1` is guaranteed to be greater than 1.
                        let capacity = unsafe { NonZeroUsize::new_unchecked(capacity) };

                        let mut int = ApInt::with_capacity(capacity);

                        // If sizes are equal don't include last limb. This is hacky,
                        // due to the nature of non-standard bit-shifts across platforms.
                        let iter_to = capacity.get() - (SIZE_TY == SIZE_LIMB) as usize;

                        let mut val = val;
                        for i in limb_order(iter_to) {
                            // The value of the limb.
                            let limb = val & MASK;

                            // SAFETY: `i` is guaranteed to be a valid limb offset.
                            unsafe { *int.limb_mut(i) = Limb(limb as LimbRepr) };

                            // Should never wrap.
                            val = val.wrapping_shr(BITS_LIMB as u32);
                        }

                        int
                    }
                }
            }
        )*
    };
    (signed: $($ty:ty),* $(,)?) => {
        $(
            impl core::convert::From<$ty> for ApInt {
                fn from(val: $ty) -> ApInt {
                    const SIZE_TY: usize = core::mem::size_of::<$ty>();
                    const SIZE_LIMB: usize = Limb::SIZE;

                    const BITS_TY: usize = SIZE_TY * 8;
                    const BITS_LIMB: usize = Limb::BITS;

                    const FITS: bool = SIZE_TY < SIZE_LIMB;

                    const SHIFT_TY: usize = BITS_TY - 1;
                    const SIGN_TY: $ty = 1 << SHIFT_TY;

                    let abs_val = val & !SIGN_TY;
                    let leading = (val.leading_zeros() + val.leading_ones()) as usize;

                    // The number of bits actually required to hold the absolute value plus
                    // an additional sign bit.
                    let bits_val = BITS_TY - leading;

                    // Check if the value fits, or can be truncated to fit.
                    if FITS || bits_val <= BITS_LIMB {
                        // Apply sign bit to limb.
                        let sign_limb = (val & SIGN_TY) as LimbRepr;
                        let limb = (abs_val as LimbRepr) | sign_limb;

                        ApInt::from_limb(Limb(limb))
                    } else {
                        const MASK: $ty = !(0 as LimbRepr) as $ty;

                        // Equivalent to `ceil(bits_val / BITS_LIMB)`.
                        let capacity = {
                            let q = bits_val / BITS_LIMB;
                            let r = bits_val % BITS_LIMB;
                            q + ((r != 0) as usize)
                        };
                        // SAFETY: `factor` is guaranteed to be greater than 1,
                        //          since `bits_val` >= `BITS_LIMB`.
                        let capacity = unsafe { NonZeroUsize::new_unchecked(capacity) };

                        let mut int = ApInt::with_capacity(capacity);

                        let mut val = val;
                        for i in limb_order(capacity.get()) {
                            // The value of the limb.
                            let limb = val & MASK;

                            // SAFETY: `i` is guaranteed to be a valid limb offset.
                            unsafe { *int.limb_mut(i) = Limb(limb as LimbRepr) };

                            // Should never wrap.
                            val = val.wrapping_shr(BITS_LIMB as u32);
                        }

                        int
                    }
                }
            }
        )*
    };
}

impl_from_prim!(unsigned: u8, u16, u32, u64, u128, usize);
impl_from_prim!(signed: i8, i16, i32, i64, i128, isize);

macro_rules! impl_to_prim {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'a> core::convert::From<&'a ApInt> for $ty {
                fn from(int: &'a ApInt) -> $ty {
                    const SIZE_TY: usize = core::mem::size_of::<$ty>();
                    const SIZE_LIMB: usize = Limb::SIZE;

                    const BITS_LIMB: usize = Limb::BITS;

                    unsafe {
                        match int.storage() {
                            ApIntStorage::Stack(limb) => limb.repr() as $ty,
                            ApIntStorage::Heap(ptr) if SIZE_TY <= SIZE_LIMB * int.len.get() => *ptr.cast().as_ptr(),
                            ApIntStorage::Heap(ptr) => {
                                // The number of limbs that fit in $ty.
                                const FACTOR: usize = SIZE_TY / SIZE_LIMB;
                                // Copy only as many limbs as we have.
                                let n_copy = int.len.get().min(FACTOR);

                                let mut val = 0;
                                for (i, j) in limb_order(n_copy).enumerate() {
                                    let limb = *ptr.as_ptr().add(j);
                                    let limb = limb.repr() as $ty;
                                    val |= limb.wrapping_shl((BITS_LIMB * i) as u32);
                                }

                                val
                            }
                        }
                    }
                }
            }

            impl core::convert::From<ApInt> for $ty {
                #[inline]
                fn from(int: ApInt) -> $ty {
                    <$ty>::from(&int)
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_to_prim!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);
