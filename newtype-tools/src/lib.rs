#![doc = include_str!("../README.md")]

pub mod iter;

pub use iter::NewtypeIterator;
#[cfg(feature = "derive")]
pub use newtype_tools_derive::Newtype;

/// `Newtype` trait defines conversions from and into the inner type.
///
/// This trait is automatically derived for all types annotated with
/// `#[derive(Newtype)]`.
pub trait Newtype {
    /// The inner type.
    type Inner;

    /// Unwraps the value returning a reference to the inner type.
    fn as_inner(&self) -> &Self::Inner;
}
