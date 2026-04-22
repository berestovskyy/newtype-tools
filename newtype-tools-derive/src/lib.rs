#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod expand;
mod parse;

/// Expands the derive macro into a `TokenStream`.
#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    parse_input_and_expand_derive(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Expands the `DeriveInput` into a `syn::Result<TokenStream>`.
fn parse_input_and_expand_derive(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let derive = parse::parse_input(input)?;
    expand::expand_derive(&derive)
}

/// All parsed newtype derives.
#[derive(Debug)]
struct ParseResult {
    /// Top-level newtype identifier.
    newtype: syn::Ident,
    /// Inner type field type.
    inner_ty: syn::Type,
    /// Newtype generics.
    generics: syn::Generics,
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

impl ParseResult {
    /// Creates a new `ParseResult` instance.
    fn new(newtype: syn::Ident, inner_ty: syn::Type, generics: syn::Generics) -> Self {
        Self {
            newtype,
            inner_ty,
            generics,
            from: Vec::default(),
            try_from: Vec::default(),
            into: Vec::default(),
            try_into: Vec::default(),
            add: Vec::default(),
            add_assign: Vec::default(),
            bitand: Vec::default(),
            bitand_assign: Vec::default(),
            bitor: Vec::default(),
            bitor_assign: Vec::default(),
            bitxor: Vec::default(),
            bitxor_assign: Vec::default(),
            div: Vec::default(),
            div_assign: Vec::default(),
            mul: Vec::default(),
            mul_assign: Vec::default(),
            rem: Vec::default(),
            rem_assign: Vec::default(),
            shl: Vec::default(),
            shl_assign: Vec::default(),
            shr: Vec::default(),
            shr_assign: Vec::default(),
            partial_eq: Vec::default(),
            sub: Vec::default(),
            sub_assign: Vec::default(),
        }
    }
}
