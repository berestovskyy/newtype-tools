use crate::ParseResult;

/// Expands all parsed derives into a token stream.
pub(crate) fn expand_derive(res: &ParseResult) -> syn::Result<proc_macro::TokenStream> {
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.extend(expand_from(res)?);
    tokens.extend(expand_into(res)?);
    tokens.extend(expand_partial_eq(res)?);
    tokens.extend(expand_iter(res)?);
    Ok(tokens.into())
}

/// Expands all `from` derives into a token stream.
fn expand_from(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &res.ident;
    res.from
        .iter()
        .map(|(input_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl From<#input_ty> for #ident {
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

/// Expands all `into` derives into a token stream.
/// Note, that it still produces the `from` derives, but with reversed types.
fn expand_into(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &res.ident;
    res.into
        .iter()
        .map(|(output_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl From<#ident> for #output_ty {
                    fn from(value: #ident) -> Self {
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
    let ident = &res.ident;
    res.partial_eq
        .iter()
        .map(|(other_ty, expr)| {
            Ok(quote::quote! {
                #[automatically_derived]
                impl PartialEq<#other_ty> for #ident {
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

/// Expands `iter` derive into a token stream.
fn expand_iter(res: &ParseResult) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &res.ident;
    let ident_iterator = quote::format_ident!("{ident}Iterator");
    let doc_msg = format!(
        "Creates a specialized iterator for RangeBounds<{ident}>.\n\n\
            # Examples\n\n\
            ```\n\
            for x in {ident}::iter({ident}(0)..={ident}(10)) {{\n\
                // ...\n\
            }}\n\
            ```",
    );
    Ok(if let Some(inner_ty) = &res.iter {
        quote::quote! {
            #[automatically_derived]
            struct #ident_iterator {
                inner_cur: #inner_ty,
                inner_end: #inner_ty,
                exhausted: bool,
            }

            #[automatically_derived]
            impl Iterator for #ident_iterator {
                type Item = #ident;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.exhausted || self.inner_cur > self.inner_end {
                        return None;
                    }
                    let cur = #ident(self.inner_cur);
                    let next = self.inner_cur.saturating_add(1);
                    self.exhausted = next > self.inner_end || next == self.inner_cur;
                    self.inner_cur = next;
                    Some(cur)
                }
            }

            impl #ident {
                #[doc = #doc_msg]
                pub fn iter<R: ::std::ops::RangeBounds<#ident>>(range: R) -> impl Iterator<Item = Self> {
                    use ::std::ops::Bound;
                    let inner_cur = match range.start_bound() {
                        Bound::Included(s) => s.0,
                        Bound::Excluded(s) => s.0.saturating_add(1),
                        Bound::Unbounded => #inner_ty::MIN,
                    };
                    let inner_end = match range.end_bound() {
                        Bound::Included(e) => e.0,
                        Bound::Excluded(e) => e.0.saturating_sub(1),
                        Bound::Unbounded => #inner_ty::MAX,
                    };
                    ApplesIterator {
                        inner_cur,
                        inner_end,
                        exhausted: false,
                    }
                }
            }
        }
    } else {
        proc_macro2::TokenStream::new()
    })
}
