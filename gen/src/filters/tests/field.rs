use crate::filters::field::Field;
use liquid_core::Value;
use seedle_parser::*;

#[test]
fn expect_field_primative() {
    // Input required public
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    let args = r#"{"language": "c", "required": true, "public": true}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("pub field: u8".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    let args = r#"{"language": "c", "public": true, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("pub field: Option<u8>".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    let args = r#"{"language": "c", "public": false, "required": true}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: u8".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<u8>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        ConstrainedPrimative::Str(3).into(),
    ));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<[u8; 3]>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        ConstrainedPrimative::Bytes(3).into(),
    ));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<[u8; 3]>".into())
    );
}

#[test]
fn expect_field_array() {
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedArray::new(ConstrainedPrimative::U8.into(), 3).into(),
    ));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<[u8; 3]>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedArray::new(LinkedNode::ForeignStruct("foo_bar".into()), 3).into(),
    ));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<[foo_bar; 3]>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedArray::new(LinkedNode::ForeignStruct("foo_bar".into()), 3).into(),
    ));
    let args = r#"{"language": "typescript", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<[FooBar; 3]>".into())
    );
}

#[test]
fn expect_field_struct() {
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedNode::ForeignStruct("foo_bar".into()),
    ));
    let args = r#"{"language": "c", "public": false, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: Option<foo_bar>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedNode::ForeignStruct("foo_bar".into()),
    ));
    let args = r#"{"language": "c", "public": false, "required": true}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("field: foo_bar".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field_foo",
        LinkedNode::ForeignStruct("foo_bar".into()),
    ));
    let args = r#"{"language": "typescript", "public": true, "required": false}"#;
    assert_eq!(
        liquid_core::call_filter!(Field, field, args).unwrap(),
        Value::Scalar("pub fieldFoo: Option<FooBar>".into())
    );
}
