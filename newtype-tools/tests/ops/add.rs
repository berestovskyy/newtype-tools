#![cfg(feature = "derive")]

const EXPECTED_WEIGHT: u64 = 42 + 21 * 2;

#[test]
fn add() {
    #[derive(Debug, newtype_tools::Newtype)]
    /// Doc comment.
    #[newtype(add(
        Oranges,
        output = "Weight",
        with = "|apples, oranges| Weight(apples.0 + oranges.0 as u64 * 2)"
    ))]
    struct Apples(u64);
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let res = Apples(42) + Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) + Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = Apples(42) + &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) + &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
}

#[test]
fn generic_add() {
    #[derive(Debug, newtype_tools::Newtype)]
    #[newtype(add(
        Oranges,
        output = "Weight",
        with = "|apples, oranges| Weight(apples.0.clone() + oranges.0 as u64 * 2)"
    ))]
    struct Apples<T>(T)
    where
        T: core::ops::Add<u64, Output = u64> + Clone;
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let res = Apples(42) + Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) + Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = Apples(42) + &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
    let res = &Apples(42) + &Oranges(21);
    assert_eq!(res, Weight(EXPECTED_WEIGHT));
}

#[test]
fn add_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// Doc comment.
    #[newtype(add_assign(Oranges, with = "|apples, oranges| apples.0 += oranges.0 as u64 * 2"))]
    struct Apples(u64);
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    apples += Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    apples += &Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
}

#[test]
fn generic_add_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    #[newtype(add_assign(Oranges, with = "|apples, oranges| apples.0 += oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: core::ops::AddAssign<u64> + Clone;
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples += oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples += &oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    assert_eq!(oranges, Oranges(21));
}
