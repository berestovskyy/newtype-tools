#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn newtype_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/newtype-fail/*.rs");
}

#[test]
fn newtype() {
    #[derive(Newtype)]
    #[repr(transparent)]
    struct Apples(u64);

    let apples1 = Apples(1);
    let apples2 = Apples::from(2);
    assert_eq!(apples1.as_inner(), &1);
    assert_eq!(apples2.0, 2);
    let apples1: u64 = *apples1.as_inner();
    let apples2: u64 = *apples2.as_inner();
    assert_eq!(apples1, 1);
    assert_eq!(apples2, 2);
}

#[test]
fn generic_newtype() {
    #[derive(Newtype)]
    #[repr(transparent)]
    struct Apples<T>(T);
    let apples1 = Apples(1_u64);
    let apples2 = Apples::from(2_u64);
    assert_eq!(apples1.as_inner(), &1);
    assert_eq!(apples2.0, 2);
    let apples1: u64 = *apples1.as_inner();
    let apples2: u64 = *apples2.as_inner();
    assert_eq!(apples1, 1);
    assert_eq!(apples2, 2);
}

#[test]
fn manual_generic_implementation() {
    struct Oranges<T>(T);
    impl<T> ::newtype_tools::Newtype for Oranges<T> {
        type Inner = T;
        fn as_inner(&self) -> &Self::Inner {
            &self.0
        }
    }
    impl<T> AsRef<T> for Oranges<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }
    impl<T> AsMut<T> for Oranges<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }
    impl<T> From<T> for Oranges<T> {
        fn from(inner: T) -> Self {
            Oranges(inner)
        }
    }
    let mut oranges1 = Oranges(1_u64);
    let oranges2 = Oranges::from(2_u64);
    assert_eq!(oranges1.as_ref(), &1);
    assert_eq!(oranges2.0, 2);
    *oranges1.as_mut() = 42;
    let oranges1: u64 = *oranges1.as_ref();
    let oranges2: u64 = *oranges2.as_ref();
    assert_eq!(oranges1, 42);
    assert_eq!(oranges2, 2);
}
