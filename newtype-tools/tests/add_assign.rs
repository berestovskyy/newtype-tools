#![cfg(feature = "derive")]

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
    assert_eq!(apples, Apples(84));
    let mut apples = Apples(42);
    apples += &Oranges(21);
    assert_eq!(apples, Apples(84));
}

#[test]
fn generic_add_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    #[newtype(add_assign(Oranges, with = "|apples, oranges| apples.0 += oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: std::ops::AddAssign<u64> + Clone;
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples += oranges;
    assert_eq!(apples, Apples(84));
    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples += &oranges;
    assert_eq!(apples, Apples(84));
    assert_eq!(oranges, Oranges(21));
}
