use crate::ParseResult;

/// Expands all parsed derives into a token stream.
pub(crate) fn expand_derive(res: &ParseResult) -> syn::Result<proc_macro::TokenStream> {
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.extend(expand_newtype_trait(res)?);
    tokens.extend(expand_from(res)?);
    tokens.extend(expand_try_from(res)?);
    tokens.extend(expand_into(res)?);
    tokens.extend(expand_try_into(res)?);
    tokens.extend(expand_add(res)?);
    tokens.extend(expand_partial_eq(res)?);
    Ok(tokens.into())
}

/// Expands newtype trait definition into a token stream.
fn expand_newtype_trait(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    let inner_ty = &res.inner_ty;
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    Ok(quote::quote! {
        #[automatically_derived]
        impl #impl_generics newtype_tools::Newtype for #newtype #newtype_generics #where_clause {
            type Inner = #inner_ty;
        }
        #[automatically_derived]
        impl #impl_generics From<#inner_ty> for #newtype #newtype_generics #where_clause {
            fn from(inner: #inner_ty) -> Self {
                #newtype(inner)
            }
        }
        #[automatically_derived]
        impl #impl_generics AsRef<#inner_ty> for #newtype #newtype_generics #where_clause {
            fn as_ref(&self) -> &#inner_ty {
                &self.0
            }
        }
    })
}

/// Expands all `from` derives into a token stream.
fn expand_from(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.from
        .iter()
        .map(|(from_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics From<#from_ty> for #newtype #newtype_generics #where_clause {
                    fn from(value: #from_ty) -> Self {
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
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.try_from
        .iter()
        .map(|(try_from_ty, error_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics TryFrom<#try_from_ty> for #newtype #newtype_generics #where_clause {
                    type Error = #error_ty;
                    fn try_from(value: #try_from_ty) -> Result<Self, Self::Error> {
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
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.into
        .iter()
        .map(|(output_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics From<#newtype #newtype_generics> for #output_ty #where_clause {
                    fn from(newtype: #newtype #newtype_generics) -> Self {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, newtype)
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
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.try_into
        .iter()
        .map(|(output_ty, error_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics TryFrom<#newtype #newtype_generics> for #output_ty #where_clause {
                    type Error = #error_ty;
                    fn try_from(newtype: #newtype #newtype_generics) -> Result<Self, Self::Error> {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, newtype)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `add` derives into a token stream.
fn expand_add(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.add
        .iter()
        .map(|(rhs_ty, output_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics std::ops::Add<#rhs_ty> for #newtype #newtype_generics #where_clause {
                    type Output = #output_ty;
                    fn add(self, rhs: #rhs_ty) -> Self::Output {
                        fn call_inner<S, I, O, F: FnOnce(S, I) -> O>(f: F, s: S, i: I) -> O {
                            f(s, i)
                        }
                        call_inner(#expr, self, rhs)
                    }
                }
            })
        })
        .collect()
}

/// Expands all `partial_eq` derives into a token stream.
fn expand_partial_eq(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let newtype = &res.newtype_ident;
    let (impl_generics, newtype_generics, where_clause) = &res.generics.split_for_impl();
    res.partial_eq
        .iter()
        .map(|(other_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl #impl_generics PartialEq<#other_ty> for #newtype #newtype_generics #where_clause {
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
