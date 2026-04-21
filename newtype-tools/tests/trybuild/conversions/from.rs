/// `#[newtype(from(type, with = expr))]`

mod missing_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from())]
    struct Oranges(u32);
}

mod missing_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from(u64))]
    struct Oranges(u32);
}

mod missing_type_comma_with {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from(u64,))]
    struct Oranges(u32);
}

mod missing_type_comma_with_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from(u64, with))]
    struct Oranges(u32);
}

mod missing_type_comma_with_eq_expr {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from(u64, with =))]
    struct Oranges(u32);
}

fn main() {}
