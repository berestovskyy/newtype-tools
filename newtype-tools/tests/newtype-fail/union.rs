use newtype_tools::Newtype;

#[derive(Newtype)]
union U {
    u: u32,
}

fn main() {}
