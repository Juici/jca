use core::num::NonZeroUsize;

use crate::apint::ApInt;
use crate::limb::{Limb, LimbRepr};

macro_rules! impl_from_prim {
    (unsigned: $($ty:ty),*) => {
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
                        let value = Limb(val as LimbRepr);

                        ApInt::from_limb(value)
                    } else {
                        const MASK: $ty = !(0 as LimbRepr) as $ty;

                        let factor = bits_val / BITS_LIMB;
                        // SAFETY: `capacity` is guaranteed to be greater than 1.
                        let capacity = unsafe { NonZeroUsize::new_unchecked(factor + 1) };

                        let mut int = ApInt::with_capacity(capacity);

                        let mut val = val;
                        for i in 1..capacity.get() {
                            // The value of the limb.
                            let limb_val = val & MASK;

                            // SAFETY: `i` is guaranteed to be a valid limb offset.
                            unsafe { *int.limb_mut(i) = Limb(limb_val as LimbRepr) };

                            // Shift off limb bits from val.
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
// impl_from_prim!(signed: i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_prim {
        ($value:expr, $limbs:expr, [$($ty:ty),*]) => {
            let limbs: &[LimbRepr] = $limbs;
            $(
                {
                    let int = ApInt::from($value as $ty);
                    assert_eq!(int.len().get(), limbs.len());

                    for i in 0..limbs.len() {
                        unsafe {
                            assert_eq!(int.limb(i).repr(), limbs[i]);
                        }
                    }
                }
            )*
        };
    }

    #[test]
    fn from_u_0() {
        test_prim!(0, &[0], [u8, u16, u32, u64, u128, usize]);
    }

    #[test]
    fn from_u8_max() {
        const MAX: LimbRepr = u8::MAX as LimbRepr;

        test_prim!(u8::MAX, &[MAX], [u8, u16, u32, u64, u128, usize]);
    }

    #[test]
    fn from_u16_max() {
        const MAX: LimbRepr = u16::MAX as LimbRepr;

        test_prim!(u16::MAX, &[MAX], [u16, u32, u64, u128, usize]);
    }

    #[test]
    fn from_u32_max() {
        const MAX: LimbRepr = u32::MAX as LimbRepr;

        #[cfg(target_pointer_width = "32")]
        test_prim!(u32::MAX, &[0, MAX], [u32, u64, u128, usize]);
        #[cfg(target_pointer_width = "64")]
        test_prim!(u32::MAX, &[MAX], [u32, u64, u128, usize]);
    }

    #[test]
    fn from_u64_max() {
        #[cfg(target_pointer_width = "32")]
        test_prim!(u64::MAX, &[0, LimbRepr::MAX, LimbRepr::MAX], [u64, u128]);
        #[cfg(target_pointer_width = "64")]
        test_prim!(u64::MAX, &[0, LimbRepr::MAX], [u64, u128, usize]);
    }

    #[test]
    fn from_u128_max() {
        #[cfg(target_pointer_width = "32")]
        test_prim!(
            u128::MAX,
            &[
                0,
                LimbRepr::MAX,
                LimbRepr::MAX,
                LimbRepr::MAX,
                LimbRepr::MAX
            ],
            [u128]
        );
        #[cfg(target_pointer_width = "64")]
        test_prim!(u128::MAX, &[0, LimbRepr::MAX, LimbRepr::MAX], [u128]);
    }

    #[test]
    fn from_usize_max() {
        #[cfg(target_pointer_width = "32")]
        test_prim!(usize::MAX, &[0, LimbRepr::MAX], [u32, u64, u128, usize]);
        #[cfg(target_pointer_width = "64")]
        test_prim!(usize::MAX, &[0, LimbRepr::MAX], [u64, u128, usize]);
    }
}
