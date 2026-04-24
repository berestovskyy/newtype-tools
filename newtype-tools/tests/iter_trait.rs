#![cfg(feature = "derive")]

#[rstest::rstest]
#[timeout(core::time::Duration::from_secs(1))]
fn iter_trait() {
    fn test(mut iter: newtype_tools::Iterator<Apples>) {
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

    #[derive(Clone, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Apples(u64);

    use newtype_tools::{IntoIter, Iter};

    test((Apples(0)..Apples(3)).iter());
    test((Apples(0)..Apples(3)).into_iter());
    test((Apples(0)..=Apples(2)).iter());
    test((Apples(0)..=Apples(2)).into_iter());
    test((..Apples(3)).iter());
    test((..Apples(3)).into_iter());
    test((..=Apples(2)).iter());
    test((..=Apples(2)).into_iter());
}

#[rstest::rstest]
#[timeout(core::time::Duration::from_secs(1))]
fn infinite_iter_trait() {
    fn test(mut iter: newtype_tools::Iterator<Apples>) {
        assert_eq!(iter.len(), usize::MAX);
        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
        assert_eq!(iter.clone().count(), usize::MAX);
        assert_eq!(iter.clone().next_back(), Some(Apples(u64::MAX)));
        assert_eq!(iter.clone().nth(1), Some(Apples(2)));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(Apples(1)));
        assert_eq!(iter.next(), Some(Apples(2)));
    }
    #[derive(Clone, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Apples(u64);

    use newtype_tools::{IntoIter, Iter};

    test((Apples(1)..).iter());
    test((Apples(1)..).into_iter());
    test(
        (
            core::ops::Bound::Excluded(Apples(0)),
            core::ops::Bound::Unbounded,
        )
            .iter(),
    );
    test(
        (
            core::ops::Bound::Excluded(Apples(0)),
            core::ops::Bound::Unbounded,
        )
            .into_iter(),
    );
}

#[rstest::rstest]
#[timeout(core::time::Duration::from_secs(1))]
fn empty_iter_trait() {
    fn test(mut iter: newtype_tools::Iterator<Apples>) {
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.clone().count(), 0);
        assert_eq!(iter.clone().next_back(), None);
        assert_eq!(iter.clone().nth(1), None);
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), None);
    }
    #[derive(Clone, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Apples(u64);

    use newtype_tools::{IntoIter, Iter};

    test((Apples(1)..Apples(1)).iter());
    test((Apples(1)..Apples(1)).into_iter());
}

#[rstest::rstest]
#[timeout(core::time::Duration::from_secs(1))]
fn custom_inner_type_trait() {
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct CustomInner(u64);

    impl newtype_tools::iter::MinMax for CustomInner {
        const MAX: Self = Self(u64::MAX);
        const MIN: Self = Self(u64::MIN);
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

    trait Iterator<N, R>
    where
        N: newtype_tools::Newtype,
        N::Inner: newtype_tools::iter::Step,
        R: core::ops::RangeBounds<N>,
    {
        fn iter(&self) -> newtype_tools::Iterator<N>;
    }

    impl<N, R> Iterator<N, R> for R
    where
        N: newtype_tools::Newtype,
        N::Inner: newtype_tools::iter::Step,
        R: core::ops::RangeBounds<N>,
    {
        fn iter(&self) -> newtype_tools::Iterator<N> {
            newtype_tools::Iterator::from(self)
        }
    }

    #[derive(Clone, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Oranges(CustomInner);

    let range = Oranges(CustomInner(1))..Oranges(CustomInner(3));
    let mut iter = range.iter();
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
#[timeout(core::time::Duration::from_secs(1))]
fn generic_iter_trait() {
    fn test<N>(mut iter: newtype_tools::Iterator<N>)
    where
        N: newtype_tools::Newtype + Clone + From<u64> + PartialEq + PartialOrd + core::fmt::Debug,
        N::Inner: newtype_tools::iter::Step,
    {
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.clone().count(), 3);
        assert_eq!(iter.clone().next_back(), Some(N::from(2)));
        assert_eq!(iter.clone().nth(1), Some(N::from(1)));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(N::from(0)));
        assert_eq!(iter.next(), Some(N::from(1)));
        assert_eq!(iter.next(), Some(N::from(2)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[derive(Clone, Copy, Debug, Default, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Apples<T>(T)
    where
        T: Into<u64>;

    use newtype_tools::{IntoIter, Iter};

    test((Apples(0_u64)..Apples(3)).iter());
    test((Apples(0_u64)..Apples(3)).into_iter());
    test((Apples(0_u64)..=Apples(2)).iter());
    test((Apples(0_u64)..=Apples(2)).into_iter());
    test((..Apples(3_u64)).iter());
    test((..Apples(3_u64)).into_iter());
    test((..=Apples(2_u64)).iter());
    test((..=Apples(2_u64)).into_iter());
}

#[rstest::rstest]
#[timeout(core::time::Duration::from_secs(1))]
fn generic_infinite_iter_trait() {
    fn test<N>(mut iter: newtype_tools::Iterator<N>)
    where
        N: newtype_tools::Newtype + Clone + From<u64> + PartialEq + PartialOrd + core::fmt::Debug,
        N::Inner: newtype_tools::iter::Step,
    {
        assert_eq!(iter.len(), usize::MAX);
        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
        assert_eq!(iter.clone().count(), usize::MAX);
        assert_eq!(iter.clone().next_back(), Some(N::from(u64::MAX)));
        assert_eq!(iter.clone().nth(1), Some(N::from(2)));
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), Some(N::from(1)));
        assert_eq!(iter.next(), Some(N::from(2)));
    }

    #[derive(Clone, Copy, Debug, Default, newtype_tools::Newtype, PartialEq, PartialOrd)]
    struct Apples<T>(T)
    where
        T: Into<u64>;

    use newtype_tools::{IntoIter, Iter};

    test((Apples(1_u64)..).iter());
    test((Apples(1_u64)..).into_iter());
}
