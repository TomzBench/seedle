use crate::filters::field::Field;
use liquid_core::Value;
use seedle_parser::*;

#[test]
fn expect_field_primative() {
    // Input required public
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", true, true).unwrap(),
        Value::Scalar("pub field: u8".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", false, true).unwrap(),
        Value::Scalar("pub field: Option<u8>".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", true, false).unwrap(),
        Value::Scalar("field: u8".into())
    );
    let field = Value::from(LinkedKeyVal::new("field", ConstrainedPrimative::U8.into()));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", false, false).unwrap(),
        Value::Scalar("field: Option<u8>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        ConstrainedPrimative::Str(3).into(),
    ));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", false, false).unwrap(),
        Value::Scalar("field: Option<[u8; 3]>".into())
    );
    let field = Value::from(LinkedKeyVal::new(
        "field",
        ConstrainedPrimative::Bytes(3).into(),
    ));
    assert_eq!(
        liquid_core::call_filter!(Field, field, "c", false, false).unwrap(),
        Value::Scalar("field: Option<[u8; 3]>".into())
    );
}

