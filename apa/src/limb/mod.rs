use core::mem;

#[cfg(target_pointer_width = "32")]
pub type LimbRepr = u32;
#[cfg(target_pointer_width = "64")]
pub type LimbRepr = u64;

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
    #[inline]
    pub fn repr(self) -> LimbRepr {
        self.0
    }

    /// Calculates `self` + `other`.
    ///
    /// Returns a tuple of the addition along with a boolean indicating whether
    /// an arithmetic overflow would occur. If an overflow would have occurred
    /// then the wrapped value is returned.
    #[inline]
    pub fn add_overflow(self, other: Limb) -> (Limb, bool) {
        let (val, carry) = self.repr().overflowing_add(other.repr());
        (Limb(val), carry)
    }

    /// Calculates `self` - `other`.
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would have
    /// occurred then the wrapped value is returned.
    #[inline]
    pub fn sub_overflow(self, other: Limb) -> (Limb, bool) {
        let (val, carry) = self.repr().overflowing_sub(other.repr());
        (Limb(val), carry)
    }

    /// Returns the number of leading zeros in the binary representation of the
    /// limb.
    #[inline]
    pub fn leading_zeros(self) -> LimbRepr {
        self.repr().leading_zeros() as LimbRepr
    }

    /// Returns the number of trailing zeros in the binary representation of
    /// the limb.
    #[inline]
    pub fn trailing_zeros(self) -> LimbRepr {
        self.repr().trailing_zeros() as LimbRepr
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
