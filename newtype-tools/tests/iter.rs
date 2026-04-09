#![cfg(feature = "derive")]

#[test]
fn iter() {
    #[derive(Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples(u64);

    let range = Apples(1)..Apples(3);
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.clone().count(), 2);
    assert_eq!(iter.clone().next_back(), Some(Apples(2)));
    assert_eq!(iter.clone().nth(1), Some(Apples(2)));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Apples(1)));
    assert_eq!(iter.next(), Some(Apples(2)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = ..Apples(3);
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
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

    let range = ..=Apples(3);
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.clone().count(), 4);
    assert_eq!(iter.clone().next_back(), Some(Apples(3)));
    assert_eq!(iter.clone().nth(1), Some(Apples(1)));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Apples(0)));
    assert_eq!(iter.next(), Some(Apples(1)));
    assert_eq!(iter.next(), Some(Apples(2)));
    assert_eq!(iter.next(), Some(Apples(3)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Apples(1)..;
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    // The `ExactSize` default implementation asserts that the range is finite.
    // assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (usize::MAX, None));
    assert_eq!(iter.clone().count(), usize::MAX);
    assert_eq!(iter.clone().next_back(), Some(Apples(u64::MAX)));
    assert_eq!(iter.clone().nth(1), Some(Apples(2)));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Apples(1)));
    assert_eq!(iter.next(), Some(Apples(2)));

    let range = Apples(1)..=Apples(3);
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.clone().count(), 3);
    assert_eq!(iter.clone().next_back(), Some(Apples(3)));
    assert_eq!(iter.clone().nth(1), Some(Apples(2)));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Apples(1)));
    assert_eq!(iter.next(), Some(Apples(2)));
    assert_eq!(iter.next(), Some(Apples(3)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Apples(1)..=Apples(3);
    for apple in newtype_tools::NewtypeIterator::iter(&range) {
        println!("{apple:?}");
    }
    for apple in newtype_tools::NewtypeIterator::iter(&range) {
        println!("{apple:?}");
    }
}

#[test]
fn custom_inner_type() {
    #[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
    struct CustomInner(u64);

    impl newtype_tools::iter::NewtypeMinMax for CustomInner {
        const MIN: Self = Self(u64::MIN);
        const MAX: Self = Self(u64::MAX);
    }

    impl newtype_tools::iter::NewtypeStep for CustomInner {
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
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
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

    let range = ..Oranges(CustomInner(3));
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.clone().count(), 3);
    assert_eq!(iter.clone().next_back(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.clone().nth(1), Some(Oranges(CustomInner(1))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Oranges(CustomInner(0))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(1))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = ..=Oranges(CustomInner(3));
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.clone().count(), 4);
    assert_eq!(iter.clone().next_back(), Some(Oranges(CustomInner(3))));
    assert_eq!(iter.clone().nth(1), Some(Oranges(CustomInner(1))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Oranges(CustomInner(0))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(1))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(3))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Oranges(CustomInner(1))..;
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    // The `ExactSize` default implementation asserts that the range is finite.
    // assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (usize::MAX, None));
    assert_eq!(iter.clone().count(), usize::MAX);
    assert_eq!(
        iter.clone().next_back(),
        Some(Oranges(CustomInner(u64::MAX)))
    );
    assert_eq!(iter.clone().nth(1), Some(Oranges(CustomInner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Oranges(CustomInner(1))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(2))));

    let range = Oranges(CustomInner(1))..=Oranges(CustomInner(3));
    let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.clone().count(), 3);
    assert_eq!(iter.clone().next_back(), Some(Oranges(CustomInner(3))));
    assert_eq!(iter.clone().nth(1), Some(Oranges(CustomInner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Oranges(CustomInner(1))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(2))));
    assert_eq!(iter.next(), Some(Oranges(CustomInner(3))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Oranges(CustomInner(1))..=Oranges(CustomInner(3));
    for apple in newtype_tools::NewtypeIterator::iter(&range) {
        println!("{apple:?}");
    }
    for apple in newtype_tools::NewtypeIterator::iter(&range) {
        println!("{apple:?}");
    }
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn manual_iter_implementation() {
    #[derive(Clone, Debug, PartialOrd, PartialEq)]
    struct Inner(u64);

    impl newtype_tools::iter::NewtypeMinMax for Inner {
        const MIN: Self = Self(u64::MIN);
        const MAX: Self = Self(u64::MAX);
    }

    impl newtype_tools::iter::NewtypeStep for Inner {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            (
                (end.0 as usize).saturating_sub(start.0 as usize),
                (end.0 as usize).checked_sub(start.0 as usize),
            )
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            start.0.checked_add(count as u64).map(Inner)
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            start.0.checked_sub(count as u64).map(Inner)
        }
    }

    #[derive(Clone, Debug, PartialOrd, PartialEq)]
    struct Bananas(Inner);
    impl newtype_tools::Newtype for Bananas {
        type Inner = Inner;
        fn as_inner(&self) -> &Self::Inner {
            &self.0
        }
    }
    impl From<Inner> for Bananas {
        fn from(inner: Inner) -> Self {
            Self(inner)
        }
    }
    impl From<Bananas> for Inner {
        fn from(newtype: Bananas) -> Self {
            newtype.0
        }
    }

    #[derive(Clone)]
    struct ApplesIterator {
        start: Inner,
        last: Inner,
    }

    impl ApplesIterator {
        pub fn is_empty(&self) -> bool {
            self.start > self.last
        }
    }

    impl Iterator for ApplesIterator {
        type Item = Bananas;
        fn next(&mut self) -> Option<Self::Item> {
            if ApplesIterator::is_empty(self) {
                return None;
            }

            let next = newtype_tools::iter::NewtypeStep::forward_checked(self.start.clone(), 1)?;
            Some(Bananas::from(core::mem::replace(&mut self.start, next)))
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            use newtype_tools::iter::NewtypeMinMax;
            if self.is_empty() {
                return (0, Some(0));
            }

            if self.start == Inner::MIN && self.start < Inner(0) || self.last == Inner::MAX {
                return (usize::MAX, None);
            }

            let hint = newtype_tools::iter::NewtypeStep::steps_between(&self.start, &self.last);
            (
                hint.0.saturating_add(1),
                hint.1.and_then(|steps| steps.checked_add(1)),
            )
        }

        #[inline]
        fn count(self) -> usize {
            if self.is_empty() {
                return 0;
            }

            newtype_tools::iter::NewtypeStep::steps_between(&self.start, &self.last)
                .1
                .and_then(|steps| steps.checked_add(1))
                .expect("count overflowed usize")
        }

        #[inline]
        fn is_sorted(self) -> bool {
            true
        }
    }

    impl DoubleEndedIterator for ApplesIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if ApplesIterator::is_empty(self) {
                return None;
            }
            let next = newtype_tools::iter::NewtypeStep::backward_checked(self.last.clone(), 1)?;
            Some(Bananas::from(core::mem::replace(&mut self.last, next)))
        }
    }

    impl ExactSizeIterator for ApplesIterator {}
    impl std::iter::FusedIterator for ApplesIterator {}

    impl Bananas {
        pub fn iter<R: ::std::ops::RangeBounds<Bananas>>(range: &R) -> ApplesIterator {
            use ::std::ops::Bound;
            use newtype_tools::Newtype;
            use newtype_tools::iter::NewtypeMinMax;
            use newtype_tools::iter::NewtypeStep;
            let start = match range.start_bound() {
                Bound::Included(s) => s.as_inner().clone(),
                Bound::Excluded(s) => NewtypeStep::forward(s.as_inner().clone(), 1),
                Bound::Unbounded => Inner::MIN,
            };
            let last = match range.end_bound() {
                Bound::Included(e) => e.as_inner().clone(),
                Bound::Excluded(e) => NewtypeStep::backward(e.as_inner().clone(), 1),
                Bound::Unbounded => Inner::MAX,
            };
            ApplesIterator { start, last }
        }
    }

    let range = Bananas(Inner(1))..Bananas(Inner(3));
    let mut iter = Bananas::iter(&range);
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.clone().count(), 2);
    assert_eq!(iter.clone().next_back(), Some(Bananas(Inner(2))));
    assert_eq!(iter.clone().nth(1), Some(Bananas(Inner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Bananas(Inner(1))));
    assert_eq!(iter.next(), Some(Bananas(Inner(2))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = ..Bananas(Inner(3));
    let mut iter = Bananas::iter(&range);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.clone().count(), 3);
    assert_eq!(iter.clone().next_back(), Some(Bananas(Inner(2))));
    assert_eq!(iter.clone().nth(1), Some(Bananas(Inner(1))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Bananas(Inner(0))));
    assert_eq!(iter.next(), Some(Bananas(Inner(1))));
    assert_eq!(iter.next(), Some(Bananas(Inner(2))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = ..=Bananas(Inner(3));
    let mut iter = Bananas::iter(&range);
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.clone().count(), 4);
    assert_eq!(iter.clone().next_back(), Some(Bananas(Inner(3))));
    assert_eq!(iter.clone().nth(1), Some(Bananas(Inner(1))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Bananas(Inner(0))));
    assert_eq!(iter.next(), Some(Bananas(Inner(1))));
    assert_eq!(iter.next(), Some(Bananas(Inner(2))));
    assert_eq!(iter.next(), Some(Bananas(Inner(3))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Bananas(Inner(1))..;
    let mut iter = Bananas::iter(&range);
    // The `ExactSize` default implementation asserts that the range is finite.
    // assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (usize::MAX, None));
    assert_eq!(iter.clone().count(), usize::MAX);
    assert_eq!(iter.clone().next_back(), Some(Bananas(Inner(u64::MAX))));
    assert_eq!(iter.clone().nth(1), Some(Bananas(Inner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Bananas(Inner(1))));
    assert_eq!(iter.next(), Some(Bananas(Inner(2))));

    let range = Bananas(Inner(1))..=Bananas(Inner(3));
    let mut iter = Bananas::iter(&range);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.clone().count(), 3);
    assert_eq!(iter.clone().next_back(), Some(Bananas(Inner(3))));
    assert_eq!(iter.clone().nth(1), Some(Bananas(Inner(2))));
    assert!(iter.clone().is_sorted());
    assert_eq!(iter.next(), Some(Bananas(Inner(1))));
    assert_eq!(iter.next(), Some(Bananas(Inner(2))));
    assert_eq!(iter.next(), Some(Bananas(Inner(3))));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let range = Bananas(Inner(1))..=Bananas(Inner(3));
    for apple in Bananas::iter(&range) {
        println!("{apple:?}");
    }
    for apple in Bananas::iter(&range) {
        println!("{apple:?}");
    }
}
