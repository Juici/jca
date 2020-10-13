use apa::ApInt;

#[test]
fn zero() {
    assert_eq!(0, usize::from(ApInt::ZERO));
}

#[test]
fn one() {
    assert_eq!(1, usize::from(ApInt::ONE));
}
