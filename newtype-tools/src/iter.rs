/// Blanket `Iterator` implementation for all `Newtype`s.
#[derive(Clone)]
pub struct Iterator<T>
where
    T: crate::Newtype,
{
    start: T::Inner,
    last: T::Inner,
}

impl<T> Iterator<T>
where
    T: crate::Newtype,
    T::Inner: Step,
{
    /// Returns `true` if the iterator is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start > self.last
    }

    /// Creates a new `Iterator` instance based on the given range.
    /// The internal `Newtype` representation must implement `Step` trait.
    ///
    /// # Example
    /// ```
    /// # #[cfg(feature = "derive")]
    /// # {
    /// #[derive(Debug, newtype_tools::Newtype, PartialEq)]
    /// struct Apples(u64);
    /// let range = Apples(1)..Apples(3);
    /// let mut iter = newtype_tools::Iterator::from(&range);
    /// assert_eq!(iter.len(), 2);
    /// assert_eq!(iter.next(), Some(Apples(1)));
    /// assert_eq!(iter.next(), Some(Apples(2)));
    /// assert_eq!(iter.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn from<R: std::ops::RangeBounds<T>>(range: &R) -> Self {
        use crate::iter::Step;
        use std::ops::Bound;
        let start = match range.start_bound() {
            Bound::Included(s) => s.as_ref().clone(),
            Bound::Excluded(s) => Step::forward(s.as_ref().clone(), 1),
            Bound::Unbounded => T::Inner::MIN,
        };
        let last = match range.end_bound() {
            Bound::Included(e) => e.as_ref().clone(),
            Bound::Excluded(e) => Step::backward(e.as_ref().clone(), 1),
            Bound::Unbounded => T::Inner::MAX,
        };
        Self { start, last }
    }
}

impl<T> std::iter::Iterator for Iterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: Step + std::fmt::Debug,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if Iterator::is_empty(self) {
            return None;
        }

        let next = crate::iter::Step::forward_checked(self.start.clone(), 1)?;
        Some(T::from(core::mem::replace(&mut self.start, next)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_empty() {
            return (0, Some(0));
        }

        println!(
            "XXX getting steps between start:{:?} end:{:?}",
            self.start, self.last
        );
        let hint = Step::steps_between(&self.start, &self.last);
        println!("XXX hint:{hint:?}");
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

        crate::iter::Step::steps_between(&self.start, &self.last)
            .1
            .and_then(|steps| steps.checked_add(1))
            .expect("count overflowed usize")
    }

    #[inline]
    fn is_sorted(self) -> bool {
        true
    }
}

impl<T> DoubleEndedIterator for Iterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: Step + std::fmt::Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if Iterator::is_empty(self) {
            return None;
        }
        let next = crate::iter::Step::backward_checked(self.last.clone(), 1)?;
        Some(T::from(core::mem::replace(&mut self.last, next)))
    }
}

impl<T> ExactSizeIterator for Iterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: Step + std::fmt::Debug,
{
}

impl<T> std::iter::FusedIterator for Iterator<T>
where
    T: crate::Newtype + From<T::Inner>,
    T::Inner: Step + std::fmt::Debug,
{
}

////////////////////////////////////////////////////////////////////////
// The following complexity should go away once the `Step` trait is stable.
// Mostly it's a copy-paste from the unstable `std::iter::Step` trait.

/// A helper trait to support inner values ranges.
pub trait MinMax {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_min_max {
    ($($t:ty),*) => {
        $(
            impl MinMax for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
        )+
    };
}

impl_min_max!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
);

/// A copy of the unstable `std::iter::Step` trait required for the `iter`
/// derive.
pub trait Step: Clone + PartialOrd + MinMax + Sized {
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
        Step::forward_checked(start, count).expect("overflow in `Step::forward`")
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
        Step::backward_checked(start, count).expect("overflow in `Step::backward`")
    }
}

// These are still macro-generated because the integer literals resolve to different types.
macro_rules! step_identical_methods {
    () => {
        #[inline]
        #[allow(arithmetic_overflow)]
        fn forward(start: Self, n: usize) -> Self {
            // In debug builds, trigger a panic on overflow.
            // This should optimize completely out in release builds.
            if Self::forward_checked(start, n).is_none() {
                let _ = Self::MAX + 1;
            }
            // Do wrapping math to allow e.g. `Step::forward(-128i8, 255)`.
            start.wrapping_add(n as Self)
        }

        #[inline]
        #[allow(arithmetic_overflow)]
        fn backward(start: Self, n: usize) -> Self {
            // In debug builds, trigger a panic on overflow.
            // This should optimize completely out in release builds.
            if Self::backward_checked(start, n).is_none() {
                let _ = Self::MIN - 1;
            }
            // Do wrapping math to allow e.g. `Step::backward(127i8, 255)`.
            start.wrapping_sub(n as Self)
        }
    };
}

macro_rules! step_integer_impls {
    {
        [ $( [ $u_narrower:ident $i_narrower:ident ] ),+ ] <= usize <
        [ $( [ $u_wider:ident $i_wider:ident ] ),+ ]
    } => {
        $(
            #[allow(unreachable_patterns)]
            impl Step for $u_narrower {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if *start <= *end {
                        // This relies on $u_narrower <= usize
                        let steps = (*end - *start) as usize;
                        (steps, Some(steps))
                    } else {
                        (0, None)
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n) => start.checked_add(n),
                        Err(_) => None, // if n is out of range, `unsigned_start + n` is too
                    }
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n) => start.checked_sub(n),
                        Err(_) => None, // if n is out of range, `unsigned_start - n` is too
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl Step for $i_narrower {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if *start <= *end {
                        // This relies on $i_narrower <= usize
                        //
                        // Casting to isize extends the width but preserves the sign.
                        // Use wrapping_sub in isize space and cast to usize to compute
                        // the difference that might not fit inside the range of isize.
                        let steps = (*end as isize).wrapping_sub(*start as isize) as usize;
                        (steps, Some(steps))
                    } else {
                        (0, None)
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    match $u_narrower::try_from(n) {
                        Ok(n) => {
                            // Wrapping handles cases like
                            // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                            // even though 200 is out of range for i8.
                            let wrapped = start.wrapping_add(n as Self);
                            if wrapped >= start {
                                Some(wrapped)
                            } else {
                                None // Addition overflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 + n` necessarily overflows i8.
                        Err(_) => None,
                    }
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    match $u_narrower::try_from(n) {
                        Ok(n) => {
                            // Wrapping handles cases like
                            // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                            // even though 200 is out of range for i8.
                            let wrapped = start.wrapping_sub(n as Self);
                            if wrapped <= start {
                                Some(wrapped)
                            } else {
                                None // Subtraction overflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 - n` necessarily overflows i8.
                        Err(_) => None,
                    }
                }
            }
        )+

        $(
            #[allow(unreachable_patterns)]
            impl Step for $u_wider {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if *start <= *end {
                        if let Ok(steps) = usize::try_from(*end - *start) {
                            (steps, Some(steps))
                        } else {
                            (usize::MAX, None)
                        }
                    } else {
                        (0, None)
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_add(n as Self)
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_sub(n as Self)
                }
            }

            #[allow(unreachable_patterns)]
            impl Step for $i_wider {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                    if *start <= *end {
                        match end.checked_sub(*start) {
                            Some(result) => {
                                if let Ok(steps) = usize::try_from(result) {
                                    (steps, Some(steps))
                                } else {
                                    (usize::MAX, None)
                                }
                            }
                            // If the difference is too big for e.g. i128,
                            // it's also gonna be too big for usize with fewer bits.
                            None => (usize::MAX, None),
                        }
                    } else {
                        (0, None)
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_add(n as Self)
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_sub(n as Self)
                }
            }
        )+
    };
}

#[cfg(target_pointer_width = "64")]
step_integer_impls! {
    [ [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize] ] <= usize < [ [u128 i128] ]
}

#[cfg(target_pointer_width = "32")]
step_integer_impls! {
    [ [u8 i8], [u16 i16], [u32 i32], [usize isize] ] <= usize < [ [u64 i64], [u128 i128] ]
}

#[cfg(target_pointer_width = "16")]
step_integer_impls! {
    [ [u8 i8], [u16 i16], [usize isize] ] <= usize < [ [u32 i32], [u64 i64], [u128 i128] ]
}
