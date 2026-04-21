#![cfg(feature = "derive")]

#[test]
fn newtype_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/conversions/*.rs");
    t.compile_fail("tests/trybuild/newtype/*.rs");
    t.compile_fail("tests/trybuild/ops/*.rs");
}
