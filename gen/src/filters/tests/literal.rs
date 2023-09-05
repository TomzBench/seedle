use crate::filters::literal::Literal;
use liquid_core::Value;

#[test]
fn expect_literal_bool() {
    let literal = Value::from(seedle_parser::Literal::Bool(false));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "c", "foo").unwrap(),
        Value::Scalar("pub const FOO: bool = false;".into())
    );
}

#[test]
fn expect_literal_int() {
    let literal = Value::from(seedle_parser::Literal::Int(42));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "c", "foo").unwrap(),
        Value::Scalar("pub const FOO: i32 = 42;".into())
    );
}

#[test]
fn expect_literal_uint() {
    let literal = Value::from(seedle_parser::Literal::UInt(42));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "c", "foo").unwrap(),
        Value::Scalar("pub const FOO: u32 = 42;".into())
    );
}

#[test]
fn expect_literal_str() {
    let literal = Value::from(seedle_parser::Literal::Str("hello".into()));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "c", "foo").unwrap(),
        Value::Scalar("pub const FOO: 'static = \"hello\";".into())
    );
}

#[test]
fn expect_literal_char() {
    let literal = Value::from(seedle_parser::Literal::Char('a'));
    assert_eq!(
        liquid_core::call_filter!(Literal, literal, "c", "foo").unwrap(),
        Value::Scalar("pub const FOO: char = 'a';".into())
    );
}
