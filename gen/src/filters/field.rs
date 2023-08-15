use super::error::invalid_argument;
use crate::language::Language;
use liquid_core::Error;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Expression, Result, Runtime};
use liquid_core::{Value, ValueView};
use seedle_parser::*;
use std::fmt;

// TODO Input is of tyep Value, arguments are of type Expression
//      Therefore we need to impl From<LinkedKeyVal> for Value
//      Both value and ivt implement serialize/deserialize
#[derive(Debug, FilterParameters)]
struct FieldArgs {
    #[parameter(description = "Langauge (C,Rust,Typescript)")]
    language: Expression,
    #[parameter(description = "Is this member public or private?")]
    public: Expression,
    #[parameter(description = "Is this member required or optional?")]
    required: Expression,
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
        let args = self.args.evaluate(runtime)?;

        let node = LinkedKeyVal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        let language = &Language::try_from(args.language.to_value())?;
        let public = args
            .public
            .as_scalar()
            .ok_or_else(|| invalid_argument("public", "Boolean expected"))?
            .to_bool()
            .ok_or_else(|| invalid_argument("public", "Boolean expected"))?;
        let required = args
            .required
            .as_scalar()
            .ok_or_else(|| invalid_argument("required", "Boolean expected"))?
            .to_bool()
            .ok_or_else(|| invalid_argument("required", "Boolean expected"))?;
        let field = FieldFormatter {
            language,
            required,
            public,
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
