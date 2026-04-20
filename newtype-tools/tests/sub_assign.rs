#![cfg(feature = "derive")]

#[test]
fn sub_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// Doc comment.
    #[newtype(sub_assign(Oranges, with = "|apples, oranges| apples.0 -= oranges.0 as u64 * 2"))]
    struct Apples(u64);
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    apples -= Oranges(21);
    assert_eq!(apples, Apples(0));
    let mut apples = Apples(42);
    apples -= &Oranges(21);
    assert_eq!(apples, Apples(0));
}

#[test]
fn generic_sub_assign() {
    #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    #[newtype(sub_assign(Oranges, with = "|apples, oranges| apples.0 -= oranges.0 as u64 * 2"))]
    struct Apples<T>(T)
    where
        T: std::ops::SubAssign<u64> + Clone;
    #[derive(Debug, PartialEq)]
    struct Oranges(u32);

    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples -= oranges;
    assert_eq!(apples, Apples(0));
    let mut apples = Apples(42);
    let oranges = Oranges(21);
    apples -= &oranges;
    assert_eq!(apples, Apples(0));
    assert_eq!(oranges, Oranges(21));
}
