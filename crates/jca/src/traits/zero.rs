/// Defines the additive identity for `Self`.
///
/// # Laws
///
/// ```text
/// n + 0 = n = 0 + n       ∀ n ∈ Self
/// ```
pub trait Zero {
    /// The additive identity.
    const ZERO: Self;

    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool;
}

// Primitive implementations.
macro_rules! zero_impl {
    ($($ty:ty)*; $v:expr) => {
        $(
            impl Zero for $ty {
                const ZERO: $ty = $v;

                fn is_zero(&self) -> bool {
                    *self == Self::ZERO
                }
            }
        )*
    };
}

zero_impl!(u8 u16 u32 u64 u128 usize; 0);
zero_impl!(i8 i16 i32 i64 i128 isize; 0);
zero_impl!(f32 f64; 0.0);
