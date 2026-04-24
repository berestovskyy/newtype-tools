/// `#[newtype(Amount)]`

mod ok {
    #[newtype_tools::newtype(Id)]
    struct Apples(u64);
    #[newtype_tools::newtype(Amount)]
    struct Oranges(u32);
}

mod missing_type {
    #[newtype_tools::newtype]
    struct Oranges(u32);
}

mod missing_kind {
    #[newtype_tools::newtype()]
    struct Oranges(u32);
}

mod invalid_kind {
    #[newtype_tools::newtype(InvalidKind)]
    struct Oranges(u32);
}

mod invalid_newtype {
    #[newtype_tools::newtype(Amount)]
    struct Oranges();
}

mod invalid_derive {
    #[newtype_tools::newtype(Amount)]
    fn not_a_struct() {}
}

mod extra_comma_kind {
    #[newtype_tools::newtype(Amount, Amount)]
    struct Oranges(u32);
}

fn main() {}
