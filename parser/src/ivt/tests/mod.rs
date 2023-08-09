use super::{flatten::flatten, link::link, *};

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
        u8_0 = u8
        u8_1 = u8_0
        u8_2 = u8_1
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["u8"], Node::Primative(ConstrainedPrimative::U8),);
    assert_eq!(nodes["u16"], Node::Primative(ConstrainedPrimative::U16),);
    assert_eq!(nodes["u32"], Node::Primative(ConstrainedPrimative::U32),);
    assert_eq!(nodes["u64"], Node::Primative(ConstrainedPrimative::U64),);
    assert_eq!(nodes["i8"], Node::Primative(ConstrainedPrimative::I8),);
    assert_eq!(nodes["i16"], Node::Primative(ConstrainedPrimative::I16),);
    assert_eq!(nodes["i32"], Node::Primative(ConstrainedPrimative::I32),);
    assert_eq!(nodes["i64"], Node::Primative(ConstrainedPrimative::I64),);
    assert_eq!(
        nodes["is-dhcp"],
        Node::Primative(ConstrainedPrimative::Bool)
    );

    let linked = link(nodes).unwrap();
    assert_eq!(
        linked["u8_0"],
        LinkedNode::Primative(ConstrainedPrimative::U8)
    );
    assert_eq!(
        linked["u8_1"],
        LinkedNode::Primative(ConstrainedPrimative::U8)
    );
    assert_eq!(
        linked["u8_2"],
        LinkedNode::Primative(ConstrainedPrimative::U8)
    );
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
        d = 3
        f = d
        ff = f
        fff = ff
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["u"], Node::Literal(Literal::UInt(3)));
    assert_eq!(nodes["i"], Node::Literal(Literal::Int(-3)));
    assert_eq!(nodes["s"], Node::Literal(Literal::Str("bar".into())));
    assert_eq!(nodes["b"], Node::Literal(Literal::Bool(false)));
    assert_eq!(nodes["c"], Node::Literal(Literal::Char('A')));
    assert_eq!(nodes["f"], Node::Foreign("d".into()));

    let linked = link(nodes).unwrap();
    assert_eq!(linked["f"], LinkedNode::Literal(Literal::UInt(3)));
    assert_eq!(linked["ff"], LinkedNode::Literal(Literal::UInt(3)));
    assert_eq!(linked["fff"], LinkedNode::Literal(Literal::UInt(3)));
}

#[test]
fn expect_top_level_foreign() {
    const TEST_DATA: &'static str = r#"
        foo = {
            a: uint .size 2
        }
        data = foo
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(nodes["data"], Node::Foreign("foo".into()));
    let linked = link(nodes).unwrap();
    assert_eq!(linked["data"], LinkedNode::ForeignStruct("foo".into()));
}

#[test]
fn expect_map() {
    const TEST_DATA: &'static str = r#"
        u8 = uint .size 1;
        u16 = uint .size 2;
        u16_0 = u16;
        u16_1 = u16_0;
		colors = {
            green: uint .size 4,
			red: u8,
			boat: u16_1
		}
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["colors"],
        Node::Map(Group {
            members: vec![
                KeyVal::new("green", ConstrainedPrimative::U32.into()).into(),
                KeyVal::new("red", Node::Foreign("u8".into())).into(),
                KeyVal::new("boat", Node::Foreign("u16_1".into())).into(),
            ]
        })
    );
    let linked = link(nodes).unwrap();
    assert_eq!(
        linked["colors"],
        LinkedNode::Struct(Fields {
            members: vec![
                LinkedKeyVal::new("green", ConstrainedPrimative::U32.into()).into(),
                LinkedKeyVal::new("red", ConstrainedPrimative::U8.into()).into(),
                LinkedKeyVal::new("boat", ConstrainedPrimative::U16.into()).into(),
            ]
        })
    );
}

#[test]
fn expect_group() {
    const TEST_DATA: &'static str = r#"
        is-dhcp = bool
        u16 = uint .size 2
		ip = (
	        address: tstr .size 16,
	        port: u16,
	        dhcp: is-dhcp,
		)
        ip_0 = ip
        ip_1 = ip_0
        ip_2 = ip_1
        data = {
          ip
        }
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["ip"],
        Node::Group(Group {
            members: vec![
                KeyVal::new("address", ConstrainedPrimative::Str(16).into()).into(),
                KeyVal::new("port", Node::Foreign("u16".into())).into(),
                KeyVal::new("dhcp", Node::Foreign("is-dhcp".into())).into(),
            ]
        })
    );
    assert_eq!(
        nodes["data"],
        Node::Map(Group {
            members: vec![Node::Foreign("ip".into()).into()]
        })
    );
    let linked = link(nodes).unwrap();
    assert_eq!(
        linked["data"],
        LinkedNode::Struct(Fields {
            members: vec![
                LinkedKeyVal::new("address", ConstrainedPrimative::Str(16).into()).into(),
                LinkedKeyVal::new("port", ConstrainedPrimative::U16.into()).into(),
                LinkedKeyVal::new("dhcp", ConstrainedPrimative::Bool.into()).into(),
            ]
        })
    );
}

#[test]
fn expect_array() {
    const TEST_DATA: &'static str = r#"
        xs = [ 6*6 uint .size 1 ]
        sm = [ 6*6 uint .size 2 ]
        lg = [ 6*6 uint .size 4 ]
        xl = [ 6*6 uint .size 8 ]
        bl = [ 6*6 bool ]
        ss = [ 6*6 tstr .size 4 ]
        bb = [ 6*6 bstr .size 4 ]
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["xs"],
        Array::new(ConstrainedPrimative::U8.into(), 6).into()
    );
    assert_eq!(
        nodes["sm"],
        Array::new(ConstrainedPrimative::U16.into(), 6).into()
    );
    assert_eq!(
        nodes["lg"],
        Array::new(ConstrainedPrimative::U32.into(), 6).into()
    );
    assert_eq!(
        nodes["xl"],
        Array::new(ConstrainedPrimative::U64.into(), 6).into()
    );
    assert_eq!(
        nodes["bl"],
        Array::new(ConstrainedPrimative::Bool.into(), 6).into()
    );
    assert_eq!(
        nodes["ss"],
        Array::new(ConstrainedPrimative::Str(4).into(), 6).into()
    );
    assert_eq!(
        nodes["bb"],
        Array::new(ConstrainedPrimative::Bytes(4).into(), 6).into()
    );
}

#[test]
fn expect_array_w_foreign_element() {
    const TEST_DATA: &'static str = r#"
        foo = tstr .size 4
        foo_0 = foo
        foo_1 = foo_0
        foo_2 = foo_1
        data = [ 6*6 foo_2 ]
		"#;
    let nodes = flatten(TEST_DATA).unwrap();
    assert_eq!(
        nodes["data"],
        Array::new(Node::Foreign("foo_2".into()), 6).into()
    );
    let linked = link(nodes).unwrap();
    assert_eq!(
        linked["data"],
        LinkedArray::new(ConstrainedPrimative::Str(4).into(), 6).into()
    );
}

#[test]
fn expect_nested_maps() {
    const TEST_DATA: &'static str = r#"
        u8 = uint .size 1
        u16 = uint .size 2
        data = {
          foo: {
            a: u8,
            b: tstr .size 32,
            bar: {
                c: u16,
                d: bstr .size 32
            }
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
                        KeyVal::new("b", Node::Primative(ConstrainedPrimative::Str(32))).into(),
                        KeyVal::new(
                            "bar",
                            Node::Map(Group {
                                members: vec![
                                    KeyVal::new("c", Node::Foreign("u16".into())).into(),
                                    KeyVal::new(
                                        "d",
                                        Node::Primative(ConstrainedPrimative::Bytes(32))
                                    )
                                    .into(),
                                ]
                            })
                        )
                        .into()
                    ],
                }),
            )
            .into()],
        })
    );
    let linked = link(nodes).unwrap();
    assert_eq!(
        linked["data"],
        LinkedNode::Struct(Fields {
            members: vec![LinkedKeyVal::new(
                "foo",
                LinkedNode::Struct(Fields {
                    members: vec![
                        LinkedKeyVal::new("a", ConstrainedPrimative::U8.into()).into(),
                        LinkedKeyVal::new("b", ConstrainedPrimative::Str(32).into()),
                        LinkedKeyVal::new(
                            "bar",
                            LinkedNode::Struct(Fields {
                                members: vec![
                                    LinkedKeyVal::new("c", ConstrainedPrimative::U16.into()).into(),
                                    LinkedKeyVal::new("d", ConstrainedPrimative::Bytes(32).into())
                                ]
                            })
                        )
                        .into()
                    ],
                }),
            )
            .into()],
        })
    );
}
