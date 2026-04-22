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

    let sum = Apples(42) + Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = &Apples(42) + Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = Apples(42) + &Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = &Apples(42) + &Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
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
        T: std::ops::Add<u64, Output = u64> + Clone;
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let sum = Apples(42) + Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = &Apples(42) + Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = Apples(42) + &Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
    let sum = &Apples(42) + &Oranges(21);
    assert_eq!(sum, Weight(EXPECTED_WEIGHT));
}
