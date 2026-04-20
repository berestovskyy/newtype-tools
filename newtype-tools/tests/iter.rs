#![cfg(feature = "derive")]

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
struct Gold(i8);
impl newtype_tools::iter::MinMax for Gold {
    const MIN: Self = Gold(i8::MIN);
    const MAX: Self = Gold(i8::MAX);
}

impl TryFrom<Gold> for usize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: Gold) -> Result<Self, Self::Error> {
        usize::try_from(value.0)
    }
}

/// `Step::steps_between` invariants:
///
/// For any `a`, `b`, and `n`:
///
/// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::forward_checked(&a, n) == Some(b)`
/// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::backward_checked(&b, n) == Some(a)`
/// * `steps_between(&a, &b) == (n, Some(n))` only if `a <= b`
///   * Corollary: `steps_between(&a, &b) == (0, Some(0))` if and only if `a == b`
/// * `steps_between(&a, &b) == (0, None)` if `a > b`
#[rstest::rstest]
#[case::u8_a0_b0_n0(0_u8, 0_u8, 0)]
#[case::u8_a0_b1_n1(0_u8, 1_u8, 1)]
#[case::u8_a1_b0_n1(1_u8, 0_u8, 1)]
#[case::u8_a_max1_b_max_n1(u8::MAX - 1, u8::MAX, 1)]
#[case::u8_a_max_b_max_n0(u8::MAX, u8::MAX, 0)]
#[case::u8_a0_b0_n_max(0_u8, 0_u8, u8::MAX as usize)]
#[case::u8_a0_b1_n_max(0_u8, 1_u8, u8::MAX as usize)]
#[case::u8_a1_b0_n_max(1_u8, 0_u8, u8::MAX as usize)]
#[case::i64_a0_b0_n0(0_i64, 0_i64, 0)]
#[case::i64_a0_b1_n1(0_i64, 1_i64, 1)]
#[case::i64_a1_b0_n1(1_i64, 0_i64, 1)]
#[case::i64_a_max1_b_max_n1(i64::MAX - 1, i64::MAX, 1)]
#[case::i64_a_max_b_max_n0(i64::MAX, i64::MAX, 0)]
#[case::i64_a0_b0_n_max(0_i64, 0_i64, i64::MAX as usize)]
#[case::i64_a0_b1_n_max(0_i64, 1_i64, i64::MAX as usize)]
#[case::i64_a1_b0_n_max(1_i64, 0_i64, i64::MAX as usize)]
#[case::i128_a0_b_max_n1(0_i128, i128::MAX, 1)]
#[case::i128_a_max_b0_n1(i128::MAX, 0_i128, 1)]
#[case::u128_a0_b_max_n1(0_u128, u128::MAX, 1)]
#[case::u128_a_max_b0_n1(u128::MAX, 0_u128, 1)]
#[case::gold_a0_b0_n0(Gold(0), Gold(0), 0)]
#[case::gold_a_max2_b1_n1(Gold(i8::MAX - 2), Gold(1), 1)]
#[case::gold_a1_b_max2_n1(Gold(1), Gold(i8::MAX - 2), 0)]
#[case::gold_a1_b1_n_max2(Gold(1), Gold(1), i8::MAX as usize - 2)]
#[timeout(std::time::Duration::from_secs(1))]
fn steps_between_invariants<T>(#[case] a: T, #[case] b: T, #[case] n: usize)
where
    T: newtype_tools::iter::Step + Copy,
    usize: TryFrom<T>,
{
    if let Some(b) = T::forward_checked(a, n) {
        assert_eq!(T::steps_between(&a, &b), (n, Some(n)));
    } else {
        assert_ne!(T::steps_between(&a, &b), (n, Some(n)));
    }
    if let Some(a) = T::backward_checked(b, n) {
        assert_eq!(T::steps_between(&a, &b), (n, Some(n)));
    } else {
        assert_ne!(T::steps_between(&a, &b), (n, Some(n)));
    }
    if a <= b && usize::try_from(b).is_ok() {
        assert!(T::steps_between(&a, &b).1.is_some());
    }
    if a == b {
        assert_eq!(T::steps_between(&a, &b), (0, Some(0)));
    } else {
        assert_ne!(T::steps_between(&a, &b), (0, Some(0)));
    }
    if a > b {
        assert_eq!(T::steps_between(&a, &b), (0, None));
    }
}

/// Cover `Step::steps_between` subtraction overflow.
#[rstest::rstest]
#[case::i128_a_min_b_max(i128::MIN, i128::MAX)]
#[timeout(std::time::Duration::from_secs(1))]
fn steps_between_overflow<T>(#[case] a: T, #[case] b: T)
where
    T: newtype_tools::iter::Step,
{
    assert_eq!(T::steps_between(&a, &b), (usize::MAX, None));
}

/// `Step::forward_checked` invariants:
///
/// For any `a`, `n`, and `m`:
///
/// * `Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == Step::forward_checked(a, m).and_then(|x| Step::forward_checked(x, n))`
/// * `Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == try { Step::forward_checked(a, n.checked_add(m)) }`
///
/// For any `a` and `n`:
///
/// * `Step::forward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::forward_checked(&x, 1))`
///   * Corollary: `Step::forward_checked(a, 0) == Some(a)`
#[rstest::rstest]
#[case::u8_a0_n0_m0(0_u8, 0, 0)]
#[case::u8_a0_n1_m1(0_u8, 1, 1)]
#[case::u8_a_max1_n1_m1(u8::MAX - 1, 1, 1)]
#[case::u8_a_max_n1_m1(u8::MAX, 1, 1)]
#[case::u8_a_max_n_max_m1(u8::MAX, usize::MAX, 1)]
#[case::u8_a_max_n_max_m_max(u8::MAX, usize::MAX, usize::MAX)]
#[case::i64_a0_n0_m0(0_i64, 0, 0)]
#[case::i64_a0_n1_m1(0_i64, 1, 1)]
#[case::i64_a_max1_n1_m1(i64::MAX - 1, 1, 1)]
#[case::i64_a_max_n1_m1(i64::MAX, 1, 1)]
#[case::i64_a_max_n_max_m1(i64::MAX, usize::MAX, 1)]
#[case::i64_a_max_n_max_m_max(i64::MAX, usize::MAX, usize::MAX)]
#[case::i128_a_max_n1_m1(i128::MAX, 1, 1)]
#[case::u128_a_max_n1_m1(u128::MAX, 1, 1)]
#[timeout(std::time::Duration::from_secs(1))]
fn forward_checked_invariants<T: newtype_tools::iter::Step + Copy + core::fmt::Debug>(
    #[case] a: T,
    #[case] n: usize,
    #[case] m: usize,
) {
    assert_eq!(
        T::forward_checked(a, n).and_then(|x| T::forward_checked(x, m)),
        T::forward_checked(a, m).and_then(|x| T::forward_checked(x, n))
    );
    if let Some(n_plus_m) = n.checked_add(m) {
        assert_eq!(
            T::forward_checked(a, n).and_then(|x| T::forward_checked(x, m)),
            T::forward_checked(a, n_plus_m)
        );
    }
    assert_eq!(
        T::forward_checked(a, n),
        (0..n).try_fold(a, |x, _| T::forward_checked(x, 1))
    );
    assert_eq!(T::forward_checked(a, 0), Some(a));
}

/// `Step::forward` invariants for default implementation:
///
/// For any `a`, `n`, and `m`, where no overflow occurs:
///
/// * `Step::forward(Step::forward(a, n), m) == Step::forward(a, n + m)`
///
/// For any `a` and `n`, where no overflow occurs:
///
/// * `Step::forward_checked(a, n) == Some(Step::forward(a, n))`
/// * `Step::forward(a, n) == (0..n).fold(a, |x, _| Step::forward(x, 1))`
///   * Corollary: `Step::forward(a, 0) == a`
/// * `Step::forward(a, n) >= a`
/// * `Step::backward(Step::forward(a, n), n) == a`
#[rstest::rstest]
#[case::gold_a0_n0_m0(Gold(0), 0, 0)]
#[case::gold_a0_n1_m1(Gold(1), 1, 1)]
#[case::gold_a_max2_n1_m1(Gold(i8::MAX - 2), 1, 1)]
#[case::gold_a1_n_max2_m1(Gold(1), i8::MAX as usize - 2, 1)]
#[case::gold_a1_n1_m_max2(Gold(1), 1, i8::MAX as usize - 2)]
#[timeout(std::time::Duration::from_secs(1))]
fn default_impl_forward_invariants<T: newtype_tools::iter::Step + Copy + core::fmt::Debug>(
    #[case] a: T,
    #[case] n: usize,
    #[case] m: usize,
) {
    assert_eq!(T::forward(T::forward(a, n), m), T::forward(a, n + m));
    assert_eq!(T::forward_checked(a, n), Some(T::forward(a, n)));
    assert_eq!(
        T::forward(a, n),
        (0..n).fold(a, |x, _| newtype_tools::iter::Step::forward(x, 1))
    );
    assert_eq!(T::forward(a, 0), a);
    assert!(T::forward(a, n) >= a);
    assert_eq!(T::backward(T::forward(a, n), n), a);
}

/// Cover `Step::forward` implementation for `step_identical_methods` macro.
#[test]
#[should_panic]
fn step_identical_methods_forward() {
    use newtype_tools::iter::Step;
    #[allow(arithmetic_overflow)]
    let _ = i32::forward(i32::MAX, 1);
}

/// Cover `Step::backward` implementation for `step_identical_methods` macro.
#[test]
#[should_panic]
fn step_identical_methods_backward() {
    use newtype_tools::iter::Step;
    #[allow(arithmetic_overflow)]
    let _ = i32::backward(i32::MIN, 1);
}

/// Cover `Step::forward_checked` and `backward_checked` implementation for `step_integer_impls` macro.
#[test]
fn step_integer_impls_forward_backward_checked() {
    use newtype_tools::iter::Step;
    assert!(i8::forward_checked(0, usize::MAX).is_none());
    assert!(i8::backward_checked(0, usize::MAX).is_none());
}

/// `Step::backward_checked` invariants:
///
/// For any `a`, `n`, and `m`:
///
/// * `Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == n.checked_add(m).and_then(|x| Step::backward_checked(a, x))`
/// * `Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == try { Step::backward_checked(a, n.checked_add(m)?) }`
///
/// For any `a` and `n`:
///
/// * `Step::backward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::backward_checked(x, 1))`
///   * Corollary: `Step::backward_checked(a, 0) == Some(a)`
#[rstest::rstest]
#[case::u8_a0_n0_m0(0_u8, 0, 0)]
#[case::u8_a0_n1_m1(0_u8, 1, 1)]
#[case::u8_a_max1_n1_m1(u8::MAX - 1, 1, 1)]
#[case::u8_a_max_n1_m1(u8::MAX, 1, 1)]
#[case::u8_a_max_n_max_m1(u8::MAX, usize::MAX, 1)]
#[case::u8_a_max_n_max_m_max(u8::MAX, usize::MAX, usize::MAX)]
#[case::i64_a0_n0_m0(0_i64, 0, 0)]
#[case::i64_a0_n1_m1(0_i64, 1, 1)]
#[case::i64_a_max1_n1_m1(i64::MAX - 1, 1, 1)]
#[case::i64_a_max_n1_m1(i64::MAX, 1, 1)]
#[case::i64_a_max_n_max_m1(i64::MAX, usize::MAX, 1)]
#[case::i64_a_max_n_max_m_max(i64::MAX, usize::MAX, usize::MAX)]
#[case::i128_a_max_n1_m1(i128::MAX, 1, 1)]
#[case::u128_a_max_n1_m1(u128::MAX, 1, 1)]
#[timeout(std::time::Duration::from_secs(1))]
fn backward_checked_invariants<T: newtype_tools::iter::Step + Copy + core::fmt::Debug>(
    #[case] a: T,
    #[case] n: usize,
    #[case] m: usize,
) {
    assert_eq!(
        T::backward_checked(a, n).and_then(|x| T::backward_checked(x, m)),
        n.checked_add(m).and_then(|x| T::backward_checked(a, x))
    );
    if let Some(n_plus_m) = n.checked_add(m) {
        assert_eq!(
            T::backward_checked(a, n).and_then(|x| T::backward_checked(x, m)),
            T::backward_checked(a, n_plus_m)
        );
    }
    if n < 1_000 {
        assert_eq!(
            T::backward_checked(a, n),
            (0..n).try_fold(a, |x, _| T::backward_checked(x, 1))
        );
    }
    assert_eq!(T::backward_checked(a, 0), Some(a));
}

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
    test((
        std::ops::Bound::Excluded(Apples(0)),
        std::ops::Bound::Unbounded,
    ));
}

#[rstest::rstest]
#[timeout(std::time::Duration::from_secs(1))]
fn empty_iter() {
    fn test<R: std::ops::RangeBounds<Apples>>(range: R) {
        let mut iter = newtype_tools::Iterator::from(&range);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.clone().count(), 0);
        assert_eq!(iter.clone().next_back(), None);
        assert_eq!(iter.clone().nth(1), None);
        assert!(iter.clone().is_sorted());
        assert_eq!(iter.next(), None);
    }
    #[derive(Clone, Debug, PartialOrd, PartialEq, newtype_tools::Newtype)]
    struct Apples(u64);

    test(Apples(1)..Apples(1));
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
