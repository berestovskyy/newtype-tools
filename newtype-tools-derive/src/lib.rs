#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod expand;
mod parse;

/// Parses and expands a `newtype` attribute kind into a token stream.
///
/// The crate supports predefined sets of newtype properties. The concept is similar
/// to the `phantom_newtype` crate but avoids its limitations, as the newtype
/// generated here is a distinct Rust type. This allows new traits
/// to be implemented easily for the type and makes the set of derived traits
/// simple to extend.
///
/// ```ignore
/// #[newtype(Amount)]
/// #[derive(Default)]
/// struct Apples(u64);
/// ```
#[proc_macro_attribute]
pub fn newtype(attr: TokenStream, item: TokenStream) -> TokenStream {
    let kind = match parse::parse_newtype_kind(attr.into()) {
        Ok(kind) => kind,
        Err(err) => return err.to_compile_error().into(),
    };
    let input = item.clone();
    let newtype = match parse::parse_newtype(input) {
        Ok(newtype) => newtype,
        Err(err) => return err.to_compile_error().into(),
    };
    expand::expand_newtype(newtype, kind, item)
}

/// Parses and expands a `Newtype` derive into a token stream.
///
/// ```ignore
/// #[derive(Newtype)]
/// #[newtype(from(Oranges, with =  "|oranges| Apples(oranges.0 as u64 * 2)"))]
/// struct Apples(u64);
/// ```
#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn newtype_derive(input: TokenStream) -> TokenStream {
    let newtype_derives = match parse::parse_newtype_derives(input.into()) {
        Ok(newtype_derives) => newtype_derives,
        Err(err) => return err.to_compile_error().into(),
    };
    expand::expand_newtype_derives(newtype_derives)
}

/// Structured representation of a `newtype`.
///
/// ```ignore
/// #[newtype(Amount)]
/// struct Apples(u64);
/// ```
struct Newtype {
    /// Top-level newtype identifier.
    newtype: syn::Ident,
    /// Inner type field type.
    inner_ty: syn::Type,
    /// Newtype generics.
    generics: syn::Generics,
}

impl Newtype {
    /// Creates a new `Newtype` instance.
    fn new(newtype: syn::Ident, inner_ty: syn::Type, generics: syn::Generics) -> Self {
        Self {
            newtype,
            inner_ty,
            generics,
        }
    }
}

/// Structured representation of all `Newtype` derives.
///
/// ```ignore
/// #[derive(Newtype)]
/// #[newtype(from(Oranges, with =  "|oranges| Apples(oranges.0 as u64 * 2)"))]
/// struct Apples(u64);
/// ```
#[derive(Default)]
struct NewtypeDerives {
    /// Tuples of `(from type, conversion expression)`.
    from: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(from type, error type, conversion expression)`.
    try_from: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(into type, conversion expression)`.
    into: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(into type, error type, conversion expression)`.
    try_into: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    add: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    add_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    bitand: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    bitand_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    bitor: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    bitor_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    bitxor: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    bitxor_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    div: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    div_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    mul: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    mul_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    rem: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    rem_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    shl: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    shl_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, add expression)`.
    shr: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, add-assign expression)`.
    shr_assign: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(other type, comparison expression)`.
    partial_eq: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, output type, sub expression)`.
    sub: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(rhs type, sub-assign expression)`.
    sub_assign: Vec<(syn::Type, syn::Expr)>,
}

/// Defines `newtype` attribute kind.
///
/// ```ignore
/// #[newtype(Amount)]
/// struct Apples(u64);
/// ```
#[derive(Debug, PartialEq)]
enum NewtypeKind {
    Amount,
    Id,
}

impl core::fmt::Display for NewtypeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Amount => f.write_str("Amount"),
            Self::Id => f.write_str("Id"),
        }
    }
}

impl TryFrom<&syn::Ident> for NewtypeKind {
    type Error = syn::Error;

    fn try_from(value: &syn::Ident) -> Result<Self, Self::Error> {
        match value {
            ident if ident == "Amount" => Ok(Self::Amount),
            ident if ident == "Id" => Ok(Self::Id),
            _ => Err(syn::Error::new_spanned(value, "expected 'Amount' or 'Id'")),
        }
    }
}

/// Defines `Newtype` derive attribute type.
///
/// ```ignore
/// #[derive(Newtype)]
/// #[newtype(from(Oranges, with =  "|oranges| Apples(oranges.0 as u64 * 2)"))]
/// struct Apples(u64);
/// ```
#[derive(Debug, PartialEq)]
enum DeriveType {
    From,
    TryFrom,
    Into,
    TryInto,
    Add,
    AddAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    PartialEq,
    Sub,
    SubAssign,
}

impl core::fmt::Display for DeriveType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::From => f.write_str("from"),
            Self::TryFrom => f.write_str("try_from"),
            Self::Into => f.write_str("into"),
            Self::TryInto => f.write_str("try_into"),
            Self::Add => f.write_str("add"),
            Self::AddAssign => f.write_str("add_assign"),
            Self::BitAnd => f.write_str("bitand"),
            Self::BitAndAssign => f.write_str("bitand_assign"),
            Self::BitOr => f.write_str("bitor"),
            Self::BitOrAssign => f.write_str("bitor_assign"),
            Self::BitXor => f.write_str("bitxor"),
            Self::BitXorAssign => f.write_str("bitxor_assign"),
            Self::Div => f.write_str("div"),
            Self::DivAssign => f.write_str("div_assign"),
            Self::Mul => f.write_str("mul"),
            Self::MulAssign => f.write_str("mul_assign"),
            Self::Rem => f.write_str("rem"),
            Self::RemAssign => f.write_str("rem_assign"),
            Self::Shl => f.write_str("shl"),
            Self::ShlAssign => f.write_str("shl_assign"),
            Self::Shr => f.write_str("shr"),
            Self::ShrAssign => f.write_str("shr_assign"),
            Self::PartialEq => f.write_str("partial_eq"),
            Self::Sub => f.write_str("sub"),
            Self::SubAssign => f.write_str("sub_assign"),
        }
    }
}

impl TryFrom<Option<&syn::Ident>> for DeriveType {
    type Error = syn::Error;

    fn try_from(value: Option<&syn::Ident>) -> Result<Self, Self::Error> {
        match value {
            Some(ident) if ident == "from" => Ok(Self::From),
            Some(ident) if ident == "try_from" => Ok(Self::TryFrom),
            Some(ident) if ident == "into" => Ok(Self::Into),
            Some(ident) if ident == "try_into" => Ok(Self::TryInto),
            Some(ident) if ident == "add" => Ok(Self::Add),
            Some(ident) if ident == "add_assign" => Ok(Self::AddAssign),
            Some(ident) if ident == "bitand" => Ok(Self::BitAnd),
            Some(ident) if ident == "bitand_assign" => Ok(Self::BitAndAssign),
            Some(ident) if ident == "bitor" => Ok(Self::BitOr),
            Some(ident) if ident == "bitor_assign" => Ok(Self::BitOrAssign),
            Some(ident) if ident == "bitxor" => Ok(Self::BitXor),
            Some(ident) if ident == "bitxor_assign" => Ok(Self::BitXorAssign),
            Some(ident) if ident == "div" => Ok(Self::Div),
            Some(ident) if ident == "div_assign" => Ok(Self::DivAssign),
            Some(ident) if ident == "mul" => Ok(Self::Mul),
            Some(ident) if ident == "mul_assign" => Ok(Self::MulAssign),
            Some(ident) if ident == "rem" => Ok(Self::Rem),
            Some(ident) if ident == "rem_assign" => Ok(Self::RemAssign),
            Some(ident) if ident == "shl" => Ok(Self::Shl),
            Some(ident) if ident == "shl_assign" => Ok(Self::ShlAssign),
            Some(ident) if ident == "shr" => Ok(Self::Shr),
            Some(ident) if ident == "shr_assign" => Ok(Self::ShrAssign),
            Some(ident) if ident == "partial_eq" => Ok(Self::PartialEq),
            Some(ident) if ident == "sub" => Ok(Self::Sub),
            Some(ident) if ident == "sub_assign" => Ok(Self::SubAssign),
            _ => Err(syn::Error::new_spanned(
                value,
                "expected `(try_)from`, `(try_)into`, `add(_assign)`, `bitand(_assign)`, \
                `bitor(_assign)`, `bitxor(_assign)`, `div(_assign)`, `mul(_assign)`, \
                `rem(_assign)`, `shl(_assign)`, `shr(_assign)`, `partial_eq`, or `sub(_assign)`",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn newtype_kind_display_roundtrip() {
        use super::NewtypeKind;
        assert_eq!(format!("{}", NewtypeKind::Amount), "Amount");
        assert_eq!(format!("{}", NewtypeKind::Id), "Id");
    }

    #[test]
    fn derive_type_display_roundtrip() {
        use super::DeriveType;
        assert_eq!(format!("{}", DeriveType::From), "from");
        assert_eq!(format!("{}", DeriveType::TryFrom), "try_from");
        assert_eq!(format!("{}", DeriveType::Into), "into");
        assert_eq!(format!("{}", DeriveType::TryInto), "try_into");
        assert_eq!(format!("{}", DeriveType::Add), "add");
        assert_eq!(format!("{}", DeriveType::AddAssign), "add_assign");
        assert_eq!(format!("{}", DeriveType::BitAnd), "bitand");
        assert_eq!(format!("{}", DeriveType::BitAndAssign), "bitand_assign");
        assert_eq!(format!("{}", DeriveType::BitOr), "bitor");
        assert_eq!(format!("{}", DeriveType::BitOrAssign), "bitor_assign");
        assert_eq!(format!("{}", DeriveType::BitXor), "bitxor");
        assert_eq!(format!("{}", DeriveType::BitXorAssign), "bitxor_assign");
        assert_eq!(format!("{}", DeriveType::Div), "div");
        assert_eq!(format!("{}", DeriveType::DivAssign), "div_assign");
        assert_eq!(format!("{}", DeriveType::Mul), "mul");
        assert_eq!(format!("{}", DeriveType::MulAssign), "mul_assign");
        assert_eq!(format!("{}", DeriveType::Rem), "rem");
        assert_eq!(format!("{}", DeriveType::RemAssign), "rem_assign");
        assert_eq!(format!("{}", DeriveType::Shl), "shl");
        assert_eq!(format!("{}", DeriveType::ShlAssign), "shl_assign");
        assert_eq!(format!("{}", DeriveType::Shr), "shr");
        assert_eq!(format!("{}", DeriveType::ShrAssign), "shr_assign");
        assert_eq!(format!("{}", DeriveType::PartialEq), "partial_eq");
        assert_eq!(format!("{}", DeriveType::Sub), "sub");
        assert_eq!(format!("{}", DeriveType::SubAssign), "sub_assign");
    }
}
