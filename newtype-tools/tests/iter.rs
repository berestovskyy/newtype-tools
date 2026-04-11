#![cfg(feature = "derive")]

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn iter() {
    fn test<R: std::ops::RangeBounds<Apples>>(range: R) {
        let mut iter = newtype_tools::Iterator::from(&range);
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.clone().count(), 3);
        assert_eq!(iter.clone().next_back(), Some(Apples(2)));
        assert_eq!(iter.clone().nth(1), Some(Apples(1)));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(Apples(0)));
        assert_eq!(iter.next(), Some(Apples(1)));
        assert_eq!(iter.next(), Some(Apples(2)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[derive(Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples(u64);

    test(Apples(0)..Apples(3));
    test(Apples(0)..=Apples(2));
    test(..Apples(3));
    test(..=Apples(2));
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn infinite_iter() {
    fn test<R: std::ops::RangeBounds<Apples>>(range: R) {
        let mut iter = newtype_tools::Iterator::from(&range);
        assert_eq!(iter.len(), usize::MAX);
        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
        assert_eq!(iter.clone().count(), usize::MAX);
        assert_eq!(iter.clone().next_back(), Some(Apples(u64::MAX)));
        assert_eq!(iter.clone().nth(1), Some(Apples(2)));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(Apples(1)));
        assert_eq!(iter.next(), Some(Apples(2)));
    }
    #[derive(Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples(u64);

    test(Apples(1)..);
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn custom_inner_type() {
    #[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
    struct CustomInner(u64);

    impl newtype_tools::iter::MinMax for CustomInner {
        const MIN: Self = Self(u64::MIN);
        const MAX: Self = Self(u64::MAX);
    }

    impl newtype_tools::iter::Step for CustomInner {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            (
                (end.0 as usize).saturating_sub(start.0 as usize),
                (end.0 as usize).checked_sub(start.0 as usize),
            )
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            start.0.checked_add(count as u64).map(CustomInner)
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            start.0.checked_sub(count as u64).map(CustomInner)
        }
    }

    #[derive(Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Oranges(CustomInner);

    let range = Oranges(CustomInner(1))..Oranges(CustomInner(3));
    let mut iter = newtype_tools::Iterator::from(&range);
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.clone().count(), 2);
    assert_eq!(iter.clone().next_back(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.clone().nth(1), Some(Oranges(CustomInner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Oranges(CustomInner(1))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn generic_iter() {
    fn test<T, R: std::ops::RangeBounds<Apples<T>>>(range: R)
    where
        T: std::fmt::Debug + Into<u64> + From<u64> + newtype_tools::iter::Step,
    {
        let mut iter = newtype_tools::Iterator::from(&range);
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.clone().count(), 3);
        assert_eq!(iter.clone().next_back(), Some(Apples(T::from(2))));
        assert_eq!(iter.clone().nth(1), Some(Apples(T::from(1))));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(Apples(T::from(0))));
        assert_eq!(iter.next(), Some(Apples(T::from(1))));
        assert_eq!(iter.next(), Some(Apples(T::from(2))));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples<T>(T)
    where
        T: Into<u64>;

    test(Apples(0_u64)..Apples(3));
    test(Apples(0_u64)..=Apples(2));
    test(..Apples(3_u64));
    test(..=Apples(2_u64));
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn generic_infinite_iter() {
    fn test<T, R: std::ops::RangeBounds<Apples<T>>>(range: R)
    where
        T: std::fmt::Debug + Into<u64> + From<u64> + newtype_tools::iter::Step,
    {
        let mut iter = newtype_tools::Iterator::from(&range);
        assert_eq!(iter.len(), usize::MAX);
        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
        assert_eq!(iter.clone().count(), usize::MAX);
        assert_eq!(iter.clone().next_back(), Some(Apples(T::from(u64::MAX))));
        assert_eq!(iter.clone().nth(1), Some(Apples(T::from(2))));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(Apples(T::from(1))));
        assert_eq!(iter.next(), Some(Apples(T::from(2))));
    }

    #[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples<T>(T)
    where
        T: Into<u64>;

    test(Apples(1_u64)..);
}
