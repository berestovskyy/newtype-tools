use crate::ParseResult;

mod kw {
    syn::custom_keyword!(error);
    syn::custom_keyword!(with);
}

const NEWTYPE_NAME: &str = "newtype";

/// Parses all attributes and produce their structured representation.
pub(crate) fn parse_input(input: syn::DeriveInput) -> syn::Result<ParseResult> {
    let mut res = ParseResult::new(input.ident);
    for attr in input.attrs {
        // Just skip all other top-level attributes.
        if !attr.path().is_ident(NEWTYPE_NAME) {
            continue;
        }
        parse_top_level_meta(attr.meta, &mut res)?;
    }
    Ok(res)
}

/// Parses a single top-level attribute's meta and fills in its structured representation.
fn parse_top_level_meta(meta: syn::Meta, res: &mut ParseResult) -> syn::Result<()> {
    match meta {
        // `#[newtype]`
        syn::Meta::Path(path) => parse_top_level_path(path, res)?,
        // `#[newtype(item1, item2)]`
        syn::Meta::List(list) => parse_top_level_list(list, res)?,
        // `#[newtype = value]`
        syn::Meta::NameValue(name_value) => parse_top_level_name_value(name_value, res)?,
    }
    Ok(())
}

/// Parses a single top-level path attribute and fills in its structured representation:
/// `#[newtype]`
fn parse_top_level_path(_path: syn::Path, _res: &mut ParseResult) -> syn::Result<()> {
    Ok(())
}

/// Parses a single top-level list attribute and fills in its structured representation:
/// `#[newtype(item1, item2)]`
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
    _name_value: syn::MetaNameValue,
    _res: &mut ParseResult,
) -> syn::Result<()> {
    Ok(())
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
        | AttrType::PartialEq
        | AttrType::RangeIter => Err(syn::Error::new_spanned(
            path,
            format!("expected `{attr_type}(...)`"),
        )),
    }
}

/// Parses a single nested list attribute and fills in its structured representation:
/// `#[newtype(attr(item1, item2))]`
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
        AttrType::PartialEq => parse_partial_eq(list, res),
        AttrType::RangeIter => parse_iter(list, res),
    }
}

/// Parses a single nested name-value attribute and fills in its structured representation.
/// `#[newtype(attr = value)]`
fn parse_nested_name_value(
    _attr_type: AttrType,
    _name_value: &syn::MetaNameValue,
    _res: &mut ParseResult,
) -> syn::Result<()> {
    Ok(())
}

/// Parses newtype `from` attribute from a list:
/// `#[newtype(from(type, with = expr))]`
fn parse_from(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (input_ty, with_expr) = list.parse_args_with(|input: syn::parse::ParseStream| {
        let input_ty = parse_lit_or::<syn::Type>(&input)?;
        input.parse::<syn::Token![,]>()?;
        input.parse::<kw::with>()?;
        input.parse::<syn::Token![=]>()?;
        let with_expr = parse_lit_or::<syn::Expr>(&input)?;
        Ok((input_ty, with_expr))
    })?;
    res.from.push((input_ty, with_expr));
    Ok(())
}

/// Parses newtype `try_from` attribute from a list:
/// `#[newtype(try_from(type, error = type, with = expr))]`
fn parse_try_from(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let (input_ty, error_ty, with_expr) =
        list.parse_args_with(|input: syn::parse::ParseStream| {
            let input_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::error>()?;
            input.parse::<syn::Token![=]>()?;
            let error_ty = parse_lit_or::<syn::Type>(&input)?;
            input.parse::<syn::Token![,]>()?;
            input.parse::<kw::with>()?;
            input.parse::<syn::Token![=]>()?;
            let with_expr = parse_lit_or::<syn::Expr>(&input)?;
            Ok((input_ty, error_ty, with_expr))
        })?;
    res.try_from.push((input_ty, error_ty, with_expr));
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

/// Parses newtype `range_iter` attribute from a list:
/// `#[newtype(range_iter(type))]`
fn parse_iter(list: &syn::MetaList, res: &mut ParseResult) -> syn::Result<()> {
    let inner_ty = list.parse_args_with(|input: syn::parse::ParseStream| {
        let inner_ty = parse_lit_or::<syn::Type>(&input)?;
        Ok(inner_ty)
    })?;
    res.range_iter = Some(inner_ty);
    Ok(())
}

/// Attribute types.
#[derive(Debug, PartialEq)]
enum AttrType {
    From,
    TryFrom,
    Into,
    TryInto,
    PartialEq,
    RangeIter,
}

impl std::fmt::Display for AttrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::From => f.write_str("from"),
            Self::TryFrom => f.write_str("try_from"),
            Self::Into => f.write_str("into"),
            Self::TryInto => f.write_str("try_into"),
            Self::PartialEq => f.write_str("partial_eq"),
            Self::RangeIter => f.write_str("range_iter"),
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
            Some(i) if i == "partial_eq" => Ok(Self::PartialEq),
            Some(i) if i == "range_iter" => Ok(Self::RangeIter),
            _ => Err(syn::Error::new_spanned(
                value,
                "Error matching attribute: expected one of \
                    `newtype`, `(try_)from`, `(try_)into`, `partial_eq`, `range_iter`",
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
