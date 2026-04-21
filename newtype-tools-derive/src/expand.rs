use crate::ParseResult;

/// Expands all parsed derives into a token stream.
pub(crate) fn expand_derive(res: &ParseResult) -> syn::Result<proc_macro::TokenStream> {
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.extend(expand_newtype_trait(res));
    tokens.extend(expand_from(res));
    tokens.extend(expand_try_from(res));
    tokens.extend(expand_into(res));
    tokens.extend(expand_try_into(res));
    tokens.extend(expand_add(res));
    tokens.extend(expand_add_assign(res));
    tokens.extend(expand_partial_eq(res));
    tokens.extend(expand_sub(res));
    tokens.extend(expand_sub_assign(res));
    Ok(tokens.into())
}

/// Expands newtype trait definition into a token stream.
fn expand_newtype_trait(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let inner_ty = &res.inner_ty;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    quote::quote! {
        #[automatically_derived]
        impl #impl_generics newtype_tools::Newtype for #newtype #newtype_generics #r#where {
            type Inner = #inner_ty;
        }
        #[automatically_derived]
        impl #impl_generics From<#inner_ty> for #newtype #newtype_generics #r#where {
            fn from(inner: #inner_ty) -> Self {
                #newtype(inner)
            }
        }
        #[automatically_derived]
        impl #impl_generics AsRef<#inner_ty> for #newtype #newtype_generics #r#where {
            fn as_ref(&self) -> &#inner_ty {
                &self.0
            }
        }
    }
}

/// Expands all `from` derives into a token stream.
fn expand_from(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    res.from
        .iter()
        .map(|(from_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics From<#from_ty> for #newtype #newtype_generics #r#where {
                    fn from(value: #from_ty) -> Self {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            }
        })
        .collect()
}

/// Expands all `try_from` derives into a token stream.
fn expand_try_from(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    res.try_from
        .iter()
        .map(|(try_from_ty, error_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics TryFrom<#try_from_ty> for #newtype #newtype_generics #r#where {
                    type Error = #error_ty;
                    fn try_from(value: #try_from_ty) -> Result<Self, Self::Error> {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, value)
                    }
                }
            }
        })
        .collect()
}

/// Expands all `into` derives into a token stream.
/// Note, that it still produces the `from` derives, but with reversed types.
fn expand_into(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    res.into
        .iter()
        .map(|(output_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics From<#newtype #newtype_generics> for #output_ty #r#where {
                    fn from(newtype: #newtype #newtype_generics) -> Self {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, newtype)
                    }
                }
            }
        })
        .collect()
}

/// Expands all `try_into` derives into a token stream.
/// Note, that it still produces the `try_from` derives, but with reversed types.
fn expand_try_into(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    res.try_into
        .iter()
        .map(|(output_ty, error_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics TryFrom<#newtype #newtype_generics> for #output_ty #r#where {
                    type Error = #error_ty;
                    fn try_from(newtype: #newtype #newtype_generics) -> Result<Self, Self::Error> {
                        fn call_inner<I, O, F: FnOnce(I) -> O>(f: F, i: I) -> O {
                            f(i)
                        }
                        call_inner(#expr, newtype)
                    }
                }
            }
        })
        .collect()
}

/// Expands all `add` derives into a token stream.
fn expand_add(res: &ParseResult) -> proc_macro2::TokenStream {
    expand_bin_op(
        syn::parse_quote!(std::ops::Add),
        syn::parse_quote!(add),
        res,
        &res.add,
    )
}

/// Expands all `add_assign` derives into a token stream.
fn expand_add_assign(res: &ParseResult) -> proc_macro2::TokenStream {
    expand_assign_op(
        syn::parse_quote!(std::ops::AddAssign),
        syn::parse_quote!(add_assign),
        res,
        &res.add_assign,
    )
}

/// Expands all `partial_eq` derives into a token stream.
fn expand_partial_eq(res: &ParseResult) -> proc_macro2::TokenStream {
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    res.partial_eq
        .iter()
        .map(|(other_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics PartialEq<#other_ty> for #newtype #newtype_generics #r#where {
                    fn eq(&self, other: &#other_ty) -> bool {
                        fn call_inner<S, I, O, F: FnOnce(S, I) -> O>(f: F, s: S, i: I) -> O {
                            f(s, i)
                        }
                        call_inner(#expr, self, other)
                    }
                }
            }
        })
        .collect()
}

/// Expands all `sub` derives into a token stream.
fn expand_sub(res: &ParseResult) -> proc_macro2::TokenStream {
    expand_bin_op(
        syn::parse_quote!(std::ops::Sub),
        syn::parse_quote!(sub),
        res,
        &res.sub,
    )
}

/// Expands all `sub_assign` derives into a token stream.
fn expand_sub_assign(res: &ParseResult) -> proc_macro2::TokenStream {
    expand_assign_op(
        syn::parse_quote!(std::ops::SubAssign),
        syn::parse_quote!(sub_assign),
        res,
        &res.sub_assign,
    )
}

/// Expands a binary operation into a token stream.
/// `#[newtype(binary_op(type, output = type, with = expr))]`
fn expand_bin_op(
    r#trait: syn::Path,
    method: syn::Ident,
    res: &ParseResult,
    ops: &[(syn::Type, syn::Type, syn::Expr)],
) -> proc_macro2::TokenStream {
    if ops.is_empty() {
        return proc_macro2::TokenStream::new();
    }
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    ops.iter()
        .map(|(rhs_ty, output_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics #r#trait<&#rhs_ty> for &#newtype #newtype_generics #r#where {
                    type Output = #output_ty;
                    fn #method(self, rhs: &#rhs_ty) -> Self::Output {
                        fn call_inner<S, I, O, F: FnOnce(S, I) -> O>(f: F, s: S, i: I) -> O {
                            f(s, i)
                        }
                        call_inner(#expr, self, rhs)
                    }
                }
                #[automatically_derived]
                impl #impl_generics #r#trait<&#rhs_ty> for #newtype #newtype_generics #r#where {
                    type Output = #output_ty;
                    fn #method(self, rhs: &#rhs_ty) -> Self::Output {
                        #r#trait::#method(&self, rhs)
                    }
                }
                #[automatically_derived]
                impl #impl_generics #r#trait<#rhs_ty> for &#newtype #newtype_generics #r#where {
                    type Output = #output_ty;
                    fn #method(self, rhs: #rhs_ty) -> Self::Output { #r#trait::#method(self, &rhs) }
                }
                #[automatically_derived]
                impl #impl_generics #r#trait<#rhs_ty> for #newtype #newtype_generics #r#where {
                    type Output = #output_ty;
                    fn #method(self, rhs: #rhs_ty) -> Self::Output {
                        #r#trait::#method(&self, &rhs)
                    }
                }
            }
        })
        .collect()
}

/// Expands an assignment operation into a token stream.
/// `#[newtype(assignment_op(type, with = expr))]`
fn expand_assign_op(
    r#trait: syn::Path,
    method: syn::Ident,
    res: &ParseResult,
    ops: &[(syn::Type, syn::Expr)],
) -> proc_macro2::TokenStream {
    if ops.is_empty() {
        return proc_macro2::TokenStream::new();
    }
    let newtype = &res.newtype;
    let (impl_generics, newtype_generics, r#where) = &res.generics.split_for_impl();
    ops.iter()
        .map(|(rhs_ty, expr)| {
            quote::quote! {
                #[automatically_derived]
                impl #impl_generics #r#trait<&#rhs_ty> for #newtype #newtype_generics #r#where {
                    fn #method(&mut self, rhs: &#rhs_ty) {
                        fn call_inner<S, I, F: FnOnce(S, I)>(f: F, s: S, i: I) {
                            f(s, i)
                        }
                        call_inner(#expr, self, rhs)
                    }
                }
                #[automatically_derived]
                impl #impl_generics #r#trait<#rhs_ty> for #newtype #newtype_generics #r#where {
                    fn #method(&mut self, rhs: #rhs_ty) { #r#trait::#method(self, &rhs) }
                }
            }
        })
        .collect()
}
