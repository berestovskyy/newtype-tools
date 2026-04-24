#![cfg(feature = "derive")]

// There is a small discrepancy between the stable and nightly in span reporting.
// ```ignore
// error: expected `#[newtype(NewtypeKind)]`
//   --> tests/trybuild/newtype_attribute.rs:34:30
//    |
// 34 |     #[newtype_tools::newtype(Amount, Amount)]
//    |                              ^^^^^^
// ```
#[rustversion::stable]
#[test]
fn newtype_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/*.rs");
    t.compile_fail("tests/trybuild/conversions/*.rs");
    t.compile_fail("tests/trybuild/ops/*.rs");
}
