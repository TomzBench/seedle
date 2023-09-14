use crate::filters::group::Group;
use liquid_core::{Object, Value};
use seedle_parser::{LinkedNode, Literal};

#[test]
fn expect_collect_group_literals() {
    const TEST_DATA: &'static str = r#"
        litchar = 'a'
        litbool = true
        litstr = "hello"
        litnum = 3
        foo_char = 'a'
        foo_bool = true
        foo_str = "hello"
        foo_num = 3
        bar_char = 'a'
        bar_bool = true
        bar_str = "hello"
        bar_num = 3
        "#;

    // Create input to filter.
    let input = seedle_parser::parse(TEST_DATA).unwrap().into_iter().fold(
        Object::new(),
        |mut state, (key, val)| {
            state.insert(key.into(), val.into());
            state
        },
    );

    let expect_ungrouped = Value::Object(liquid_core::object!({
        "litchar": Value::from(LinkedNode::Literal(Literal::Char('a'))),
        "litbool": Value::from(LinkedNode::Literal(Literal::Bool(true))),
        "litstr": Value::from(LinkedNode::Literal(Literal::Str("hello".into()))),
        "litnum": Value::from(LinkedNode::Literal(Literal::UInt(3))),
    }));

    let expect_foo = Value::Object(liquid_core::object!({
        "char": Value::from(LinkedNode::Literal(Literal::Char('a'))),
        "bool": Value::from(LinkedNode::Literal(Literal::Bool(true))),
        "str": Value::from(LinkedNode::Literal(Literal::Str("hello".into()))),
        "num": Value::from(LinkedNode::Literal(Literal::UInt(3))),
    }));

    let expect_bar = Value::Object(liquid_core::object!({
        "char": Value::from(LinkedNode::Literal(Literal::Char('a'))),
        "bool": Value::from(LinkedNode::Literal(Literal::Bool(true))),
        "str": Value::from(LinkedNode::Literal(Literal::Str("hello".into()))),
        "num": Value::from(LinkedNode::Literal(Literal::UInt(3))),
    }));

    let expect_group = Value::Object(liquid_core::object!({
        "foo": expect_foo,
        "bar": expect_bar,
        "_": expect_ungrouped
    }));

    assert_eq!(
        expect_group,
        liquid_core::call_filter!(Group, input).unwrap()
    );
}
