use std::num::NonZeroUsize;

/// Represents the number of bits in an `Integer`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BitWidth(NonZeroUsize);

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! assoc_consts {
    ($(
        $vis:vis $ident:ident = $value:expr;
    )*) => {
        $(
            doc_comment!{
                concat!("Represents a bit-width of value `", stringify!($value), "`."),
                #[allow(unused)]
                $vis const $ident: BitWidth = BitWidth(unsafe { NonZeroUsize::new_unchecked($value) });
            }
        )*
    };
}

impl BitWidth {
    assoc_consts! {
        pub W1 = 1;
        pub W8 = 8;
        pub W16 = 16;
        pub W32 = 32;
        pub W64 = 64;
        pub W128 = 128;
    }
}
