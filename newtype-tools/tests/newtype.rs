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

    let mut apples1 = Apples(1);
    let apples2 = Apples::from(2);
    assert_eq!(apples1.as_ref(), &1);
    assert_eq!(apples2.0, 2);
    apples1.0 = 42;
    let apples1: u64 = *apples1.as_ref();
    let apples2: u64 = *apples2.as_ref();
    assert_eq!(apples1, 42);
    assert_eq!(apples2, 2);
}

#[test]
fn generic_newtype() {
    #[derive(Newtype)]
    #[repr(transparent)]
    struct Apples<T>(T)
    where
        T: Into<i32>;
    let mut apples1 = Apples(1);
    let apples2 = Apples::from(2);
    assert_eq!(apples1.as_ref(), &1);
    assert_eq!(apples2.0, 2);
    apples1.0 = 42;
    let apples1: i32 = *apples1.as_ref();
    let apples2: i32 = *apples2.as_ref();
    assert_eq!(apples1, 42);
    assert_eq!(apples2, 2);
}
