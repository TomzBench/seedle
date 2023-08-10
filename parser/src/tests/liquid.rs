use crate::node::*;
use liquid_core::{model::Scalar, Value};

#[test]
fn expect_value_from_primative() {
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("u8".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::U8));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("u16".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::U16));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("u32".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::U32));
    let o = Value::Object(liquid_core::object!({
         "type":"primative",
        "value":Value::Scalar("u64".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::U64));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("i8".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::I8));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("i16".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::I16));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("i32".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::I32));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("i64".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::I64));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("i64".into()),
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::I64));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("string".into()),
        "len": Value::Scalar(3.into())
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::Str(3)));
    let o = Value::Object(liquid_core::object!({
        "type":"primative",
        "value":Value::Scalar("bytes".into()),
        "len": Value::Scalar(3.into())
    }));
    assert_eq!(o, Value::from(ConstrainedPrimative::Bytes(3)));
}

#[test]
fn expect_primative_from_value() {
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "u8",
    }));
    assert_eq!(
        ConstrainedPrimative::U8,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "u16",
    }));
    assert_eq!(
        ConstrainedPrimative::U16,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "u32",
    }));
    assert_eq!(
        ConstrainedPrimative::U32,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "u64",
    }));
    assert_eq!(
        ConstrainedPrimative::U64,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "i8",
    }));
    assert_eq!(
        ConstrainedPrimative::I8,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "i16",
    }));
    assert_eq!(
        ConstrainedPrimative::I16,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "i32",
    }));
    assert_eq!(
        ConstrainedPrimative::I32,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "i64",
    }));
    assert_eq!(
        ConstrainedPrimative::I64,
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "string",
        "len": 32
    }));
    assert_eq!(
        ConstrainedPrimative::Str(32),
        ConstrainedPrimative::try_from(v).unwrap()
    );
    let v = Value::Object(liquid_core::object!({
        "type": "primative",
        "value": "bytes",
        "len": 32
    }));
    assert_eq!(
        ConstrainedPrimative::Bytes(32),
        ConstrainedPrimative::try_from(v).unwrap()
    );
}

#[test]
fn expect_value_from_literal() {
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value":Value::Scalar(8.into()),
        "valueType": "uint"
    }));
    assert_eq!(expect, Value::from(Literal::UInt(8)));
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value":Value::Scalar(8.into()),
        "valueType": "int"
    }));
    assert_eq!(expect, Value::from(Literal::Int(8)));
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value":Value::Scalar(false.into()),
        "valueType": "bool"
    }));
    assert_eq!(expect, Value::from(Literal::Bool(false)));
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value":Value::Scalar("C".into()),
        "valueType": "char"
    }));
    assert_eq!(expect, Value::from(Literal::Char('C')));
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value":Value::Scalar("Cat".into()),
        "valueType": "string"
    }));
    assert_eq!(expect, Value::from(Literal::Str("Cat".into())));
    let arr = vec![1, 2, 3]
        .into_iter()
        .map(|i| Value::Scalar(Scalar::from(i)))
        .collect();
    let expect = Value::Object(liquid_core::object!({
        "type":"literal",
        "value": Value::Array(arr),
        "valueType": "array"
    }));
    assert_eq!(expect, Value::from(Literal::Bytes(vec![1, 2, 3])));
}

#[test]
fn expect_literal_from_value() {
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": Value::Scalar(8.into()),
        "valueType": "uint"
    }));
    assert_eq!(Literal::UInt(8), Literal::try_from(v).unwrap());
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": Value::Scalar(8.into()),
        "valueType": "int"
    }));
    assert_eq!(Literal::Int(8), Literal::try_from(v).unwrap());
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": Value::Scalar(true.into()),
        "valueType": "bool"
    }));
    assert_eq!(Literal::Bool(true), Literal::try_from(v).unwrap());
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": "C",
        "valueType": "char"
    }));
    assert_eq!(Literal::Char('C'), Literal::try_from(v).unwrap());
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": "Cat",
        "valueType": "string"
    }));
    assert_eq!(Literal::Str("Cat".into()), Literal::try_from(v).unwrap());
    let arr = vec![1, 2, 3]
        .into_iter()
        .map(|i| Value::Scalar(Scalar::from(i)))
        .collect();
    let v = Value::Object(liquid_core::object!({
        "type": "literal",
        "value": Value::Array(arr),
        "valueType": "bytes"}));
    assert_eq!(Literal::Bytes(vec![1, 2, 3]), Literal::try_from(v).unwrap());
}

#[test]
fn expect_linked_keyval() {}
