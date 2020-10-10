use core::mem;

/// Internal representation of the digits in an `ApInt`.
pub type LimbRepr = u64;
/// Internal computation unit in an `ApInt`.
pub type DoubleLimbRepr = u128;

const REPR_ZERO: LimbRepr = 0x0;
const REPR_ONE: LimbRepr = 0x1;
const REPR_ONES: LimbRepr = !REPR_ZERO;

/// A limb is part of an `ApInt` that fits within a single machine word.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Limb(LimbRepr);

impl Limb {
    /// The number of bits in a single `Limb`.
    pub const BITS: usize = mem::size_of::<LimbRepr>();

    /// A `Limb` with value `0`.
    pub const ZERO: Limb = Limb(REPR_ZERO);
    /// A `Limb` with value `1`.
    pub const ONE: Limb = Limb(REPR_ONE);
    /// A `Limb` with all bits set to `1`.
    pub const ONES: Limb = Limb(REPR_ONES);

    /// Returns the value of the internal representation.
    pub fn repr(self) -> LimbRepr {
        self.0
    }
}

// Delegate formatting.
macro_rules! impl_fmt {
    ($ty:ty: [$($trait:ident),* $(,)*]) => {
        $(
            impl core::fmt::$trait for $ty {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    self.repr().fmt(f)
                }
            }
        )*
    };
}

impl_fmt!(Limb: [Binary, Octal, LowerHex, UpperHex]);
