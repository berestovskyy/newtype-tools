#![cfg(feature = "derive")]

// Manual integer newtype trait definition.
#[derive(Clone, Copy, Debug, Eq, Hash, newtype_tools::Newtype, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct ManInt(i64);

// Attribute integer newtype trait definition.
#[newtype_tools::newtype(Id)]
/// Doc comment.
struct AttrInt(i64);

#[rstest::rstest]
#[case::i64(2_i64, 3_i64, 2_i64)]
#[case::man_int(ManInt(2_i64), ManInt(3_i64), 2_i64)]
#[case::attr_int(AttrInt(2_i64), AttrInt(3_i64), 2_i64)]
#[timeout(core::time::Duration::from_secs(1))]
fn int_newtype_id<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone + Copy + PartialEq + PartialOrd + Ord + core::hash::Hash + core::fmt::Debug + From<R>,
    R: Copy + PartialOrd,
{
    // PartialEq
    assert!(a != b);
    // PartialOrd
    assert!(a < b);
    // Ord
    assert_eq!(a.min(b), a);
    // Hash
    let mut hasher = std::hash::DefaultHasher::new();
    let prev_a = a;
    a.hash(&mut hasher);
    assert_eq!(prev_a, a);
    // From<Repr>
    let res = T::from(repr_a);
    assert!(res == a);
}

// Manual floating point newtype trait definition.
#[derive(Clone, Copy, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
#[repr(transparent)]
struct ManFloat(f64);

// Attribute floating point newtype trait definition.
#[newtype_tools::newtype(Id)]
/// Doc comment.
struct AttrFloat(f64);

#[rstest::rstest]
#[case::f64(2.0_f64, 3.0_f64, 2.0_f64)]
#[case::man_float(ManFloat(2.0_f64), ManFloat(3.0_f64), 2.0_f64)]
#[case::attr_float(AttrFloat(2.0_f64), AttrFloat(3.0_f64), 2.0_f64)]
#[timeout(core::time::Duration::from_secs(1))]
fn float_newtype_id<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone + Copy + PartialEq + PartialOrd + core::fmt::Debug + From<R>,
    R: Copy + PartialOrd,
{
    // PartialEq
    assert!(a != b);
    // PartialOrd
    assert!(a < b);
    // Ord: is not implemented for f64
    // Hash: is not implemented for f64
    // From<Repr>
    let res = T::from(repr_a);
    assert!(res == a);
}

// Manual generic newtype trait definition.
#[derive(Clone, Copy, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
#[repr(transparent)]
struct ManGeneric<T>(T)
where
    T: Clone + Copy + PartialEq + PartialOrd + core::fmt::Debug + From<T>;
// Attribute generic newtype trait definition.
#[newtype_tools::newtype(Id)]
/// Doc comment.
struct AttrGeneric<T>(T)
where
    T: Clone + Copy + PartialEq + PartialOrd + core::fmt::Debug + From<T>;
#[rstest::rstest]
#[case::generic(2.0_f64, 3.0_f64, 2.0_f64)]
#[case::man_generic(ManGeneric(2.0_f64), ManGeneric(3.0_f64), 2.0_f64)]
#[case::attr_generic(AttrGeneric(2.0_f64), AttrGeneric(3.0_f64), 2.0_f64)]
#[timeout(core::time::Duration::from_secs(1))]
fn generic_float_newtype_id<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone + Copy + PartialEq + PartialOrd + core::fmt::Debug + From<R>,
    R: Copy + PartialOrd + core::ops::Mul<R, Output = R> + core::ops::MulAssign<R>,
{
    // PartialEq
    assert!(a != b);
    // PartialOrd
    assert!(a < b);
    // Ord: is not implemented for f64
    // Hash: is not implemented for f64
    // From<Repr>
    let res = T::from(repr_a);
    assert!(res == a);
}
