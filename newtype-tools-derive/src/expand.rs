use crate::ParseResult;

/// Expands all parsed derives into a token stream.
pub(crate) fn expand_derive(res: &ParseResult) -> syn::Result<proc_macro::TokenStream> {
    let add = (
        syn::parse_quote!(std::ops::Add),
        syn::parse_quote!(add),
        syn::parse_quote!(std::ops::AddAssign),
        syn::parse_quote!(add_assign),
    );
    let band = (
        syn::parse_quote!(std::ops::BitAnd),
        syn::parse_quote!(bitand),
        syn::parse_quote!(std::ops::BitAndAssign),
        syn::parse_quote!(bitand_assign),
    );
    let bor = (
        syn::parse_quote!(std::ops::BitOr),
        syn::parse_quote!(bitor),
        syn::parse_quote!(std::ops::BitOrAssign),
        syn::parse_quote!(bitor_assign),
    );
    let bxor = (
        syn::parse_quote!(std::ops::BitXor),
        syn::parse_quote!(bitxor),
        syn::parse_quote!(std::ops::BitXorAssign),
        syn::parse_quote!(bitxor_assign),
    );
    let div = (
        syn::parse_quote!(std::ops::Div),
        syn::parse_quote!(div),
        syn::parse_quote!(std::ops::DivAssign),
        syn::parse_quote!(div_assign),
    );
    let mul = (
        syn::parse_quote!(std::ops::Mul),
        syn::parse_quote!(mul),
        syn::parse_quote!(std::ops::MulAssign),
        syn::parse_quote!(mul_assign),
    );
    let rem = (
        syn::parse_quote!(std::ops::Rem),
        syn::parse_quote!(rem),
        syn::parse_quote!(std::ops::RemAssign),
        syn::parse_quote!(rem_assign),
    );
    let shl = (
        syn::parse_quote!(std::ops::Shl),
        syn::parse_quote!(shl),
        syn::parse_quote!(std::ops::ShlAssign),
        syn::parse_quote!(shl_assign),
    );
    let shr = (
        syn::parse_quote!(std::ops::Shr),
        syn::parse_quote!(shr),
        syn::parse_quote!(std::ops::ShrAssign),
        syn::parse_quote!(shr_assign),
    );
    let sub = (
        syn::parse_quote!(std::ops::Sub),
        syn::parse_quote!(sub),
        syn::parse_quote!(std::ops::SubAssign),
        syn::parse_quote!(sub_assign),
    );

    let mut tokens = proc_macro2::TokenStream::new();
    tokens.extend(expand_newtype_trait(res));
    tokens.extend(expand_from(res));
    tokens.extend(expand_try_from(res));
    tokens.extend(expand_into(res));
    tokens.extend(expand_try_into(res));
    tokens.extend(expand_bin_op(add.0, add.1, res, &res.add));
    tokens.extend(expand_assign_op(add.2, add.3, res, &res.add_assign));
    tokens.extend(expand_bin_op(band.0, band.1, res, &res.bitand));
    tokens.extend(expand_assign_op(band.2, band.3, res, &res.bitand_assign));
    tokens.extend(expand_bin_op(bor.0, bor.1, res, &res.bitor));
    tokens.extend(expand_assign_op(bor.2, bor.3, res, &res.bitor_assign));
    tokens.extend(expand_bin_op(bxor.0, bxor.1, res, &res.bitxor));
    tokens.extend(expand_assign_op(bxor.2, bxor.3, res, &res.bitxor_assign));
    tokens.extend(expand_bin_op(div.0, div.1, res, &res.div));
    tokens.extend(expand_assign_op(div.2, div.3, res, &res.div_assign));
    tokens.extend(expand_bin_op(mul.0, mul.1, res, &res.mul));
    tokens.extend(expand_assign_op(mul.2, mul.3, res, &res.mul_assign));
    tokens.extend(expand_bin_op(rem.0, rem.1, res, &res.rem));
    tokens.extend(expand_assign_op(rem.2, rem.3, res, &res.rem_assign));
    tokens.extend(expand_bin_op(shl.0, shl.1, res, &res.shl));
    tokens.extend(expand_assign_op(shl.2, shl.3, res, &res.shl_assign));
    tokens.extend(expand_bin_op(shr.0, shr.1, res, &res.shr));
    tokens.extend(expand_assign_op(shr.2, shr.3, res, &res.shr_assign));
    tokens.extend(expand_partial_eq(res));
    tokens.extend(expand_bin_op(sub.0, sub.1, res, &res.sub));
    tokens.extend(expand_assign_op(sub.2, sub.3, res, &res.sub_assign));
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
