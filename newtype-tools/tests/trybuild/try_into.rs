/// `#[newtype(try_into(type, error = type, with = expr))]`

mod missing_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into())]
    struct Oranges(u32);
}

mod missing_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64))]
    struct Oranges(u32);
}

mod missing_type_comma_error {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64,))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error =))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error = Error))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq_type_comma_with {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error = Error,))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq_type_comma_with_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error = Error, with))]
    struct Oranges(u32);
}

mod missing_type_comma_error_eq_type_comma_with_eq_expr {
    #[derive(newtype_tools::Newtype)]
    #[newtype(try_into(u64, error = Error, with =))]
    struct Oranges(u32);
}

fn main() {}
