use crate::{Newtype, NewtypeDerives, NewtypeKind};

/// Expands the parsed `newtype` attribute into a token stream.
///
/// ```ignore
/// #[newtype(Amount)]
/// struct Apples(u64);
/// ```
pub(crate) fn expand_newtype(
    attr: Newtype,
    kind: NewtypeKind,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let newtype = &attr.newtype;
    let inner_ty = &attr.inner_ty;
    let standard_derives = quote::quote!(Clone, Copy, Debug, Default, PartialEq, PartialOrd);
    let standard_derives = if is_int_type(inner_ty) {
        quote::quote!(
            #standard_derives, Eq, Ord, Hash
        )
    } else {
        standard_derives
    };
    let mut derives: proc_macro::TokenStream = match kind {
        NewtypeKind::Amount => quote::quote! {
            #[automatically_derived]
            #[derive(newtype_tools::Newtype, #standard_derives)]
            #[newtype(
                add(#newtype, output = #newtype, with = |l, r| #newtype(l.0 + r.0)),
                add_assign(#newtype, with = |this, other| this.0 += other.0),
                sub(#newtype, output = #newtype, with = |l, r| #newtype(l.0 - r.0)),
                sub_assign(#newtype, with = |this, other| this.0 -= other.0),
                mul(#inner_ty, output = #newtype, with = |l, inner| #newtype(l.0 * inner)),
                mul_assign(#inner_ty, with = |this, inner| this.0 *= inner),
                div(#newtype, output = #inner_ty, with = |l, r| l.0 / r.0)
            )]
            // Guarantees the memory layout is identical to the inner type.
            #[repr(transparent)]
        }
        .into(),
    };
    derives.extend(item);
    derives
}

/// Expands all parsed attributes of a `Newtype` derive into a token stream.
///
/// ```ignore
/// #[derive(Newtype)]
/// #[newtype(from(Oranges, with =  "|oranges| Apples(oranges.0 as u64 * 2)"))]
/// struct Apples(u64);
/// ```
pub(crate) fn expand_newtype_derives(
    newtype_derives: (Newtype, NewtypeDerives),
) -> proc_macro::TokenStream {
    let add = (
        syn::parse_quote!(core::ops::Add),
        syn::parse_quote!(add),
        syn::parse_quote!(core::ops::AddAssign),
        syn::parse_quote!(add_assign),
    );
    let band = (
        syn::parse_quote!(core::ops::BitAnd),
        syn::parse_quote!(bitand),
        syn::parse_quote!(core::ops::BitAndAssign),
        syn::parse_quote!(bitand_assign),
    );
    let bor = (
        syn::parse_quote!(core::ops::BitOr),
        syn::parse_quote!(bitor),
        syn::parse_quote!(core::ops::BitOrAssign),
        syn::parse_quote!(bitor_assign),
    );
    let bxor = (
        syn::parse_quote!(core::ops::BitXor),
        syn::parse_quote!(bitxor),
        syn::parse_quote!(core::ops::BitXorAssign),
        syn::parse_quote!(bitxor_assign),
    );
    let div = (
        syn::parse_quote!(core::ops::Div),
        syn::parse_quote!(div),
        syn::parse_quote!(core::ops::DivAssign),
        syn::parse_quote!(div_assign),
    );
    let mul = (
        syn::parse_quote!(core::ops::Mul),
        syn::parse_quote!(mul),
        syn::parse_quote!(core::ops::MulAssign),
        syn::parse_quote!(mul_assign),
    );
    let rem = (
        syn::parse_quote!(core::ops::Rem),
        syn::parse_quote!(rem),
        syn::parse_quote!(core::ops::RemAssign),
        syn::parse_quote!(rem_assign),
    );
    let shl = (
        syn::parse_quote!(core::ops::Shl),
        syn::parse_quote!(shl),
        syn::parse_quote!(core::ops::ShlAssign),
        syn::parse_quote!(shl_assign),
    );
    let shr = (
        syn::parse_quote!(core::ops::Shr),
        syn::parse_quote!(shr),
        syn::parse_quote!(core::ops::ShrAssign),
        syn::parse_quote!(shr_assign),
    );
    let sub = (
        syn::parse_quote!(core::ops::Sub),
        syn::parse_quote!(sub),
        syn::parse_quote!(core::ops::SubAssign),
        syn::parse_quote!(sub_assign),
    );

    let mut tokens = proc_macro2::TokenStream::new();
    let d = &newtype_derives;
    tokens.extend(expand_newtype_trait(d));
    tokens.extend(expand_from(d));
    tokens.extend(expand_try_from(d));
    tokens.extend(expand_into(d));
    tokens.extend(expand_try_into(d));
    tokens.extend(expand_bin_op(add.0, add.1, d, &d.1.add));
    tokens.extend(expand_assign_op(add.2, add.3, d, &d.1.add_assign));
    tokens.extend(expand_bin_op(band.0, band.1, d, &d.1.bitand));
    tokens.extend(expand_assign_op(band.2, band.3, d, &d.1.bitand_assign));
    tokens.extend(expand_bin_op(bor.0, bor.1, d, &d.1.bitor));
    tokens.extend(expand_assign_op(bor.2, bor.3, d, &d.1.bitor_assign));
    tokens.extend(expand_bin_op(bxor.0, bxor.1, d, &d.1.bitxor));
    tokens.extend(expand_assign_op(bxor.2, bxor.3, d, &d.1.bitxor_assign));
    tokens.extend(expand_bin_op(div.0, div.1, d, &d.1.div));
    tokens.extend(expand_assign_op(div.2, div.3, d, &d.1.div_assign));
    tokens.extend(expand_bin_op(mul.0, mul.1, d, &d.1.mul));
    tokens.extend(expand_assign_op(mul.2, mul.3, d, &d.1.mul_assign));
    tokens.extend(expand_bin_op(rem.0, rem.1, d, &d.1.rem));
    tokens.extend(expand_assign_op(rem.2, rem.3, d, &d.1.rem_assign));
    tokens.extend(expand_bin_op(shl.0, shl.1, d, &d.1.shl));
    tokens.extend(expand_assign_op(shl.2, shl.3, d, &d.1.shl_assign));
    tokens.extend(expand_bin_op(shr.0, shr.1, d, &d.1.shr));
    tokens.extend(expand_assign_op(shr.2, shr.3, d, &d.1.shr_assign));
    tokens.extend(expand_partial_eq(d));
    tokens.extend(expand_bin_op(sub.0, sub.1, d, &d.1.sub));
    tokens.extend(expand_assign_op(sub.2, sub.3, d, &d.1.sub_assign));
    tokens.into()
}

/// Expands newtype trait definition into a token stream.
fn expand_newtype_trait(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let inner_ty = &newtype_derives.0.inner_ty;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
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
fn expand_from(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
    newtype_derives
        .1
        .from
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
fn expand_try_from(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
    newtype_derives
        .1
        .try_from
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
fn expand_into(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
    newtype_derives
        .1
        .into
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
fn expand_try_into(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
    newtype_derives
        .1
        .try_into
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
fn expand_partial_eq(newtype_derives: &(Newtype, NewtypeDerives)) -> proc_macro2::TokenStream {
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
    newtype_derives
        .1
        .partial_eq
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
    newtype_derives: &(Newtype, NewtypeDerives),
    ops: &[(syn::Type, syn::Type, syn::Expr)],
) -> proc_macro2::TokenStream {
    if ops.is_empty() {
        return proc_macro2::TokenStream::new();
    }
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
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
    newtype_derives: &(Newtype, NewtypeDerives),
    ops: &[(syn::Type, syn::Expr)],
) -> proc_macro2::TokenStream {
    if ops.is_empty() {
        return proc_macro2::TokenStream::new();
    }
    let newtype = &newtype_derives.0.newtype;
    let (impl_generics, newtype_generics, r#where) = &newtype_derives.0.generics.split_for_impl();
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

/// Returns `true` if the newtype inner representation is a known integer type.
fn is_int_type(inner_ty: &syn::Type) -> bool {
    let syn::Type::Path(tp) = inner_ty else {
        return false;
    };
    if tp.qself.is_some() || tp.path.segments.len() != 1 {
        return false;
    }
    match &tp.path.segments[0].ident {
        ident if ident == "i8" => true,
        ident if ident == "u8" => true,
        ident if ident == "i16" => true,
        ident if ident == "u16" => true,
        ident if ident == "i32" => true,
        ident if ident == "u32" => true,
        ident if ident == "i64" => true,
        ident if ident == "u64" => true,
        ident if ident == "isize" => true,
        ident if ident == "usize" => true,
        ident if ident == "i128" => true,
        ident if ident == "u128" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_int_type() {
        use super::is_int_type;
        assert!(is_int_type(&syn::parse_quote!(i8)));
        assert!(is_int_type(&syn::parse_quote!(u8)));
        assert!(is_int_type(&syn::parse_quote!(i16)));
        assert!(is_int_type(&syn::parse_quote!(u16)));
        assert!(is_int_type(&syn::parse_quote!(i32)));
        assert!(is_int_type(&syn::parse_quote!(u32)));
        assert!(is_int_type(&syn::parse_quote!(i64)));
        assert!(is_int_type(&syn::parse_quote!(u64)));
        assert!(is_int_type(&syn::parse_quote!(isize)));
        assert!(is_int_type(&syn::parse_quote!(usize)));
        assert!(is_int_type(&syn::parse_quote!(i128)));
        assert!(is_int_type(&syn::parse_quote!(u128)));
        assert!(!is_int_type(&syn::parse_quote!(bool)));
        assert!(!is_int_type(&syn::parse_quote!(char)));
        assert!(!is_int_type(&syn::parse_quote!(str)));
        assert!(!is_int_type(&syn::parse_quote!(String)));
        assert!(!is_int_type(&syn::parse_quote!(Other)));
        assert!(!is_int_type(&syn::parse_quote!(T)));

        assert!(!is_int_type(&syn::parse_quote!(dyn T)));
        assert!(!is_int_type(&syn::parse_quote!(<T as T>::Output)));
    }
}
