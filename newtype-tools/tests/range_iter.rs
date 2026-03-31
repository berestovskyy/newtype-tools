#![cfg(feature = "derive")]

use newtype_tools::Newtype;

#[test]
fn derive_range_iter() {
    let mut range = 1..3;
    assert_eq!(range.len(), 2);
    assert_eq!(range.next(), Some(1));
    assert_eq!(range.next(), Some(2));
    assert_eq!(range.next(), None);

    let mut range_inclusive = 2..=3;
    // The `RangeInclusive`` does not implement `len` for i32 :(
    // assert_eq!(range_inclusive.len(), 2);
    assert_eq!(range_inclusive.next(), Some(2));
    assert_eq!(range_inclusive.next(), Some(3));
    assert_eq!(range_inclusive.next(), None);

    #[derive(Debug, Newtype, PartialEq)]
    #[newtype(from(usize, with = |u| Apples(u as u64)))]
    #[newtype(range_iter(usize))]
    struct Apples(u64);
    let range = Apples(1)..Apples(3);
    let mut range_iter = Apples::range_iter(range);
    assert_eq!(range_iter.len(), 2);
    assert_eq!(range_iter.next(), Some(Apples(1)));
    assert_eq!(range_iter.next(), Some(Apples(2)));
    assert_eq!(range_iter.next(), None);

    #[derive(Debug, Newtype, PartialEq)]
    #[newtype(from(u16, with = |u| Oranges(u as u32)))]
    #[newtype(range_iter(u16))]
    struct Oranges(u32);
    let range = Oranges(2)..=Oranges(3);
    let mut range_iter = Oranges::range_iter(range);
    assert_eq!(range_iter.len(), 2);
    assert_eq!(range_iter.next(), Some(Oranges(2)));
    assert_eq!(range_iter.next(), Some(Oranges(3)));
    assert_eq!(range_iter.next(), None);
}
