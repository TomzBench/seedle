use super::error::invalid_argument;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Error, Expression, Result, Runtime};
use liquid_core::{Value, ValueView};
use std::fmt;

#[derive(Debug, FilterParameters)]
struct LiteralArgs {
    #[parameter(description = "")]
    name: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "literal",
    description = "Render a global literal",
    parameters(LiteralArgs),
    parsed(LiteralFilter)
)]
pub struct Literal;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "literal"]
pub struct LiteralFilter {
    #[parameters]
    args: LiteralArgs,
}
impl Filter for LiteralFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let name = args
            .name
            .as_scalar()
            .ok_or_else(|| invalid_argument("name", "string expected"))?
            .into_cow_str();
        let literal = seedle_parser::Literal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        let fmtr = LiteralFormatter {
            name: &name,
            literal: &literal,
        };
        Ok(Value::Scalar(format!("{}", fmtr).into()))
    }
}

struct LiteralFormatter<'s> {
    name: &'s str,
    literal: &'s seedle_parser::Literal,
}

impl<'s> fmt::Display for LiteralFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use seedle_parser::Literal;
        match self.literal {
            Literal::Bool(v) => write!(
                f,
                "pub const {name}: {ty} = {val}",
                name = self.name,
                ty = "bool",
                val = v
            ),
            Literal::Int(v) => write!(
                f,
                "pub const {name}: {ty} = {val}",
                name = self.name,
                ty = "i32",
                val = v
            ),
            Literal::UInt(v) => write!(
                f,
                "pub const {name}: {ty} = {val}",
                name = self.name,
                ty = "u32",
                val = v
            ),
            Literal::Str(v) => write!(
                f,
                "pub const {name}: {ty} = \"{val}\"",
                name = self.name,
                ty = "'static",
                val = v
            ),
            Literal::Char(v) => write!(
                f,
                "pub const {name}: {ty} = '{val}'",
                name = self.name,
                ty = "char",
                val = v
            ),
            Literal::Bytes(_) => Err(fmt::Error),
        }
    }
}
