#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn into() {
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

#[test]
fn generic_into() {
    #[derive(Newtype)]
    #[newtype(into(Oranges, with = |apples| Oranges((apples.0.into() / 2) as u32)))]
    #[repr(transparent)]
    struct Apples<T>(T)
    where
        T: Into<i32>;
    struct Oranges(u32);

    let apples = Apples(42);
    assert_eq!(apples.0, 42);

    let oranges = Oranges::from(apples);
    assert_eq!(oranges.0, 21);
}

#[test]
fn try_into() {
    use std::num::TryFromIntError;
    #[derive(Newtype)]
    #[newtype(try_into(
        Oranges,
        error = TryFromIntError,
        with = |apples| u32::try_from(apples.0 / 2).map(Oranges)
    ))]
    #[repr(transparent)]
    struct Apples(u64);
    struct Oranges(u32);

    let apples = Apples(42);
    assert_eq!(apples.0, 42);

    let oranges = Oranges::try_from(apples).unwrap();
    assert_eq!(oranges.0, 21);
}

#[test]
fn generic_try_into() {
    use std::num::TryFromIntError;
    #[derive(Newtype)]
    #[newtype(try_into(
        Oranges,
        error = TryFromIntError,
        with = |apples| u32::try_from(apples.0.into() / 2).map(Oranges)
    ))]
    #[repr(transparent)]
    struct Apples<T>(T)
    where
        T: Into<i32>;
    struct Oranges(u32);

    let apples = Apples(42);
    assert_eq!(apples.0, 42);

    let oranges = Oranges::try_from(apples).unwrap();
    assert_eq!(oranges.0, 21);
}
