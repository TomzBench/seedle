use crate::flatten::{flatten, *};

#[test]
fn expect_primatives() {
    const TEST_DATA: &'static str = r#"
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
    assert_eq!(
        nodes["is-dhcp"],
        Node::ConstrainedType(ConstrainedType::Bool)
    );
}

#[test]
fn expect_map() {
    const TEST_DATA: &'static str = r#"
		colors = {
			red: u8,
			boat: u16
		}
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["colors"],
        Node::Map(Group {
            members: vec![
                KeyVal::new("red", Node::Foreign("u8".into())).into(),
                KeyVal::new("boat", Node::Foreign("u16".into())).into(),
            ]
        })
    );
}

#[test]
fn expect_group() {
    const TEST_DATA: &'static str = r#"
		ip = (
	        address: tstr .size 16,
	        port: u16,
	        dhcp: is-dhcp,
		)
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["ip"],
        Node::Group(Group {
            members: vec![
                KeyVal::new("address", ConstrainedType::Str(16).into()).into(),
                KeyVal::new("port", Node::Foreign("u16".into())).into(),
                KeyVal::new("dhcp", Node::Foreign("is-dhcp".into())).into(),
            ]
        })
    );
}

#[test]
fn expect_map_w_foreign_member() {
    const TEST_DATA: &'static str = r#"
		data = {
	        thing: foo
		}
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["data"],
        Node::Map(Group {
            members: vec![KeyVal::new("thing", Node::Foreign("foo".into()).into()).into()]
        })
    );
}

#[test]
fn expect_map_w_foreign_group_member() {
    const TEST_DATA: &'static str = r#"
		data = {
	        foo
		}
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["data"],
        Node::Map(Group {
            members: vec![Node::Foreign("foo".into()).into()]
        })
    );
}

#[test]
fn expect_array_w_foreign_element() {
    const TEST_DATA: &'static str = r#"
        data = [ 6*6 foo ]
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["data"],
        Node::Array(Array {
            ty: Box::new(Node::Foreign("foo".into())),
            len: 6
        })
    );
}

#[test]
fn expect_top_level_foreign() {
    const TEST_DATA: &'static str = r#"
        data = foo
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["data"], Node::Foreign("foo".into()));
}

#[test]
fn expect_top_level_literal() {
    // TODO does not support literal byte array?
    const TEST_DATA: &'static str = r#"
        u = 3
        i = -3
        s = "bar"
        b = false
        c = "A"
        f = foreign
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["u"], Node::Literal(Literal::UInt(3)));
    assert_eq!(nodes["i"], Node::Literal(Literal::Int(-3)));
    assert_eq!(nodes["s"], Node::Literal(Literal::Str("bar".into())));
    assert_eq!(nodes["b"], Node::Literal(Literal::Bool(false)));
    assert_eq!(nodes["c"], Node::Literal(Literal::Char('A')));
    assert_eq!(nodes["f"], Node::Foreign("foreign".into()));
}

#[test]
fn expect_nested_maps() {
    const TEST_DATA: &'static str = r#"
        data = {
          foo: {
            a: u8,
            b: tstr .size 32,
          }
        }
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["data"],
        Node::Map(Group {
            members: vec![KeyVal::new(
                "foo",
                Node::Map(Group {
                    members: vec![
                        KeyVal::new("a", Node::Foreign("u8".into())).into(),
                        KeyVal::new("b", Node::ConstrainedType(ConstrainedType::Str(32))).into(),
                    ],
                }),
            )
            .into()],
        })
    );
}
