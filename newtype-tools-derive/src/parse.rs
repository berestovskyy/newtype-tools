use crate::ParseResult;

#[cfg(test)]
mod tests;

mod kw {
    syn::custom_keyword!(error);
    syn::custom_keyword!(output);
    syn::custom_keyword!(with);
}

const NEWTYPE_NAME: &str = "newtype";

/// Parses all attributes and produce their structured representation.
pub(crate) fn parse_input(input: syn::DeriveInput) -> syn::Result<ParseResult> {
    let inner_ty = parse_derive_input_data(input.data)?;
    let mut res = ParseResult::new(input.ident, inner_ty, input.generics);
    for attr in input.attrs {
        // Just skip all other top-level attributes.
        if !attr.path().is_ident(NEWTYPE_NAME) {
            continue;
        }
        parse_top_level_meta(attr.meta, &mut res)?;
    }
    Ok(res)
}

/// Parses the first struct field to get the inner type.
///
/// For `struct Newtype(type)` returns `type`.
fn parse_derive_input_data(data: syn::Data) -> syn::Result<syn::Type> {
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
fn parse_top_level_meta(meta: syn::Meta, res: &mut ParseResult) -> syn::Result<()> {
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
fn parse_top_level_path(path: syn::Path, _res: &mut ParseResult) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        path,
        "expected `#[newtype(attr1, attr2)]`",
    ))
}

/// Parses a single top-level list attribute and fills in its structured representation:
/// `#[newtype(attr1, attr2)]`
fn parse_top_level_list(list: syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
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
    _res: &mut ParseResult,
) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        name_value,
        "expected `#[newtype(attr1, attr2)]`",
    ))
}

/// Parses a single nested attribute's meta and fills in its structured representation.
fn parse_nested_meta(meta: &syn::Meta, res: &mut ParseResult) -> syn::Result<()> {
    let attr_type = AttrType::try_from(meta.path().get_ident())?;
    match meta {
        // `#[newtype(attr)]`
        syn::Meta::Path(path) => parse_nested_path(attr_type, path, res)?,
        // `#[newtype(attr1, attr2)]`
        syn::Meta::List(list) => parse_nested_list(attr_type, list, res)?,
        // `#[newtype(attr = value)]`
        syn::Meta::NameValue(name_value) => parse_nested_name_value(attr_type, name_value, res)?,
    }
    Ok(())
}

/// Parses a single nested path attribute and fills in its structured representation:
/// `#[newtype(attr)]`
fn parse_nested_path(
    attr_type: AttrType,
    path: &syn::Path,
    _res: &mut ParseResult,
) -> syn::Result<()> {
    match attr_type {
        AttrType::From
        | AttrType::TryFrom
        | AttrType::Into
        | AttrType::TryInto
        | AttrType::Add
        | AttrType::AddAssign
        | AttrType::PartialEq
        | AttrType::Sub
        | AttrType::SubAssign => Err(syn::Error::new_spanned(
            path,
            format!("expected `#[newtype({attr_type}(...))]`"),
        )),
    }
}

/// Parses a single nested list attribute and fills in its structured representation:
/// `#[newtype(attr(attr1, attr2))]`
fn parse_nested_list(
    attr_type: AttrType,
    list: &syn::MetaList,
    res: &mut ParseResult,
) -> syn::Result<()> {
    match attr_type {
        AttrType::From => parse_from(list, res),
        AttrType::TryFrom => parse_try_from(list, res),
        AttrType::Into => parse_into(list, res),
        AttrType::TryInto => parse_try_into(list, res),
        AttrType::Add => parse_add(list, res),
        AttrType::AddAssign => parse_add_assign(list, res),
        AttrType::PartialEq => parse_partial_eq(list, res),
        AttrType::Sub => parse_sub(list, res),
        AttrType::SubAssign => parse_sub_assign(list, res),
    }
}

/// Parses a single nested name-value attribute and fills in its structured representation.
/// `#[newtype(attr = value)]`
fn parse_nested_name_value(
    attr_type: AttrType,
    name_value: &syn::MetaNameValue,
    _res: &mut ParseResult,
) -> syn::Result<()> {
    Err(syn::Error::new_spanned(
        name_value,
        format!("expected `#[newtype({attr_type}(...))]`"),
    ))
}

/// Parses newtype `from` attribute from a list:
/// `#[newtype(from(type, with = expr))]`
fn parse_from(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (from_ty, with_expr) = list.parse_args_with(|input: syn::parse::ParseStream| {
        let from_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        Ok((from_ty, with_expr))
    })?;
    res.from.push((from_ty, with_expr));
    Ok(())
}

/// Parses newtype `try_from` attribute from a list:
/// `#[newtype(try_from(type, error = type, with = expr))]`
fn parse_try_from(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (try_from_ty, error_ty, with_expr) =
        list.parse_args_with(|input: syn::parse::ParseStream| {
            let try_from_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::error>()?;
            input.parse::<syn::Token![=]>()?;
            let error_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::with>()?;
            input.parse::<syn::Token![=]>()?;
            let with_expr = parse_lit_or::<syn::Expr>(&input)?;
            Ok((try_from_ty, error_ty, with_expr))
        })?;
    res.try_from.push((try_from_ty, error_ty, with_expr));
    Ok(())
}

/// Parses newtype `into` attribute from a list:
/// `#[newtype(into(type, with = expr))]`
fn parse_into(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (output_ty, with_expr) = list.parse_args_with(|input: syn::parse::ParseStream| {
        let output_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        Ok((output_ty, with_expr))
    })?;
    res.into.push((output_ty, with_expr));
    Ok(())
}

/// Parses newtype `try_into` attribute from a list:
/// `#[newtype(try_into(type, error = type, with = expr))]`
fn parse_try_into(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (output_ty, error_ty, with_expr) =
        list.parse_args_with(|input: syn::parse::ParseStream| {
            let output_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::error>()?;
            input.parse::<syn::Token![=]>()?;
            let error_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::with>()?;
            input.parse::<syn::Token![=]>()?;
            let with_expr = parse_lit_or::<syn::Expr>(&input)?;
            Ok((output_ty, error_ty, with_expr))
        })?;
    res.try_into.push((output_ty, error_ty, with_expr));
    Ok(())
}

/// Parses newtype `add` attribute from a list:
/// `#[newtype(add(type, output = type, with = expr))]`
fn parse_add(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    res.add.push(parse_type_output_with(list)?);
    Ok(())
}

/// Parses newtype `add_assign` attribute from a list:
/// `#[newtype(add_assign(type, with = expr))]`
fn parse_add_assign(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    res.add_assign.push(parse_type_with(list)?);
    Ok(())
}

/// Parses newtype `partial_eq` attribute from a list:
/// `#[newtype(partial_eq(type, with = expr))]`
fn parse_partial_eq(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (other_ty, with_expr) = list.parse_args_with(|input: syn::parse::ParseStream| {
        let other_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        Ok((other_ty, with_expr))
    })?;
    res.partial_eq.push((other_ty, with_expr));
    Ok(())
}

/// Parses newtype `sub` attribute from a list:
/// `#[newtype(sub(type, output = type, with = expr))]`
fn parse_sub(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    res.sub.push(parse_type_output_with(list)?);
    Ok(())
}

/// Parses newtype `sub` attribute from a list:
/// `#[newtype(sub_assign(type, with = expr))]`
fn parse_sub_assign(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    res.sub_assign.push(parse_type_with(list)?);
    Ok(())
}

/// Parses newtype attribute from a list:
/// `#[newtype(attribute(type, output = type, with = expr))]`
fn parse_type_output_with(list: &syn::MetaList) -> syn::Result<(syn::Type, syn::Type, syn::Expr)> {
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
        Ok((rhs_ty, output_ty, with_expr))
    })
}

/// Parses newtype attribute from a list:
/// `#[newtype(attribute(type, with = expr))]`
fn parse_type_with(list: &syn::MetaList) -> syn::Result<(syn::Type, syn::Expr)> {
    list.parse_args_with(|input: syn::parse::ParseStream| {
        let rhs_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        Ok((rhs_ty, with_expr))
    })
}

/// Attribute types.
#[derive(Debug, PartialEq)]
enum AttrType {
    From,
    TryFrom,
    Into,
    TryInto,
    Add,
    AddAssign,
    PartialEq,
    Sub,
    SubAssign,
}

impl std::fmt::Display for AttrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::From => f.write_str("from"),
            Self::TryFrom => f.write_str("try_from"),
            Self::Into => f.write_str("into"),
            Self::TryInto => f.write_str("try_into"),
            Self::Add => f.write_str("add"),
            Self::AddAssign => f.write_str("add_assign"),
            Self::PartialEq => f.write_str("partial_eq"),
            Self::Sub => f.write_str("sub"),
            Self::SubAssign => f.write_str("sub_assign"),
        }
    }
}

impl TryFrom<Option<&syn::Ident>> for AttrType {
    type Error = syn::Error;

    fn try_from(value: Option<&syn::Ident>) -> Result<Self, Self::Error> {
        match value {
            Some(i) if i == "from" => Ok(Self::From),
            Some(i) if i == "try_from" => Ok(Self::TryFrom),
            Some(i) if i == "into" => Ok(Self::Into),
            Some(i) if i == "try_into" => Ok(Self::TryInto),
            Some(i) if i == "add" => Ok(Self::Add),
            Some(i) if i == "add_assign" => Ok(Self::AddAssign),
            Some(i) if i == "partial_eq" => Ok(Self::PartialEq),
            Some(i) if i == "sub" => Ok(Self::Sub),
            Some(i) if i == "sub_assign" => Ok(Self::SubAssign),
            _ => Err(syn::Error::new_spanned(
                value,
                "expected `(try_)from`, `(try_)into`, `add(_assign)`, `partial_eq`, \
                `sub(_assign)`, `iter`",
            )),
        }
    }
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
