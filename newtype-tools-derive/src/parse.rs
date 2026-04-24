use crate::{DeriveType, Newtype, NewtypeDerives, NewtypeKind};
use alloc::vec::Vec;

mod kw {
    syn::custom_keyword!(error);
    syn::custom_keyword!(output);
    syn::custom_keyword!(with);
}

/// Newtype attribute name.
const NEWTYPE_ATTR_NAME: &str = "newtype";

/// Parses `newtype` attribute kind.
///
/// ```ignore
/// #[newtype(Amount)]
/// struct Apples(u64);
/// ```
pub(crate) fn parse_newtype_kind(attr: proc_macro2::TokenStream) -> syn::Result<NewtypeKind> {
    let attr_span = syn::spanned::Spanned::span(&attr);
    let parser = |input: syn::parse::ParseStream| parse_lit_or::<syn::Ident>(&input);
    use syn::parse::Parser;
    let ident = parser
        .parse2(attr)
        .map_err(|_| syn::Error::new(attr_span, "expected `#[newtype(NewtypeKind)]`"))?;
    NewtypeKind::try_from(&ident)
}

/// Parses `newtype` attribute and produces its structured representation.
///
/// ```ignore
/// #[newtype(Amount)]
/// struct Apples(u64);
/// ```
pub(crate) fn parse_newtype(input: proc_macro::TokenStream) -> syn::Result<Newtype> {
    let derive_input = syn::parse::<syn::DeriveInput>(input)?;
    let inner_ty = parse_derive_input_data(&derive_input.data)?;
    let attribute = Newtype::new(derive_input.ident, inner_ty, derive_input.generics);
    Ok(attribute)
}

/// Parses all attributes of a `Newtype` derive and produces their structured representation.
///
/// ```ignore
/// #[derive(Newtype)]
/// #[newtype(from(Oranges, with =  "|oranges| Apples(oranges.0 as u64 * 2)"))]
/// struct Apples(u64);
/// ```
pub(crate) fn parse_newtype_derives(
    input: proc_macro2::TokenStream,
) -> syn::Result<(Newtype, NewtypeDerives)> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;
    let inner_ty = parse_derive_input_data(&derive_input.data)?;
    let attr = Newtype::new(derive_input.ident, inner_ty, derive_input.generics);
    let mut derives = NewtypeDerives::default();
    for attr in derive_input.attrs {
        // Just skip all other top-level attributes.
        if !attr.path().is_ident(NEWTYPE_ATTR_NAME) {
            continue;
        }
        parse_top_level_meta(attr.meta, &mut derives)?;
    }
    Ok((attr, derives))
}

/// Parses the first struct field to get the inner type.
///
/// For `struct Newtype(type)` returns `type`.
pub(crate) fn parse_derive_input_data(data: &syn::Data) -> syn::Result<syn::Type> {
    let msg = "expected `struct Newtype(inner_type)`";

    let field = match &data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                fields.unnamed.first().unwrap()
            }
            syn::Fields::Unnamed(fields) => Err(syn::Error::new_spanned(fields, msg))?,
            syn::Fields::Named(fields) => Err(syn::Error::new_spanned(fields, msg))?,
            syn::Fields::Unit => Err(syn::Error::new_spanned(s.struct_token, msg))?,
        },
        syn::Data::Enum(e) => Err(syn::Error::new_spanned(e.enum_token, msg))?,
        syn::Data::Union(u) => Err(syn::Error::new_spanned(u.union_token, msg))?,
    };
    let inner_ty = field.ty.clone();
    Ok(inner_ty)
}

/// Parses a single top-level attribute's meta and fills in its structured representation.
fn parse_top_level_meta(meta: syn::Meta, res: &mut NewtypeDerives) -> syn::Result<()> {
    match meta {
        // `#[newtype]`
        syn::Meta::Path(path) => parse_top_level_path(path, res)?,
        // `#[newtype(attr1, attr2)]`
        syn::Meta::List(list) => parse_top_level_list(list, res)?,
        // `#[newtype = value]`
        syn::Meta::NameValue(name_value) => parse_top_level_name_value(name_value, res)?,
    }
    Ok(())
}

/// Parses a single top-level path attribute and fills in its structured representation:
/// `#[newtype]`
fn parse_top_level_path(path: syn::Path, _res: &mut NewtypeDerives) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        path,
        "expected `#[newtype(attr1, attr2)]`",
    ))
}

/// Parses a single top-level list attribute and fills in its structured representation:
/// `#[newtype(attr1, attr2)]`
fn parse_top_level_list(list: syn::MetaList, res: &mut NewtypeDerives) -> syn::Result<()> {
    let args = list.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    )?;
    for meta in &args {
        parse_nested_meta(meta, res)?;
    }
    Ok(())
}

/// Parses a single top-level name-value attribute and fills in its structured representation.
/// `#[newtype = value]`
fn parse_top_level_name_value(
    name_value: syn::MetaNameValue,
    _res: &mut NewtypeDerives,
) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        name_value,
        "expected `#[newtype(attr1, attr2)]`",
    ))
}

/// Parses a single nested attribute's meta and fills in its structured representation.
fn parse_nested_meta(meta: &syn::Meta, res: &mut NewtypeDerives) -> syn::Result<()> {
    let derive_type = DeriveType::try_from(meta.path().get_ident())?;
    match meta {
        // `#[newtype(attr)]`
        syn::Meta::Path(path) => parse_nested_path(derive_type, path, res)?,
        // `#[newtype(attr1, attr2)]`
        syn::Meta::List(list) => parse_nested_list(derive_type, list, res)?,
        // `#[newtype(attr = value)]`
        syn::Meta::NameValue(name_value) => parse_nested_name_value(derive_type, name_value, res)?,
    }
    Ok(())
}

/// Parses a single nested path attribute and fills in its structured representation:
/// `#[newtype(attr)]`
fn parse_nested_path(
    derive_type: DeriveType,
    path: &syn::Path,
    _res: &mut NewtypeDerives,
) -> syn::Result<()> {
    match derive_type {
        DeriveType::From
        | DeriveType::TryFrom
        | DeriveType::Into
        | DeriveType::TryInto
        | DeriveType::Add
        | DeriveType::AddAssign
        | DeriveType::BitAnd
        | DeriveType::BitAndAssign
        | DeriveType::BitOr
        | DeriveType::BitOrAssign
        | DeriveType::BitXor
        | DeriveType::BitXorAssign
        | DeriveType::Div
        | DeriveType::DivAssign
        | DeriveType::Mul
        | DeriveType::MulAssign
        | DeriveType::Rem
        | DeriveType::RemAssign
        | DeriveType::Shl
        | DeriveType::ShlAssign
        | DeriveType::Shr
        | DeriveType::ShrAssign
        | DeriveType::PartialEq
        | DeriveType::Sub
        | DeriveType::SubAssign => Err(syn::Error::new_spanned(
            path,
            alloc::format!("expected `#[newtype({derive_type}(...))]`"),
        )),
    }
}

/// Parses a single nested list attribute and fills in its structured representation:
/// `#[newtype(attr(attr1, attr2))]`
fn parse_nested_list(
    derive_type: DeriveType,
    list: &syn::MetaList,
    res: &mut NewtypeDerives,
) -> syn::Result<()> {
    match derive_type {
        DeriveType::From => parse_type_with(list, &mut res.from),
        DeriveType::TryFrom => parse_type_error_with(list, &mut res.try_from),
        DeriveType::Into => parse_type_with(list, &mut res.into),
        DeriveType::TryInto => parse_type_error_with(list, &mut res.try_into),
        DeriveType::Add => parse_type_output_with(list, &mut res.add),
        DeriveType::AddAssign => parse_type_with(list, &mut res.add_assign),
        DeriveType::BitAnd => parse_type_output_with(list, &mut res.bitand),
        DeriveType::BitAndAssign => parse_type_with(list, &mut res.bitand_assign),
        DeriveType::BitOr => parse_type_output_with(list, &mut res.bitor),
        DeriveType::BitOrAssign => parse_type_with(list, &mut res.bitor_assign),
        DeriveType::BitXor => parse_type_output_with(list, &mut res.bitxor),
        DeriveType::BitXorAssign => parse_type_with(list, &mut res.bitxor_assign),
        DeriveType::Div => parse_type_output_with(list, &mut res.div),
        DeriveType::DivAssign => parse_type_with(list, &mut res.div_assign),
        DeriveType::Mul => parse_type_output_with(list, &mut res.mul),
        DeriveType::MulAssign => parse_type_with(list, &mut res.mul_assign),
        DeriveType::Rem => parse_type_output_with(list, &mut res.rem),
        DeriveType::RemAssign => parse_type_with(list, &mut res.rem_assign),
        DeriveType::Shl => parse_type_output_with(list, &mut res.shl),
        DeriveType::ShlAssign => parse_type_with(list, &mut res.shl_assign),
        DeriveType::Shr => parse_type_output_with(list, &mut res.shr),
        DeriveType::ShrAssign => parse_type_with(list, &mut res.shr_assign),
        DeriveType::PartialEq => parse_type_with(list, &mut res.partial_eq),
        DeriveType::Sub => parse_type_output_with(list, &mut res.sub),
        DeriveType::SubAssign => parse_type_with(list, &mut res.sub_assign),
    }
}

/// Parses a single nested name-value attribute and fills in its structured representation.
/// `#[newtype(attr = value)]`
fn parse_nested_name_value(
    derive_type: DeriveType,
    name_value: &syn::MetaNameValue,
    _res: &mut NewtypeDerives,
) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        name_value,
        alloc::format!("expected `#[newtype({derive_type}(...))]`"),
    ))
}

/// Parses newtype attribute from a list:
/// `#[newtype(attribute(type, error = type, with = expr))]`
fn parse_type_error_with(
    list: &syn::MetaList,
    res_ops: &mut Vec<(syn::Type, syn::Type, syn::Expr)>,
) -> syn::Result<()> {
    list.parse_args_with(|input: syn::parse::ParseStream| {
        let rhs_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::error>()?;
        input.parse::<syn::Token![=]>()?;
        let error_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        res_ops.push((rhs_ty, error_ty, with_expr));
        Ok(())
    })
}

/// Parses newtype attribute from a list:
/// `#[newtype(attribute(type, output = type, with = expr))]`
fn parse_type_output_with(
    list: &syn::MetaList,
    res_ops: &mut Vec<(syn::Type, syn::Type, syn::Expr)>,
) -> syn::Result<()> {
    list.parse_args_with(|input: syn::parse::ParseStream| {
        let rhs_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::output>()?;
        input.parse::<syn::Token![=]>()?;
        let output_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        res_ops.push((rhs_ty, output_ty, with_expr));
        Ok(())
    })
}

/// Parses newtype attribute from a list:
/// `#[newtype(attribute(type, with = expr))]`
fn parse_type_with(
    list: &syn::MetaList,
    res_ops: &mut Vec<(syn::Type, syn::Expr)>,
) -> syn::Result<()> {
    list.parse_args_with(|input: syn::parse::ParseStream| {
        let rhs_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        res_ops.push((rhs_ty, with_expr));
        Ok(())
    })
}

/// Parses a syntax tree node of type `T` or a literal with `T` inside:
/// `with = \"|x| x.into()\"`
fn parse_lit_or<T>(input: &syn::parse::ParseStream) -> syn::Result<T>
where
    T: syn::parse::Parse,
{
    // Try to parse `LitStr` containing `T` first.
    // We fork so that if `parse` fails, we haven't moved the cursor.
    let fork = input.fork();
    if let Ok(lit_str) = fork.parse::<syn::LitStr>() {
        use syn::parse::discouraged::Speculative;
        input.advance_to(&fork);
        return lit_str.parse::<T>();
    }
    // If `LitStr` parsing failed, try to parse the `T` directly.
    input.parse::<T>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_newtype_derives() {
        let input = quote::quote! { fn not_a_struct() {} };
        let result = super::parse_newtype_derives(input);
        assert!(result.is_err());
    }
}
