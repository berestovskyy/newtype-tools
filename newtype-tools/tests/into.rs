#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn conversion() {
    #[derive(Newtype)]
    /// Doc comment.
    #[newtype(into(Oranges, with = |apples| Oranges((apples.0 / 2) as u32)))]
    #[repr(transparent)]
    struct Apples(u64);
    struct Oranges(u32);

    let apples = Apples(42);
    assert_eq!(apples.0, 42);

    let oranges = Oranges::from(apples);
    assert_eq!(oranges.0, 21);
}
