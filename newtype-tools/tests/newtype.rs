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
    let apples2 = Apples::new(2);
    assert_eq!(apples1.into_inner(), 1);
    assert_eq!(apples2.0, 2);

    #[derive(Newtype)]
    #[repr(transparent)]
    struct Oranges {
        inner: u32,
    }

    let oranges1 = Oranges { inner: 1 };
    let oranges2 = Oranges::new(2);
    assert_eq!(oranges1.into_inner(), 1);
    assert_eq!(oranges2.inner, 2);
}
