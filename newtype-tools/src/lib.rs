#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
pub use newtype_tools_derive::Newtype;

/// `Newtype` trait defines conversions from and into the inner type.
///
/// This trait is automatically derived for all types annotated with
/// `#[derive(Newtype)]`.
pub trait Newtype {
    /// The inner type.
    type Inner;

    /// Creates a new `newtype` instance from the inner representation.
    fn new(inner: Self::Inner) -> Self;

    /// Unwraps the value, consuming the `newtype`.
    fn into_inner(self) -> Self::Inner;
}
