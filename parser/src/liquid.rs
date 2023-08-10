use super::node::*;
use liquid_core::model::{KStringCow, Scalar};
use liquid_core::{Object, Value};
use std::{error, fmt};

#[derive(Debug)]
pub enum LiquidError {
    MissingTypeKey,
    MissingKey(String),
    ExpectedObject,
    ExpectedObjectFound(Value),
    ExpectedScalar,
    ExpectedScalarFound(Value),
    ExpectedInteger,
    ExpectedIntegerFound(Value),
    ExpectedArray,
    ExpectedArrayFound(Value),
    WrongType(String),
    UnexpectedValue(String),
    UnexpectedEmptyString,
}

impl fmt::Display for LiquidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidError::MissingTypeKey => write!(f, "missing type key"),
            LiquidError::MissingKey(key) => write!(f, "missing key {}", key),
            LiquidError::ExpectedObject => write!(f, "expected object"),
            LiquidError::ExpectedObjectFound(v) => write!(f, "expected object, found {:?}", v),
            LiquidError::ExpectedScalar => write!(f, "expected scalar"),
            LiquidError::ExpectedScalarFound(v) => write!(f, "expected scalar, found {:?}", v),
            LiquidError::ExpectedInteger => write!(f, "expected integer"),
            LiquidError::ExpectedIntegerFound(v) => write!(f, "expected integer, found {:?}", v),
            LiquidError::ExpectedArray => write!(f, "expected array"),
            LiquidError::ExpectedArrayFound(v) => write!(f, "expected array, found {:?}", v),
            LiquidError::WrongType(ty) => write!(f, "wrong type {}", ty),
            LiquidError::UnexpectedValue(v) => write!(f, "unexpected value {}", v),
            LiquidError::UnexpectedEmptyString => write!(f, "Unexpected empty string"),
        }
    }
}
impl error::Error for LiquidError {}

impl From<ConstrainedPrimative> for Value {
    fn from(value: ConstrainedPrimative) -> Self {
        let mut obj = Object::new();
        obj.insert("type".into(), Value::Scalar("primative".into()));
        match value {
            ConstrainedPrimative::U8 => obj.insert("value".into(), Value::Scalar("u8".into())),
            ConstrainedPrimative::U16 => obj.insert("value".into(), Value::Scalar("u16".into())),
            ConstrainedPrimative::U32 => obj.insert("value".into(), Value::Scalar("u32".into())),
            ConstrainedPrimative::U64 => obj.insert("value".into(), Value::Scalar("u64".into())),
            ConstrainedPrimative::I8 => obj.insert("value".into(), Value::Scalar("i8".into())),
            ConstrainedPrimative::I16 => obj.insert("value".into(), Value::Scalar("i16".into())),
            ConstrainedPrimative::I32 => obj.insert("value".into(), Value::Scalar("i32".into())),
            ConstrainedPrimative::I64 => obj.insert("value".into(), Value::Scalar("i64".into())),
            ConstrainedPrimative::Bool => obj.insert("value".into(), Value::Scalar("bool".into())),
            ConstrainedPrimative::Str(n) => {
                obj.insert("value".into(), Value::Scalar("string".into()));
                obj.insert("len".into(), Value::Scalar((n as i64).into()))
            }
            ConstrainedPrimative::Bytes(n) => {
                obj.insert("value".into(), Value::Scalar("bytes".into()));
                obj.insert("len".into(), Value::Scalar((n as i64).into()))
            }
        };
        Value::Object(obj)
    }
}

impl TryFrom<Value> for ConstrainedPrimative {
    type Error = LiquidError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut obj = get_value_object(value)?;
        check_valid_type("primative", &mut obj)?;
        match get_value_kstr("value", &obj)?.as_str() {
            "u8" => Ok(ConstrainedPrimative::U8),
            "u16" => Ok(ConstrainedPrimative::U16),
            "u32" => Ok(ConstrainedPrimative::U32),
            "u64" => Ok(ConstrainedPrimative::U64),
            "i8" => Ok(ConstrainedPrimative::I8),
            "i16" => Ok(ConstrainedPrimative::I16),
            "i32" => Ok(ConstrainedPrimative::I32),
            "i64" => Ok(ConstrainedPrimative::I64),
            "bool" => Ok(ConstrainedPrimative::Bool),
            "string" => {
                let len = get_value_int("len", &mut obj)?;
                Ok(ConstrainedPrimative::Str(len as u64))
            }
            "bytes" => {
                let len = get_value_int("len", &mut obj)?;
                Ok(ConstrainedPrimative::Bytes(len as u64))
            }
            v => Err(LiquidError::UnexpectedValue(v.to_string())),
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::UInt(u) => Value::Object(liquid_core::object!({
                "type": "literal",
                "value": Value::Scalar(Scalar::from(u as i64)),
                "valueType": "uint"
            })),
            Literal::Int(i) => Value::Object(liquid_core::object!({
                "type": "literal",
                "value": Value::Scalar(Scalar::from(i as i64)),
                "valueType": "int"
            })),
            Literal::Bool(b) => Value::Object(liquid_core::object!({
                "type": "literal",
                "value": Value::Scalar(Scalar::from(b)),
                "valueType": "bool"
            })),
            Literal::Char(c) => Value::Object(liquid_core::object!({
                "type": "literal",
                "value": Value::Scalar(Scalar::from(c.to_string())),
                "valueType": "char"
            })),
            Literal::Str(s) => Value::Object(liquid_core::object!({
                "type": "literal",
                "value": Value::Scalar(Scalar::from(s)),
                "valueType": "string"
            })),
            Literal::Bytes(b) => {
                let value: Vec<Value> = b
                    .into_iter()
                    .map(|b| Value::from(Scalar::from(b)))
                    .collect();
                Value::Object(liquid_core::object!({
                    "type": "literal",
                    "value": value,
                    "valueType": "array"
                }))
            }
        }
    }
}

impl TryFrom<Value> for Literal {
    type Error = LiquidError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut obj = get_value_object(value)?;
        check_valid_type("literal", &mut obj)?;
        match get_value_kstr("valueType", &obj)?.as_str() {
            "uint" => Ok(Literal::UInt(get_value_int("value", &mut obj)? as u64)),
            "int" => Ok(Literal::Int(get_value_int("value", &mut obj)? as i64)),
            "bool" => Ok(Literal::Bool(get_value_bool("value", &mut obj)?)),
            "char" => Ok(Literal::Char(get_value_char("value", &mut obj)?)),
            "string" => Ok(Literal::Str(get_value_kstr("value", &mut obj)?.to_string())),
            "bytes" => {
                let v: Vec<u8> = get_value_array("value", &mut obj)?
                    .into_iter()
                    .map(|v| {
                        Ok(v.into_scalar()
                            .ok_or(LiquidError::ExpectedScalar)?
                            .to_integer()
                            .ok_or(LiquidError::ExpectedInteger)? as u8)
                    })
                    .collect::<Result<Vec<u8>, LiquidError>>()?;
                Ok(Literal::Bytes(v))
            }
            v => Err(LiquidError::UnexpectedValue(v.to_string())),
        }
    }
}

impl From<LinkedNode> for Value {
    fn from(value: LinkedNode) -> Self {
        unimplemented!()
    }
}

impl From<LinkedKeyVal> for Value {
    fn from(value: LinkedKeyVal) -> Self {
        Value::Object(liquid_core::object!({
            "key": value.0
        }))
    }
}

fn check_valid_type<'s>(expect: &'s str, obj: &'s mut Object) -> Result<(), LiquidError> {
    match obj
        .remove("type")
        .ok_or_else(|| LiquidError::MissingTypeKey)?
        .as_view()
        .to_kstr()
        .as_str()
    {
        key if key == expect => Ok(()),
        key => Err(LiquidError::WrongType(key.to_string())),
    }
}

fn get_value_object(value: Value) -> Result<Object, LiquidError> {
    match value {
        Value::Object(obj) => Ok(obj),
        _ => Err(LiquidError::ExpectedObjectFound(value)),
    }
}

fn get_value_kstr<'s>(expect: &'s str, obj: &'s Object) -> Result<KStringCow<'s>, LiquidError> {
    Ok(obj
        .get(expect)
        .ok_or_else(|| LiquidError::MissingKey(expect.to_string()))?
        .as_view()
        .to_kstr())
}

fn get_value_array<'s>(expect: &'s str, obj: &'s mut Object) -> Result<Vec<Value>, LiquidError> {
    match obj
        .remove(expect)
        .ok_or_else(|| LiquidError::MissingKey(expect.to_string()))?
    {
        Value::Array(arr) => Ok(arr),
        _ => unimplemented!(),
    }
}

fn get_value_char<'s>(expect: &'s str, obj: &'s Object) -> Result<char, LiquidError> {
    get_value_kstr(expect, obj)?
        .chars()
        .next()
        .ok_or(LiquidError::UnexpectedEmptyString)
}

fn get_value_int(expect: &str, obj: &mut Object) -> Result<i64, LiquidError> {
    obj.remove(expect)
        .ok_or_else(|| LiquidError::MissingKey(expect.to_string()))?
        .into_scalar()
        .ok_or_else(|| LiquidError::ExpectedScalar)?
        .to_integer()
        .ok_or_else(|| LiquidError::ExpectedInteger)
}

fn get_value_bool(expect: &str, obj: &mut Object) -> Result<bool, LiquidError> {
    obj.remove(expect)
        .ok_or_else(|| LiquidError::MissingKey(expect.to_string()))?
        .into_scalar()
        .ok_or_else(|| LiquidError::ExpectedScalar)?
        .to_bool()
        .ok_or_else(|| LiquidError::ExpectedInteger)
}
