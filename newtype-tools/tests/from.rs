#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn from() {
    #[derive(Newtype)]
    /// Doc comment.
    #[newtype(
        // From oranges.
        from("Oranges", with = "|oranges| Apples(oranges.0 as u64 * 2)"),
        // From oranges reference.
        from(&Oranges, with = |oranges| Apples(oranges.0 as u64 * 2)),
        // Into oranges.
        into(Oranges, with = |apples| Oranges((apples.0 / 2) as u32)),
    )]
    #[repr(transparent)]
    struct Apples(u64);
    struct Oranges(u32);

    let apples = Apples(42);
    assert_eq!(apples.0, 42);

    let oranges = Oranges::from(apples);
    assert_eq!(oranges.0, 21);
}

#[test]
fn try_from() {
    use std::num::TryFromIntError;
    #[derive(Newtype)]
    #[newtype(try_from(
        u64,
        error = "TryFromIntError",
        with = "|v| u32::try_from(v).map(Oranges)"
    ))]
    #[repr(transparent)]
    struct Oranges(u32);

    let oranges = Oranges::try_from(42_u64).unwrap();
    assert_eq!(oranges.0, 42);
}
