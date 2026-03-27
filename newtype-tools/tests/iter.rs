#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn derive_iter() {
    #[derive(Debug, Newtype)]
    #[newtype(iter(u64))]
    struct Apples(u64);

    for apple in Apples::iter(Apples(0)..Apples(42)) {
        println!("{apple:?}");
    }
}
