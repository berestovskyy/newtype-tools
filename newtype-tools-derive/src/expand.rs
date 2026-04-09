use crate::ParseResult;

/// Expands all parsed derives into a token stream.
pub(crate) fn expand_derive(res: &ParseResult) -> syn::Result<proc_macro::TokenStream> {
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.extend(expand_newtype_trait(res)?);
    tokens.extend(expand_from(res)?);
    tokens.extend(expand_try_from(res)?);
    tokens.extend(expand_into(res)?);
    tokens.extend(expand_try_into(res)?);
    tokens.extend(expand_partial_eq(res)?);
    Ok(tokens.into())
}

/// Expands newtype trait definition into a token stream.
fn expand_newtype_trait(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    let inner_ty = &res.inner_ty;
    let (impl_generics, ty_generics, where_clause) = &res.generics.split_for_impl();
    Ok(quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::newtype_tools::Newtype for #newtype #ty_generics #where_clause {
            type Inner = #inner_ty;
            fn as_inner(&self) -> &Self::Inner {
                &self.0
            }
        }
        #[automatically_derived]
        impl #impl_generics From<#inner_ty> for #newtype #ty_generics #where_clause {
            fn from(inner: #inner_ty) -> Self {
                #newtype(inner)
            }
        }
    })
}

/// Expands all `from` derives into a token stream.
fn expand_from(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    res.from
        .iter()
        .map(|(input_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl From<#input_ty> for #newtype {
                    fn from(value: #input_ty) -> Self {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `try_from` derives into a token stream.
fn expand_try_from(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    res.try_from
        .iter()
        .map(|(input_ty, error_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl TryFrom<#input_ty> for #newtype {
                    type Error = #error_ty;
                    fn try_from(value: #input_ty) -> Result<Self, Self::Error> {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `into` derives into a token stream.
/// Note, that it still produces the `from` derives, but with reversed types.
fn expand_into(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    res.into
        .iter()
        .map(|(output_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl From<#newtype> for #output_ty {
                    fn from(value: #newtype) -> Self {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `try_into` derives into a token stream.
/// Note, that it still produces the `try_from` derives, but with reversed types.
fn expand_try_into(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    res.try_into
        .iter()
        .map(|(output_ty, error_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl TryFrom<#newtype> for #output_ty {
                    type Error = #error_ty;
                    fn try_from(value: #newtype) -> Result<Self, Self::Error> {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `partial_eq` derives into a token stream.
fn expand_partial_eq(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    res.partial_eq
        .iter()
        .map(|(other_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl PartialEq<#other_ty> for #newtype {
                    fn eq(&self, other: &#other_ty) -> bool {
                        fn call_inner<S, I, O, F: FnOnce(S, I) -> O>(f: F, s: S, i: I) -> O {
                            f(s, i)
                        }
                        call_inner(#expr, self, other)
                    }
                }
            })
        })
        .collect()
}
