#![cfg(feature = "derive")]

// Manual integer newtype trait definition.
#[derive(Clone, Copy, Debug, Eq, Hash, newtype_tools::Newtype, Ord, PartialEq, PartialOrd)]
#[newtype(
    add(ManInt, output = "ManInt", with = "|a1, a2| ManInt(a1.0 + a2.0)"),
    add_assign(ManInt, with = "|this, other| this.0 += other.0"),
    sub(ManInt, output = "ManInt", with = "|a1, a2| ManInt(a1.0 - a2.0)"),
    sub_assign(ManInt, with = "|this, other| this.0 -= other.0"),
    mul(i64, output = "ManInt", with = "|a, r| ManInt(a.0 * r)"),
    mul_assign(i64, with = "|this, r| this.0 *= r"),
    div(ManInt, output = "i64", with = "|a1, a2| a1.0 / a2.0")
)]
#[repr(transparent)]
struct ManInt(i64);

// Attribute integer newtype trait definition.
#[newtype_tools::newtype(Amount)]
/// Doc comment.
struct AttrInt(i64);

#[rstest::rstest]
#[case::i64(2_i64, 3_i64, 2_i64)]
#[case::man_int(ManInt(2_i64), ManInt(3_i64), 2_i64)]
#[case::attr_int(AttrInt(2_i64), AttrInt(3_i64), 2_i64)]
#[timeout(core::time::Duration::from_secs(1))]
fn int_newtype_amount<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + core::hash::Hash
        + core::fmt::Debug
        + From<R>
        + core::ops::Add<T, Output = T>
        + core::ops::AddAssign<T>
        + core::ops::Sub<T, Output = T>
        + core::ops::SubAssign<T>
        + core::ops::Mul<R, Output = T>
        + core::ops::MulAssign<R>
        + core::ops::Div<T, Output = R>,
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
    // Add<Self>
    let res = a + b;
    assert!(res > a);
    // AddAssign<Self>
    let mut res = a;
    res += b;
    assert!(res > a);
    // Sub<Self>
    let res = a - b;
    assert!(res < a);
    // SubAssign<Self>
    let mut res = a;
    res -= b;
    assert!(res < a);
    // Mul<Repr>
    let res = a * repr_a;
    assert!(res > a);
    // MulAssign<Repr>
    let mut res = a;
    res *= repr_a;
    assert!(res > a);
    // Div<Self>
    let res = a / b;
    assert!(res < repr_a);
}

// Manual floating point newtype trait definition.
#[derive(Clone, Copy, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
#[newtype(
    add(ManFloat, output = "ManFloat", with = "|a1, a2| ManFloat(a1.0 + a2.0)"),
    add_assign(ManFloat, with = "|this, other| this.0 += other.0"),
    sub(ManFloat, output = "ManFloat", with = "|a1, a2| ManFloat(a1.0 - a2.0)"),
    sub_assign(ManFloat, with = "|this, other| this.0 -= other.0"),
    mul(f64, output = "ManFloat", with = "|a, r| ManFloat(a.0 * r)"),
    mul_assign(f64, with = "|this, r| this.0 *= r"),
    div(ManFloat, output = "f64", with = "|a1, a2| a1.0 / a2.0")
)]
#[repr(transparent)]
struct ManFloat(f64);

// Attribute floating point newtype trait definition.
#[newtype_tools::newtype(Amount)]
/// Doc comment.
struct AttrFloat(f64);

#[rstest::rstest]
#[case::f64(2.0_f64, 3.0_f64, 2.0_f64)]
#[case::man_float(ManFloat(2.0_f64), ManFloat(3.0_f64), 2.0_f64)]
#[case::attr_float(AttrFloat(2.0_f64), AttrFloat(3.0_f64), 2.0_f64)]
#[timeout(core::time::Duration::from_secs(1))]
fn float_newtype_amount<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone
        + Copy
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + From<R>
        + core::ops::Add<T, Output = T>
        + core::ops::AddAssign<T>
        + core::ops::Sub<T, Output = T>
        + core::ops::SubAssign<T>
        + core::ops::Mul<R, Output = T>
        + core::ops::MulAssign<R>
        + core::ops::Div<T, Output = R>,
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
    // Add<Self>
    let res = a + b;
    assert!(res > a);
    // AddAssign<Self>
    let mut res = a;
    res += b;
    assert!(res > a);
    // Sub<Self>
    let res = a - b;
    assert!(res < a);
    // SubAssign<Self>
    let mut res = a;
    res -= b;
    assert!(res < a);
    // Mul<Repr>
    let res = a * repr_a;
    assert!(res > a);
    // MulAssign<Repr>
    let mut res = a;
    res *= repr_a;
    assert!(res > a);
    // Div<Self>
    let res = a / b;
    assert!(res < repr_a);
}

// Manual generic newtype trait definition.
#[derive(Clone, Copy, Debug, newtype_tools::Newtype, PartialEq, PartialOrd)]
#[newtype(
    add(ManGeneric<T, R>, output = "ManGeneric<T, R>", with = "|a1, a2| ManGeneric(a1.0 + a2.0)"),
    add_assign(ManGeneric<T, R>, with = "|this, other| this.0 += other.0"),
    sub(ManGeneric<T, R>, output = "ManGeneric<T, R>", with = "|a1, a2| ManGeneric(a1.0 - a2.0)"),
    sub_assign(ManGeneric<T, R>, with = "|this, other| this.0 -= other.0"),
    mul(R, output = "ManGeneric<T, R>", with = "|a, r| ManGeneric(a.0 * *r)"),
    mul_assign(R, with = "|this, r| this.0 *= *r"),
    div(ManGeneric<T, R>, output = "R", with = "|a1, a2| a1.0 / a2.0")
)]
#[repr(transparent)]
struct ManGeneric<T, R>(T)
where
    T: Clone
        + Copy
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + From<R>
        + core::ops::Add<T, Output = T>
        + core::ops::AddAssign<T>
        + core::ops::Sub<T, Output = T>
        + core::ops::SubAssign<T>
        + core::ops::Mul<R, Output = T>
        + core::ops::MulAssign<R>
        + core::ops::Div<T, Output = R>,
    R: Copy + PartialOrd;

// Attribute generic newtype trait definition.
#[newtype_tools::newtype(Amount)]
/// Doc comment.
struct AttrGeneric(f64);

#[rstest::rstest]
#[case::generic(2.0_f64, 3.0_f64, 2.0_f64)]
#[case::man_generic(ManGeneric(2.0_f64), ManGeneric(3.0_f64), 2.0_f64)]
#[case::attr_generic(AttrGeneric(2.0_f64), AttrGeneric(3.0_f64), 2.0_f64)]
#[timeout(core::time::Duration::from_secs(1))]
fn generic_float_newtype_amount<T, R>(#[case] a: T, #[case] b: T, #[case] repr_a: R)
where
    T: Clone
        + Copy
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + From<R>
        + core::ops::Add<T, Output = T>
        + core::ops::AddAssign<T>
        + core::ops::Sub<T, Output = T>
        + core::ops::SubAssign<T>
        + core::ops::Mul<R, Output = T>
        + for<'a> core::ops::Mul<&'a R, Output = T>
        + core::ops::MulAssign<R>
        + core::ops::Div<T, Output = R>,
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
    // Add<Self>
    let res = a + b;
    assert!(res > a);
    // AddAssign<Self>
    let mut res = a;
    res += b;
    assert!(res > a);
    // Sub<Self>
    let res = a - b;
    assert!(res < a);
    // SubAssign<Self>
    let mut res = a;
    res -= b;
    assert!(res < a);
    // Mul<Repr>
    let res = a * repr_a;
    assert!(res > a);
    // MulAssign<Repr>
    let mut res = a;
    res *= repr_a;
    assert!(res > a);
    // Div<Self>
    let res = a / b;
    assert!(res < repr_a);
}
