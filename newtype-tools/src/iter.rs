#[cfg(test)]
mod tests;

/// A helper trait to support inner values ranges.
pub trait NewtypeMinMax {
    const MIN: Self;
    const MAX: Self;
}

/// A copy of the unstable `std::iter::Step` trait required for the `iter`
/// derive.
pub trait NewtypeStep: Clone + PartialOrd + Sized {
    /// Returns the bounds on the number of *successor* steps required to get from `start` to `end`
    /// like [`Iterator::size_hint()`][Iterator::size_hint()].
    ///
    /// Returns `(usize::MAX, None)` if the number of steps would overflow `usize`, or is infinite.
    ///
    /// # Invariants
    ///
    /// For any `a`, `b`, and `n`:
    ///
    /// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::forward_checked(&a, n) == Some(b)`
    /// * `steps_between(&a, &b) == (n, Some(n))` if and only if `Step::backward_checked(&b, n) == Some(a)`
    /// * `steps_between(&a, &b) == (n, Some(n))` only if `a <= b`
    ///   * Corollary: `steps_between(&a, &b) == (0, Some(0))` if and only if `a == b`
    /// * `steps_between(&a, &b) == (0, None)` if `a > b`
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>);

    /// Returns the value that would be obtained by taking the *successor*
    /// of `self` `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`, returns `None`.
    ///
    /// # Invariants
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
    fn forward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by taking the *successor*
    /// of `self` `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    ///
    /// Unsafe code should not rely on the correctness of behavior after overflow.
    ///
    /// # Invariants
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
    fn forward(start: Self, count: usize) -> Self {
        NewtypeStep::forward_checked(start, count).expect("overflow in `Step::forward`")
    }

    /// Returns the value that would be obtained by taking the *predecessor*
    /// of `self` `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`, returns `None`.
    ///
    /// # Invariants
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
    fn backward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by taking the *predecessor*
    /// of `self` `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    ///
    /// Unsafe code should not rely on the correctness of behavior after overflow.
    ///
    /// # Invariants
    ///
    /// For any `a`, `n`, and `m`, where no overflow occurs:
    ///
    /// * `Step::backward(Step::backward(a, n), m) == Step::backward(a, n + m)`
    ///
    /// For any `a` and `n`, where no overflow occurs:
    ///
    /// * `Step::backward_checked(a, n) == Some(Step::backward(a, n))`
    /// * `Step::backward(a, n) == (0..n).fold(a, |x, _| Step::backward(x, 1))`
    ///   * Corollary: `Step::backward(a, 0) == a`
    /// * `Step::backward(a, n) <= a`
    /// * `Step::forward(Step::backward(a, n), n) == a`
    fn backward(start: Self, count: usize) -> Self {
        NewtypeStep::backward_checked(start, count).expect("overflow in `Step::backward`")
    }
}

macro_rules! impl_step {
    ($($t:ty),*) => {
        $(
            impl NewtypeMinMax for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
            impl NewtypeStep for $t {
                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if start <= end {
                        (
                            usize::try_from(end - start).unwrap_or_else(|_|0),
                            usize::try_from(end - start).ok()
                        )
                    } else {
                        (0, None)
                    }
                }
                fn forward_checked(start: Self, count: usize) -> Option<Self> {
                    match Self::try_from(count) {
                        Ok(n) => start.checked_add(n),
                        Err(_) => None, // if n is out of range, `unsigned_start + n` is too
                    }
                }
                fn backward_checked(start: Self, count: usize) -> Option<Self> {
                    match Self::try_from(count) {
                        Ok(n) => start.checked_sub(n),
                        Err(_) => None, // if n is out of range, `unsigned_start - n` is too
                    }
                }
            }
        )*
    };
}

impl_step!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
);

/// Blanket `NewtypeStep` implementation for all `Newtype`s.
impl<T> NewtypeStep for T
where
    T: crate::Newtype + Clone + PartialOrd + Sized + From<T::Inner>,
    T::Inner: NewtypeStep,
{
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        <T::Inner as NewtypeStep>::steps_between(start.as_inner(), end.as_inner())
    }

    #[inline]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        <T::Inner as NewtypeStep>::forward_checked(start.as_inner().clone(), count).map(T::from)
    }

    #[inline]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        <T::Inner as NewtypeStep>::backward_checked(start.as_inner().clone(), count).map(Self::from)
    }
}

/// Blanket `Iterator`` implementation for all `Newtype`s.
#[derive(Clone)]
pub struct NewtypeIterator<T>
where
    T: crate::Newtype,
{
    start: T::Inner,
    last: T::Inner,
}

impl<T> NewtypeIterator<T>
where
    T: crate::Newtype,
    T::Inner: NewtypeStep + NewtypeMinMax + Default,
{
    /// Returns `true` if the iterator is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start > self.last
    }

    /// Creates a new `NewtypeIterator` instance based on the given range.
    /// The internal `Newtype` representation must implement `NewtypeStep` trait.
    ///
    /// # Example
    /// ```
    /// # #[cfg(feature = "derive")]
    /// # {
    /// #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// struct Apples(u64);
    /// let range = Apples(1)..Apples(3);
    /// let mut iter = newtype_tools::NewtypeIterator::iter(&range);
    /// assert_eq!(iter.len(), 2);
    /// assert_eq!(iter.next(), Some(Apples(1)));
    /// assert_eq!(iter.next(), Some(Apples(2)));
    /// assert_eq!(iter.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn iter<R: ::std::ops::RangeBounds<T>>(range: &R) -> Self {
        use crate::iter::NewtypeMinMax;
        use crate::iter::NewtypeStep;
        use ::std::ops::Bound;
        let start = match range.start_bound() {
            Bound::Included(s) => s.as_inner().clone(),
            Bound::Excluded(s) => NewtypeStep::forward(s.as_inner().clone(), 1),
            Bound::Unbounded => T::Inner::MIN,
        };
        let last = match range.end_bound() {
            Bound::Included(e) => e.as_inner().clone(),
            Bound::Excluded(e) => NewtypeStep::backward(e.as_inner().clone(), 1),
            Bound::Unbounded => T::Inner::MAX,
        };
        Self { start, last }
    }
}

impl<T> Iterator for NewtypeIterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: NewtypeStep + NewtypeMinMax + Default,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if NewtypeIterator::is_empty(self) {
            return None;
        }

        let next = crate::iter::NewtypeStep::forward_checked(self.start.clone(), 1)?;
        Some(T::from(core::mem::replace(&mut self.start, next)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        use crate::iter::NewtypeMinMax;
        if self.is_empty() {
            return (0, Some(0));
        }

        if self.start == T::Inner::MIN && self.start != T::Inner::default()
            || self.last == T::Inner::MAX
        {
            return (usize::MAX, None);
        }

        let hint = crate::iter::NewtypeStep::steps_between(&self.start, &self.last);
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

        crate::iter::NewtypeStep::steps_between(&self.start, &self.last)
            .1
            .and_then(|steps| steps.checked_add(1))
            .expect("count overflowed usize")
    }

    #[inline]
    fn is_sorted(self) -> bool {
        true
    }
}

impl<T> DoubleEndedIterator for NewtypeIterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: NewtypeStep + NewtypeMinMax + Default,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if NewtypeIterator::is_empty(self) {
            return None;
        }
        let next = crate::iter::NewtypeStep::backward_checked(self.last.clone(), 1)?;
        Some(T::from(core::mem::replace(&mut self.last, next)))
    }
}

impl<T> ExactSizeIterator for NewtypeIterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: NewtypeStep + NewtypeMinMax + Default,
{
}

impl<T> std::iter::FusedIterator for NewtypeIterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: NewtypeStep + NewtypeMinMax + Default,
{
}
