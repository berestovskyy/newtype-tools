#![cfg(feature = "derive")]

#[test]
fn add() {
    #[derive(Debug, newtype_tools::Newtype)]
    /// Doc comment.
    #[newtype(add(Oranges, output = Weight, with = "|apples, oranges| Weight(apples.0 + oranges.0 as u64 * 2)"))]
    struct Apples(u64);
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let apples = Apples(42);
    let oranges = Oranges(21);
    let sum = apples + oranges;
    assert_eq!(sum, Weight(84));
}

#[test]
fn generic_add() {
    #[derive(Debug, newtype_tools::Newtype)]
    #[newtype(add(Oranges, output = Weight, with = "|apples, oranges| Weight(apples.0.clone() + oranges.0 as u64 * 2)"))]
    struct Apples<T>(T)
    where
        T: std::ops::Add<u64, Output = u64> + Clone;
    #[derive(Debug)]
    struct Oranges(u32);
    #[derive(Debug, PartialEq)]
    struct Weight(u64);

    let apples = Apples(42_u64);
    let oranges = Oranges(21);
    let sum = apples + oranges;
    assert_eq!(sum, Weight(84));
}
