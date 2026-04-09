/// `NewtypeStep` `steps_between` invariants:
///
/// For any `a`, `b`, and `n`:
///
/// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::forward_checked(&a, n) == Some(b)`
/// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::backward_checked(&b, n) == Some(a)`
/// * `steps_between(&a, &b) == (n, Some(n))` only if `a <= b`
///   * Corollary: `steps_between(&a, &b) == (0, Some(0))` if and only if `a == b`
/// * `steps_between(&a, &b) == (0, None)` if `a > b`
#[rstest::rstest]
#[case::u8_a0_b0_n0(0_u8, 0_u8, 0_usize)]
#[case::u8_a0_b1_n1(0_u8, 1_u8, 1_usize)]
#[case::u8_a1_b0_n1(1_u8, 0_u8, 1_usize)]
#[case::u8_a_max1_b_max_n1(u8::MAX - 1, u8::MAX, 1_usize)]
#[case::u8_a_max_b_max_n0(u8::MAX, u8::MAX, 0_usize)]
#[case::u8_a0_b0_n_max(0_u8, 0_u8, u8::MAX as usize)]
#[case::u8_a0_b1_n_max(0_u8, 1_u8, u8::MAX as usize)]
#[case::u8_a1_b0_n_max(1_u8, 0_u8, u8::MAX as usize)]
#[case::i64_a0_b0_n0(0_i64, 0_i64, 0_usize)]
#[case::i64_a0_b1_n1(0_i64, 1_i64, 1_usize)]
#[case::i64_a1_b0_n1(1_i64, 0_i64, 1_usize)]
#[case::i64_a_max1_b_max_n1(i64::MAX - 1, i64::MAX, 1_usize)]
#[case::i64_a_max_b_max_n0(i64::MAX, i64::MAX, 0_usize)]
#[case::i64_a0_b0_n_max(0_i64, 0_i64, i64::MAX as usize)]
#[case::i64_a0_b1_n_max(0_i64, 1_i64, i64::MAX as usize)]
#[case::i64_a1_b0_n_max(1_i64, 0_i64, i64::MAX as usize)]
#[case::i128_a0_b_max_n1(0_i128, i128::MAX, 1_usize)]
#[case::i128_a_max_b0_n1(i128::MAX, 0_i128, 1_usize)]
#[timeout(std::time::Duration::from_secs(1))]
fn steps_between_invariants<T>(#[case] a: T, #[case] b: T, #[case] n: usize)
where
    T: crate::iter::NewtypeStep + Copy + core::fmt::Debug,
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

/// `NewtypeStep` `forward_checked` invariants:
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
#[case::u8_a0_n0_m0(0_u8, 0_usize, 0_usize)]
#[case::u8_a0_n1_m1(0_u8, 1_usize, 1_usize)]
#[case::u8_a_max1_n1_m1(u8::MAX - 1, 1, 1_usize)]
#[case::u8_a_max_n1_m1(u8::MAX, 1_usize, 1_usize)]
#[case::u8_a_max_n_max_m1(u8::MAX, usize::MAX, 1_usize)]
#[case::u8_a_max_n_max_m_max(u8::MAX, usize::MAX, usize::MAX)]
#[case::i64_a0_n0_m0(0_i64, 0_usize, 0_usize)]
#[case::i64_a0_n1_m1(0_i64, 1_usize, 1_usize)]
#[case::i64_a_max1_n1_m1(i64::MAX - 1, 1, 1_usize)]
#[case::i64_a_max_n1_m1(i64::MAX, 1_usize, 1_usize)]
#[case::i64_a_max_n_max_m1(i64::MAX, usize::MAX, 1_usize)]
#[case::i64_a_max_n_max_m_max(i64::MAX, usize::MAX, usize::MAX)]
#[case::i128_a_max_n1_m1(i128::MAX, 1_usize, 1_usize)]
#[timeout(std::time::Duration::from_secs(1))]
fn forward_checked_invariants<T: crate::iter::NewtypeStep + Copy + core::fmt::Debug>(
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

/// `NewtypeStep` `backward_checked` invariants:
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
#[case::u8_a0_n0_m0(0_u8, 0_usize, 0_usize)]
#[case::u8_a0_n1_m1(0_u8, 1_usize, 1_usize)]
#[case::u8_a_max1_n1_m1(u8::MAX - 1, 1, 1_usize)]
#[case::u8_a_max_n1_m1(u8::MAX, 1_usize, 1_usize)]
#[case::u8_a_max_n_max_m1(u8::MAX, usize::MAX, 1_usize)]
#[case::u8_a_max_n_max_m_max(u8::MAX, usize::MAX, usize::MAX)]
#[case::i64_a0_n0_m0(0_i64, 0_usize, 0_usize)]
#[case::i64_a0_n1_m1(0_i64, 1_usize, 1_usize)]
#[case::i64_a_max1_n1_m1(i64::MAX - 1, 1, 1_usize)]
#[case::i64_a_max_n1_m1(i64::MAX, 1_usize, 1_usize)]
#[case::i64_a_max_n_max_m1(i64::MAX, usize::MAX, 1_usize)]
#[case::i64_a_max_n_max_m_max(i64::MAX, usize::MAX, usize::MAX)]
#[case::i128_a_max_n1_m1(i128::MAX, 1_usize, 1_usize)]
#[timeout(std::time::Duration::from_secs(1))]
fn backward_checked_invariants<T: crate::iter::NewtypeStep + Copy + core::fmt::Debug>(
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
