use crate::flatten::{flatten, *};

#[test]
fn expect_primatives() {
    const TEST_DATA: &'static str = r#"
		small = u8
		u8 = uint .size 1
		i8 = int .size 1
		u16 = uint .size 2
		i16 = int .size 2
		u32 = uint .size 4
		i32 = int .size 4
		u64 = uint .size 8
		i64 = int .size 8
		is-dhcp = bool
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["u8"], Node::ConstrainedType(ConstrainedType::U8),);
    assert_eq!(nodes["u16"], Node::ConstrainedType(ConstrainedType::U16),);
    assert_eq!(nodes["u32"], Node::ConstrainedType(ConstrainedType::U32),);
    assert_eq!(nodes["u64"], Node::ConstrainedType(ConstrainedType::U64),);
    assert_eq!(nodes["i8"], Node::ConstrainedType(ConstrainedType::I8),);
    assert_eq!(nodes["i16"], Node::ConstrainedType(ConstrainedType::I16),);
    assert_eq!(nodes["i32"], Node::ConstrainedType(ConstrainedType::I32),);
    assert_eq!(nodes["i64"], Node::ConstrainedType(ConstrainedType::I64),);
}
