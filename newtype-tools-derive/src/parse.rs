use crate::ParseResult;

#[cfg(test)]
mod tests;

mod kw {
    syn::custom_keyword!(error);
    syn::custom_keyword!(output);
    syn::custom_keyword!(with);
}

/// Newtype attribute name.
const NEWTYPE_ATTR_NAME: &str = "newtype";

/// Parses all attributes and produce their structured representation.
pub(crate) fn parse_input(input: syn::DeriveInput) -> syn::Result<ParseResult> {
    let inner_ty = parse_derive_input_data(input.data)?;
    let mut res = ParseResult::new(input.ident, inner_ty, input.generics);
    for attr in input.attrs {
        // Just skip all other top-level attributes.
        if !attr.path().is_ident(NEWTYPE_ATTR_NAME) {
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
        | AttrType::BitAnd
        | AttrType::BitAndAssign
        | AttrType::BitOr
        | AttrType::BitOrAssign
        | AttrType::BitXor
        | AttrType::BitXorAssign
        | AttrType::Div
        | AttrType::DivAssign
        | AttrType::Mul
        | AttrType::MulAssign
        | AttrType::Rem
        | AttrType::RemAssign
        | AttrType::Shl
        | AttrType::ShlAssign
        | AttrType::Shr
        | AttrType::ShrAssign
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
        AttrType::From => parse_type_with(list, &mut res.from),
        AttrType::TryFrom => parse_type_error_with(list, &mut res.try_from),
        AttrType::Into => parse_type_with(list, &mut res.into),
        AttrType::TryInto => parse_type_error_with(list, &mut res.try_into),
        AttrType::Add => parse_type_output_with(list, &mut res.add),
        AttrType::AddAssign => parse_type_with(list, &mut res.add_assign),
        AttrType::BitAnd => parse_type_output_with(list, &mut res.bitand),
        AttrType::BitAndAssign => parse_type_with(list, &mut res.bitand_assign),
        AttrType::BitOr => parse_type_output_with(list, &mut res.bitor),
        AttrType::BitOrAssign => parse_type_with(list, &mut res.bitor_assign),
        AttrType::BitXor => parse_type_output_with(list, &mut res.bitxor),
        AttrType::BitXorAssign => parse_type_with(list, &mut res.bitxor_assign),
        AttrType::Div => parse_type_output_with(list, &mut res.div),
        AttrType::DivAssign => parse_type_with(list, &mut res.div_assign),
        AttrType::Mul => parse_type_output_with(list, &mut res.mul),
        AttrType::MulAssign => parse_type_with(list, &mut res.mul_assign),
        AttrType::Rem => parse_type_output_with(list, &mut res.rem),
        AttrType::RemAssign => parse_type_with(list, &mut res.rem_assign),
        AttrType::Shl => parse_type_output_with(list, &mut res.shl),
        AttrType::ShlAssign => parse_type_with(list, &mut res.shl_assign),
        AttrType::Shr => parse_type_output_with(list, &mut res.shr),
        AttrType::ShrAssign => parse_type_with(list, &mut res.shr_assign),
        AttrType::PartialEq => parse_type_with(list, &mut res.partial_eq),
        AttrType::Sub => parse_type_output_with(list, &mut res.sub),
        AttrType::SubAssign => parse_type_with(list, &mut res.sub_assign),
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

/// Attribute types.
#[derive(Debug, PartialEq)]
enum AttrType {
    From,
    TryFrom,
    Into,
    TryInto,
    Add,
    AddAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
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
            Self::BitAnd => f.write_str("bitand"),
            Self::BitAndAssign => f.write_str("bitand_assign"),
            Self::BitOr => f.write_str("bitor"),
            Self::BitOrAssign => f.write_str("bitor_assign"),
            Self::BitXor => f.write_str("bitxor"),
            Self::BitXorAssign => f.write_str("bitxor_assign"),
            Self::Div => f.write_str("div"),
            Self::DivAssign => f.write_str("div_assign"),
            Self::Mul => f.write_str("mul"),
            Self::MulAssign => f.write_str("mul_assign"),
            Self::Rem => f.write_str("rem"),
            Self::RemAssign => f.write_str("rem_assign"),
            Self::Shl => f.write_str("shl"),
            Self::ShlAssign => f.write_str("shl_assign"),
            Self::Shr => f.write_str("shr"),
            Self::ShrAssign => f.write_str("shr_assign"),
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
            Some(i) if i == "bitand" => Ok(Self::BitAnd),
            Some(i) if i == "bitand_assign" => Ok(Self::BitAndAssign),
            Some(i) if i == "bitor" => Ok(Self::BitOr),
            Some(i) if i == "bitor_assign" => Ok(Self::BitOrAssign),
            Some(i) if i == "bitxor" => Ok(Self::BitXor),
            Some(i) if i == "bitxor_assign" => Ok(Self::BitXorAssign),
            Some(i) if i == "div" => Ok(Self::Div),
            Some(i) if i == "div_assign" => Ok(Self::DivAssign),
            Some(i) if i == "mul" => Ok(Self::Mul),
            Some(i) if i == "mul_assign" => Ok(Self::MulAssign),
            Some(i) if i == "rem" => Ok(Self::Rem),
            Some(i) if i == "rem_assign" => Ok(Self::RemAssign),
            Some(i) if i == "shl" => Ok(Self::Shl),
            Some(i) if i == "shl_assign" => Ok(Self::ShlAssign),
            Some(i) if i == "shr" => Ok(Self::Shr),
            Some(i) if i == "shr_assign" => Ok(Self::ShrAssign),
            Some(i) if i == "partial_eq" => Ok(Self::PartialEq),
            Some(i) if i == "sub" => Ok(Self::Sub),
            Some(i) if i == "sub_assign" => Ok(Self::SubAssign),
            _ => Err(syn::Error::new_spanned(
                value,
                "expected `(try_)from`, `(try_)into`, `add(_assign)`, `bitand(_assign)`, \
                `bitor(_assign)`, `bitxor(_assign)`, `mul(_assign)`, `partial_eq`, `sub(_assign)`, \
                `iter`",
            )),
        }
    }
}
