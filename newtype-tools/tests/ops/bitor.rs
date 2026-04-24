#![cfg(feature = "derive")]

const EXPECTED_WEIGHT: u64 = 42;

#[test]
fn bitor() {
    #[derive(Debug, newtype_tools::Newtype)]
    /// Doc comment.
    #[newtype(bitor(
        Oranges,
        output = "Weight",
        with = "|apples, oranges| Weight(apples.0 | (oranges.0 as u64 * 2))"
    ))]
    struct Apples(u64);
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let res = Apples(42) | Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) | Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = Apples(42) | &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) | &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
}

#[test]
fn generic_bitor() {
    #[derive(Debug, newtype_tools::Newtype)]
    #[newtype(bitor(
        Oranges,
        output = "Weight",
        with = "|apples, oranges| Weight(apples.0.clone() | (oranges.0 as u64 * 2))"
    ))]
    struct Apples<T>(T)
    where
        T: core::ops::BitOr<u64, Output = u64> + Clone;
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let res = Apples(42) | Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) | Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = Apples(42) | &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) | &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
}

#[test]
fn bitor_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// Doc comment.
    #[newtype(bitor_assign(Oranges, with = "|apples, oranges| apples.0 |= oranges.0 as u64 * 2"))]
    struct Apples(u64);
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    apples |= Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    apples |= &Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
}

#[test]
fn generic_bitor_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    #[newtype(bitor_assign(Oranges, with = "|apples, oranges| apples.0 |= oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: core::ops::BitOrAssign<u64> + Clone;
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples |= oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples |= &oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    assert_eq!(oranges, Oranges(21));
}
