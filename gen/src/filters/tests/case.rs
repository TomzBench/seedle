use crate::filters::case::*;
use liquid_core::{self, Value};

#[test]
fn expect_shouty_snake_case() {
    assert_eq!(
        liquid_core::call_filter!(ShoutySnakeCase, "helloWorld").unwrap(),
        Value::scalar("HELLO_WORLD")
    );
}

#[test]
fn expect_lower_camel_case() {
    assert_eq!(
        liquid_core::call_filter!(LowerCamelCase, "HELLO_WORLD").unwrap(),
        Value::scalar("helloWorld")
    );
}

#[test]
fn expect_snake_case() {
    assert_eq!(
        liquid_core::call_filter!(SnakeCase, "helloWorld").unwrap(),
        Value::scalar("hello_world")
    );
}

#[test]
fn expect_upper_camel_case() {
    assert_eq!(
        liquid_core::call_filter!(UpperCamelCase, "hello_world").unwrap(),
        Value::scalar("HelloWorld")
    );
}
