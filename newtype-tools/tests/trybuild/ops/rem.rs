/// `#[newtype(rem(type, output = type, with = expr))]`

mod missing_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem())]
    struct Oranges(u32);
}

mod missing_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64))]
    struct Oranges(u32);
}

mod missing_type_comma_output {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64,))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq_type {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output =))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq_type_comma {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output = u64))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq_type_comma_with {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output = u64,))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq_type_comma_with_eq {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output = u64, with))]
    struct Oranges(u32);
}

mod missing_type_comma_output_eq_type_comma_with_eq_expr {
    #[derive(newtype_tools::Newtype)]
    #[newtype(rem(u64, output = u64, with =))]
    struct Oranges(u32);
}

fn main() {}
