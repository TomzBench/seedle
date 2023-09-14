use crate::filters::attrs::AttrField;
use liquid_core::Value;
use seedle_parser::*;

// NOTE we dont want to test the filters too much because integration testing will make sure we
// compile. We just do some basic testing here with arguments and inputs. But the actual context of
// the attribute strings should be validated with the trybuild crate

#[test]
fn expect_attr_primative() {
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    let args = r#"{"language":"c", "index": 3, "required": true}"#;
    assert_eq!(
        liquid_core::call_filter!(AttrField, field, args).unwrap(),
        Value::Scalar("#[n(3)]".into())
    );
}

#[test]
fn expect_attr_u8_arr() {
    let field = Value::from(LinkedKeyVal::new(
        "field",
        LinkedArray::new(ConstrainedPrimative::U8.into(), 3).into(),
    ));
    let args = r#"{"language":"c", "index": 3, "required": true}"#;
    assert_eq!(
        liquid_core::call_filter!(AttrField, field, args).unwrap(),
        Value::Scalar("#[cbor(n(3), with=\"minicbor::bytes\")]".into())
    );
}
