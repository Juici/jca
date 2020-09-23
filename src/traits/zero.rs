/// Defines the additive identity for `Self`.
pub trait Zero {
    /// The additive identity.
    const ZERO: Self;
}

// Primitive implementations.
macro_rules! impl_prim {
    ($($ty:ty)*) => {
        $(
            impl Zero for $ty {
                const ZERO: $ty = 0 as $ty;
            }
        )*
    };
}

impl_prim!(u8 u16 u32 u64 u128 usize);
impl_prim!(i8 i16 i32 i64 i128 isize);
impl_prim!(f32 f64);
