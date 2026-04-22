#[test]
fn attr_type_display_roundtrip() {
    use super::AttrType;
    assert_eq!(format!("{}", AttrType::From), "from");
    assert_eq!(format!("{}", AttrType::TryFrom), "try_from");
    assert_eq!(format!("{}", AttrType::Into), "into");
    assert_eq!(format!("{}", AttrType::TryInto), "try_into");
    assert_eq!(format!("{}", AttrType::Add), "add");
    assert_eq!(format!("{}", AttrType::AddAssign), "add_assign");
    assert_eq!(format!("{}", AttrType::BitAnd), "bitand");
    assert_eq!(format!("{}", AttrType::BitAndAssign), "bitand_assign");
    assert_eq!(format!("{}", AttrType::BitOr), "bitor");
    assert_eq!(format!("{}", AttrType::BitOrAssign), "bitor_assign");
    assert_eq!(format!("{}", AttrType::BitXor), "bitxor");
    assert_eq!(format!("{}", AttrType::BitXorAssign), "bitxor_assign");
    assert_eq!(format!("{}", AttrType::Div), "div");
    assert_eq!(format!("{}", AttrType::DivAssign), "div_assign");
    assert_eq!(format!("{}", AttrType::Mul), "mul");
    assert_eq!(format!("{}", AttrType::MulAssign), "mul_assign");
    assert_eq!(format!("{}", AttrType::PartialEq), "partial_eq");
    assert_eq!(format!("{}", AttrType::Sub), "sub");
    assert_eq!(format!("{}", AttrType::SubAssign), "sub_assign");
}
