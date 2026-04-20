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
    newtype_ident: syn::Ident,
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
    /// Tuples of `(other type, comparison expression)`.
    partial_eq: Vec<(syn::Type, syn::Expr)>,
}

impl ParseResult {
    /// Creates a new `ParseResult` instance.
    fn new(newtype_ident: syn::Ident, inner_ty: syn::Type, generics: syn::Generics) -> Self {
        Self {
            newtype_ident,
            inner_ty,
            generics,
            from: Vec::default(),
            try_from: Vec::default(),
            into: Vec::default(),
            try_into: Vec::default(),
            add: Vec::default(),
            add_assign: Vec::default(),
            partial_eq: Vec::default(),
        }
    }
}
