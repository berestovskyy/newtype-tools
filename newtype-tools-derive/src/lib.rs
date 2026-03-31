use proc_macro::TokenStream;

mod expand;
mod parse;

/// All parsed newtype derives.
#[derive(Debug)]
struct ParseResult {
    /// Top-level identifier.
    pub ident: syn::Ident,
    /// Tuples of `(input type, conversion expression)`.
    pub from: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(input type, error type, conversion expression)`.
    pub try_from: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(output type, conversion expression)`.
    pub into: Vec<(syn::Type, syn::Expr)>,
    /// Tuples of `(input type, error type, conversion expression)`.
    pub try_into: Vec<(syn::Type, syn::Type, syn::Expr)>,
    /// Tuples of `(other type, comparison expression)`.
    pub partial_eq: Vec<(syn::Type, syn::Expr)>,
    /// Range iterator trait.
    pub range_iter: Option<syn::Type>,
}

impl ParseResult {
    fn new(ident: syn::Ident) -> Self {
        Self {
            ident,
            from: Vec::default(),
            try_from: Vec::default(),
            into: Vec::default(),
            try_into: Vec::default(),
            partial_eq: Vec::default(),
            range_iter: None,
        }
    }
}

/// Expands the derive macro into a TokenStream.
#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    parse_input_and_expand_derive(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Expands the derive macro into a TokenStream.
fn parse_input_and_expand_derive(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let derive = parse::parse_input(input)?;
    expand::expand_derive(&derive)
}
