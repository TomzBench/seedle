use crate::language::Language;
use liquid_core::Error;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Expression, Result, Runtime};
use liquid_core::{Value, ValueView};
use seedle_parser::*;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub(crate) struct FieldJsonArgs {
    language: Language,
    public: bool,
    required: bool,
}

#[derive(Debug, FilterParameters)]
struct FieldArgs {
    #[parameter(description = "JSON: language:str, public:bool, required:bool")]
    json: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "field",
    description = "render a field member of a struct",
    parameters(FieldArgs),
    parsed(FieldFilter)
)]
pub struct Field;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "field"]
pub struct FieldFilter {
    #[parameters]
    args: FieldArgs,
}
impl Filter for FieldFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let json = self.args.evaluate(runtime)?.json;
        let args = serde_json::from_str::<FieldJsonArgs>(json.to_kstr().as_str())
            .map_err(|e| Error::with_msg(e.to_string()))?;
        let node = LinkedKeyVal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        let field = FieldFormatter {
            language: &args.language,
            required: args.required,
            public: args.public,
            node: &node,
        };
        Ok(Value::Scalar(format!("{}", field).into()))
    }
}

struct FieldFormatter<'s> {
    language: &'s Language,
    required: bool,
    public: bool,
    node: &'s LinkedKeyVal,
}
impl<'s> fmt::Display for FieldFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let language = self.language;
        let key = language.fieldify(self.node.key());
        let node = self.node.val();
        if self.public {
            write!(f, "pub {}: ", key)?;
        } else {
            write!(f, "{}: ", key)?;
        }
        match self.required {
            true => write!(f, "{}", NodeFormatter { language, node }),
            false => write!(f, "Option<{}>", NodeFormatter { language, node }),
        }
    }
}

struct NodeFormatter<'s> {
    language: &'s Language,
    node: &'s LinkedNode,
}
impl<'s> fmt::Display for NodeFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let language = self.language;
        match self.node {
            LinkedNode::Primative(p) => PrimativeFormatter(p).fmt(f),
            LinkedNode::Array(a) => ArrayFormatter { language, node: a }.fmt(f),
            LinkedNode::ForeignStruct(s) => StructFormatter { language, node: s }.fmt(f),
            _ => Err(fmt::Error),
        }
    }
}

struct PrimativeFormatter<'s>(&'s ConstrainedPrimative);
impl<'s> fmt::Display for PrimativeFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ConstrainedPrimative::U8 => write!(f, "u8"),
            ConstrainedPrimative::U16 => write!(f, "u16"),
            ConstrainedPrimative::U32 => write!(f, "u32"),
            ConstrainedPrimative::U64 => write!(f, "u64"),
            ConstrainedPrimative::I8 => write!(f, "i8"),
            ConstrainedPrimative::I16 => write!(f, "i16"),
            ConstrainedPrimative::I32 => write!(f, "i32"),
            ConstrainedPrimative::I64 => write!(f, "i64"),
            ConstrainedPrimative::Bool => write!(f, "bool"),
            ConstrainedPrimative::Str(n) => write!(f, "[u8; {}]", n),
            ConstrainedPrimative::Bytes(n) => write!(f, "[u8; {}]", n),
        }
    }
}

struct StructFormatter<'s> {
    language: &'s Language,
    node: &'s str,
}
impl<'s> fmt::Display for StructFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.language.structify(self.node))
    }
}

struct ArrayFormatter<'s> {
    language: &'s Language,
    node: &'s LinkedArray,
}
impl<'s> fmt::Display for ArrayFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatter = NodeFormatter {
            language: self.language,
            node: self.node.ty.as_ref(),
        };
        write!(f, "[{}; {}]", formatter, self.node.len)
    }
}

struct FieldDefaultFormatter<'s>(&'s LinkedNode);
impl<'s> fmt::Display for FieldDefaultFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            LinkedNode::Primative(ConstrainedPrimative::Str(n)) => write!(f, "[0; {}]", n),
            LinkedNode::Array(LinkedArray { ty, len }) => match ty.as_ref() {
                LinkedNode::Primative(ConstrainedPrimative::U8)
                | LinkedNode::Primative(ConstrainedPrimative::U16)
                | LinkedNode::Primative(ConstrainedPrimative::U32)
                | LinkedNode::Primative(ConstrainedPrimative::U64)
                | LinkedNode::Primative(ConstrainedPrimative::I8)
                | LinkedNode::Primative(ConstrainedPrimative::I16)
                | LinkedNode::Primative(ConstrainedPrimative::I32)
                | LinkedNode::Primative(ConstrainedPrimative::I64) => write!(f, "[0; {}]", len),
                LinkedNode::ForeignStruct(_) => write!(f, "[Default::default(); {}]", len),
                _ => Err(fmt::Error),
            },
            _ => write!(f, "Default::default()"),
        }
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "field_default",
    description = "Render a field default initializer",
    parsed(FieldDefaultFilter)
)]
pub struct FieldDefault;

#[derive(Debug, Default, Display_filter)]
#[name = "field_default"]
pub struct FieldDefaultFilter {}
impl Filter for FieldDefaultFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        let node = LinkedKeyVal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        Ok(Value::Scalar(
            format!("{}: {}", node.key(), FieldDefaultFormatter(node.val())).into(),
        ))
    }
}
