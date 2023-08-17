use super::error::invalid_fmt;
use crate::language::Language;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Error, Expression, Result, Runtime};
use liquid_core::{Value, ValueView};
use seedle_parser::*;
use serde::Deserialize;
use std::fmt::Write;

macro_rules! render {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(std::format_args!($($arg)*)).map_err(invalid_fmt)
    };
}

#[derive(Deserialize)]
pub(crate) struct AttrFieldJsonArgs {
    index: u8,
    language: Language,
    required: bool,
}

struct MinicborConverters {
    deserialize: &'static str,
    default: &'static str,
}
impl MinicborConverters {
    fn new(partial: bool) -> Self {
        match partial {
            true => MinicborConverters {
                deserialize: "de_option_str_as_bytes",
                default: "make_option_default_bytes",
            },
            false => MinicborConverters {
                deserialize: "de_str_as_bytes",
                default: "make_default_bytes",
            },
        }
    }
}

// TODO Input is of tyep Value, arguments are of type Expression
//      Therefore we need to impl From<LinkedKeyVal> for Value
//      Both value and ivt implement serialize/deserialize
#[derive(Debug, FilterParameters)]
struct AttrFieldArgs {
    #[parameter(description = "")]
    json: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "attr_field",
    description = "render field attributes",
    parameters(AttrFieldArgs),
    parsed(AttrFieldFilter)
)]
pub struct AttrField;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "attr_field"]
pub struct AttrFieldFilter {
    #[parameters]
    args: AttrFieldArgs,
}
impl Filter for AttrFieldFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let json = self.args.evaluate(runtime)?.json;
        let args = serde_json::from_str::<AttrFieldJsonArgs>(json.to_kstr().as_str())
            .map_err(|e| Error::with_msg(e.to_string()))?;
        let input = LinkedKeyVal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        match args.language {
            Language::C => attr_field_c(input.into_val(), args),
            _ => attr_field_typescript(input.into_val(), args),
        }
    }
}
#[inline]
fn attr_field_c(input: LinkedNode, args: AttrFieldJsonArgs) -> Result<Value> {
    let mut ret = String::new();
    let index = args.index;
    match input {
        LinkedNode::Array(LinkedArray { ty, .. }) => match ty.as_ref() {
            LinkedNode::Primative(ConstrainedPrimative::U8) => {
                render!(ret, "#[cbor(n({}), with=\"minicbor::bytes\")]", index)
            }
            _ => render!(ret, "#[n({})]", index),
        },
        LinkedNode::Primative(ConstrainedPrimative::Str(_)) => {
            render!(ret, "#[cbor(n({}), with=\"minicbor::bytes\")]\n", index)
        }
        _ => render!(ret, "#[n({})]", args.index),
    }?;
    Ok(Value::Scalar(ret.into()))
}

#[inline]
fn attr_field_typescript(input: LinkedNode, args: AttrFieldJsonArgs) -> Result<Value> {
    let index = args.index;
    let converters = MinicborConverters::new(!args.required);
    let deserialize = converters.deserialize;
    let default = converters.default;
    let mut ret = String::new();
    match input {
        LinkedNode::Array(LinkedArray { ty, len }) => match ty.as_ref() {
            LinkedNode::Primative(ConstrainedPrimative::U8) if len < 32 => {
                render!(ret, "#[cbor(n({}), with=\"minicbor::bytes\")]", index)
            }
            LinkedNode::Primative(ConstrainedPrimative::U8) => {
                render!(ret, "#[cbor(n({}), with=\"minicbor::bytes\")]\n", index)?;
                render!(ret, "#[serde(serde_serialize_with=\"ser_bytes_as_str\")]\n")?;
                render!(ret, "#[serde(serde_deserialize_with=\"{}\")]", deserialize)
            }
            _ => render!(ret, "#[n({})]", index),
        },
        LinkedNode::Primative(ConstrainedPrimative::Str(_)) => {
            render!(ret, "#[cbor(n({}), with=\"minicbor::bytes\")]\n", index)?;
            render!(ret, "#[serde(default=\"{}\")]\n", default)?;
            render!(ret, "#[serde(serde_serialize_with=\"ser_bytes_as_str\")]\n")?;
            render!(ret, "#[serde(serde_deserialize_with=\"{}\")]", deserialize)
        }
        _ => render!(ret, "#[n({})]", index),
    }?;
    Ok(Value::Scalar(ret.into()))
}
