use apa::ApInt;

macro_rules! assert_conv {
    ($ty:ident: $($val:expr),* $(,)?) => {
        $({
            let val = $val as $ty;
            let int = ApInt::from(val);
            assert_eq!(<$ty>::from(int), val, concat!("convert equality failed for `", stringify!($val), "`: {}"), $val);
        })*
    };
}

macro_rules! test_prim {
    ($prim:ident: $fn:ident) => {
        #[test]
        fn $fn() {
            assert_conv!($prim: $prim::MAX, $prim::MIN);
        }
    };
}

test_prim!(u8: from_to_u8);
test_prim!(u16: from_to_u16);
test_prim!(u32: from_to_u32);
test_prim!(u64: from_to_u64);
test_prim!(u128: from_to_u128);
test_prim!(usize: from_to_usize);

test_prim!(i8: from_to_i8);
test_prim!(i16: from_to_i16);
test_prim!(i32: from_to_i32);
test_prim!(i64: from_to_i64);
test_prim!(i128: from_to_i128);
test_prim!(isize: from_to_isize);
