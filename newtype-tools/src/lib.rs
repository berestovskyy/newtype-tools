#![doc = include_str!("../README.md")]

pub mod iter;

pub use iter::IntoIter;
pub use iter::Iter;
pub use iter::Iterator;
#[cfg(feature = "derive")]
pub use newtype_tools_derive::Newtype;

/// `Newtype` trait defines the internal representation of the `newtype`.
///
/// This trait is automatically derived for all types annotated with `#[derive(Newtype)]`
/// along with the `From<Self::Inner>` and `AsRef<Self::Inner>` traits to convert
/// between the inner type and the newtype.
pub trait Newtype: AsRef<Self::Inner> + From<Self::Inner> {
    /// The inner type.
    type Inner;
}
