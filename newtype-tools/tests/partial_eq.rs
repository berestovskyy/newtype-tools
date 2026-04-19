#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn partial_eq() {
    #[derive(Debug, Newtype)]
    /// Doc comment.
    #[newtype(partial_eq(Oranges, with = "|apples, oranges| apples.0 == oranges.0 as u64 * 2"))]
    struct Apples(u64);
    #[derive(Debug)]
    struct Oranges(u32);

    let apples = Apples(42);
    let oranges = Oranges(21);
    assert_eq!(apples, oranges);
}

#[test]
fn generic_partial_eq() {
    #[derive(Debug, Newtype)]
    #[newtype(partial_eq(Oranges, with = "|apples, oranges| apples.0 == oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: PartialEq<u64>;
    #[derive(Debug)]
    struct Oranges(u32);

    let apples = Apples(42);
    let oranges = Oranges(21);
    assert_eq!(apples, oranges);
}
