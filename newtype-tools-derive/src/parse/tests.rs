#[test]
fn attr_type_display_roundtrip() {
    use super::AttrType;
    assert_eq!(format!("{}", AttrType::From), "from");
    assert_eq!(format!("{}", AttrType::TryFrom), "try_from");
    assert_eq!(format!("{}", AttrType::Into), "into");
    assert_eq!(format!("{}", AttrType::TryInto), "try_into");
    assert_eq!(format!("{}", AttrType::PartialEq), "partial_eq");
}
