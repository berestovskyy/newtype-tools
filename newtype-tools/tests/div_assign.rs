#![cfg(feature = "derive")]

const EXPECTED_WEIGHT: u64 = 1;

#[test]
fn div_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// Doc comment.
    #[newtype(div_assign(Oranges, with = "|apples, oranges| apples.0 /= oranges.0 as u64 * 2"))]
    struct Apples(u64);
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    apples /= Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    apples /= &Oranges(21);
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
}

#[test]
fn generic_div_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    #[newtype(div_assign(Oranges, with = "|apples, oranges| apples.0 /= oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: std::ops::DivAssign<u64> + Clone;
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples /= oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples /= &oranges;
    assert_eq!(apples, Apples(EXPECTED_WEIGHT));
    assert_eq!(oranges, Oranges(21));
}
