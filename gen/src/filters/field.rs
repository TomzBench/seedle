use super::error::invalid_argument;
use heck::ToUpperCamelCase;
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
    #[parameter(description = "Is this member required or optional?")]
    required: Expression,
    #[parameter(description = "Is this member public or private?")]
    public: Expression,
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
        let field = RustField {
            required,
            public,
            node: &node,
        };
        Ok(Value::Scalar(format!("{}", field).into()))
    }
}

struct RustField<'s> {
    required: bool,
    public: bool,
    node: &'s LinkedKeyVal,
}
impl<'s> fmt::Display for RustField<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.public {
            write!(f, "pub {}: ", self.node.key())?;
        } else {
            write!(f, "{}: ", self.node.key())?;
        }
        match self.required {
            true => write!(f, "{}", RustFieldNode(self.node.val())),
            false => write!(f, "Option<{}>", RustFieldNode(self.node.val())),
        }
    }
}

struct RustFieldNode<'s>(&'s LinkedNode);
impl<'s> fmt::Display for RustFieldNode<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            LinkedNode::Primative(p) => RustFieldPrimative(p).fmt(f),
            LinkedNode::Array(a) => RustFieldArray(a).fmt(f),
            LinkedNode::ForeignStruct(s) => RustFieldStruct(s).fmt(f),
            _ => Err(fmt::Error),
        }
    }
}

struct RustFieldPrimative<'s>(&'s ConstrainedPrimative);
impl<'s> fmt::Display for RustFieldPrimative<'s> {
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

struct RustFieldStruct<'s>(&'s str);
impl<'s> fmt::Display for RustFieldStruct<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_upper_camel_case())
    }
}

struct RustFieldArray<'s>(&'s LinkedArray);
impl<'s> fmt::Display for RustFieldArray<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", RustFieldNode(self.0.ty.as_ref()), self.0.len)
    }
}
