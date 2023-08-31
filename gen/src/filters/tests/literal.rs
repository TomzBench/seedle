use crate::filters::literal::Literal;
use liquid_core::Value;

#[test]
fn expect_literal_bool() {
    let literal = Value::from(seedle_parser::Literal::Bool(false));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "foo").unwrap(),
        Value::Scalar("pub const foo: bool = false".into())
    );
}

#[test]
fn expect_literal_int() {
    let literal = Value::from(seedle_parser::Literal::Int(42));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "foo").unwrap(),
        Value::Scalar("pub const foo: i32 = 42".into())
    );
}

#[test]
fn expect_literal_uint() {
    let literal = Value::from(seedle_parser::Literal::UInt(42));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "foo").unwrap(),
        Value::Scalar("pub const foo: u32 = 42".into())
    );
}

#[test]
fn expect_literal_str() {
    let literal = Value::from(seedle_parser::Literal::Str("hello".into()));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "foo").unwrap(),
        Value::Scalar("pub const foo: 'static = \"hello\"".into())
    );
}

#[test]
fn expect_literal_char() {
    let literal = Value::from(seedle_parser::Literal::Char('a'));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "foo").unwrap(),
        Value::Scalar("pub const foo: char = 'a'".into())
    );
}
