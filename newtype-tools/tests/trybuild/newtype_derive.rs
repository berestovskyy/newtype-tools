/// `#[derive(Newtype)]`

mod enum_invalid {
    #[derive(newtype_tools::Newtype)]
    enum Enum {}
}

mod named_struct {
    #[derive(newtype_tools::Newtype)]
    struct Oranges {
        inner: u32,
    }
}

mod struct_list {
    #[derive(newtype_tools::Newtype)]
    #[newtype(;)]
    struct Oranges(u32);
}

mod struct_list_name_value {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from = Oranges)]
    struct Oranges(u32);
}

mod struct_list_path {
    #[derive(newtype_tools::Newtype)]
    #[newtype(from)]
    struct Oranges(u32);
}

mod struct_list_path_invalid {
    #[derive(newtype_tools::Newtype)]
    #[newtype(invalid_path)]
    struct Oranges(u32);
}

mod struct_name_value {
    #[derive(newtype_tools::Newtype)]
    #[newtype = value]
    struct Oranges(u32);
}

mod struct_path {
    #[derive(newtype_tools::Newtype)]
    #[newtype]
    struct Oranges(u32);
}

mod tuple_struct {
    #[derive(newtype_tools::Newtype)]
    struct N(u64, u64);
}

mod union {
    #[derive(newtype_tools::Newtype)]
    union U {
        u: u32,
    }
}

mod unit_struct {
    #[derive(newtype_tools::Newtype)]
    struct Oranges;
}

fn main() {}
