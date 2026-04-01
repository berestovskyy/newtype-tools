#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn newtype_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/newtype-fail/*.rs");
}

#[test]
fn newtype() {
    #[derive(Newtype)]
    #[repr(transparent)]
    struct Apples(u64);

    let apples1 = Apples(1);
    let apples2 = Apples::from(2);
    assert_eq!(apples1.as_inner(), &1);
    assert_eq!(apples2.0, 2);
    let apples1: u64 = apples1.into();
    let apples2: u64 = apples2.into();
    assert_eq!(apples1, 1);
    assert_eq!(apples2, 2);
}
