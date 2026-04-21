/// `#[newtype(partial_eq(type, with = expr))]`

mod missing_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(partial_eq())]
    struct Oranges(u32);
}

mod missing_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(partial_eq(u64))]
    struct Oranges(u32);
}

mod missing_type_comma_with {
    #[derive(newtype_tools::Newtype)]
    #[newtype(partial_eq(u64,))]
    struct Oranges(u32);
}

mod missing_type_comma_with_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(partial_eq(u64, with))]
    struct Oranges(u32);
}

mod missing_type_comma_with_eq_expr {
    #[derive(newtype_tools::Newtype)]
    #[newtype(partial_eq(u64, with =))]
    struct Oranges(u32);
}

fn main() {}
