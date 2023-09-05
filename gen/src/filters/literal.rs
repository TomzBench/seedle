use super::error::invalid_argument;
use crate::language::Language;
use heck::ToShoutySnakeCase;
use indoc::writedoc;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Error, Expression, Result, Runtime};
use liquid_core::{Value, ValueView};
use std::fmt;

#[derive(Debug, FilterParameters)]
struct LiteralArgs {
    #[parameter(description = "")]
    language: Expression,
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
        let language = args
            .language
            .as_scalar()
            .map(|s| s.into_cow_str())
            .and_then(|s| Language::try_from(s.as_ref()).ok())
            .ok_or_else(|| invalid_argument("language", "unknown language string"))?;
        let literal = seedle_parser::Literal::try_from(input.to_value())
            .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
        let fmtr = LiteralFormatter {
            name: &name,
            literal: &literal,
            language,
        };
        // TODO if language=TYPESCRIPT then add the wasm_type
        Ok(Value::Scalar(format!("{}", fmtr).into()))
    }
}

struct LiteralFormatter<'s> {
    name: &'s str,
    language: Language,
    literal: &'s seedle_parser::Literal,
}

impl<'s> fmt::Display for LiteralFormatter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use seedle_parser::Literal;
        let name = self.name.to_shouty_snake_case();
        match (self.language, self.literal) {
            (Language::Typescript, Literal::Bool(val)) => writedoc! {
                f,
                r#"
                    pub const {name}: {ty} = {val};
                    #[wasm_bindgen(typescript_custom_section)]
                    const TS_{name}: &'static str = "export type {name} = {val}";"#,
                name = name,
                ty = "bool",
                val = val
            },
            (_, Literal::Bool(val)) => write!(
                f,
                "pub const {name}: {ty} = {val};",
                name = name,
                ty = "bool",
                val = val
            ),
            (Language::Typescript, Literal::Int(v)) => writedoc! {
                f,
                r#"
                    pub const {name}: {ty} = {val};
                    #[wasm_bindgen(typescript_custom_section)]
                    const TS_{name}: &'static str = "export type {name} = {val}";"#,
                name = name,
                ty = "i32",
                val = v
            },
            (_, Literal::Int(v)) => write!(
                f,
                "pub const {name}: {ty} = {val};",
                name = name,
                ty = "i32",
                val = v
            ),
            (Language::Typescript, Literal::UInt(v)) => writedoc! {
                f,
                r#"
                    pub const {name}: {ty} = {val};
                    #[wasm_bindgen(typescript_custom_section)]
                    const TS_{name}: &'static str = "export type {name} = {val}";"#,
                name = name,
                ty = "i32",
                val = v
            },
            (_, Literal::UInt(v)) => write!(
                f,
                "pub const {name}: {ty} = {val};",
                name = name,
                ty = "u32",
                val = v
            ),
            (Language::Typescript, Literal::Str(v)) => writedoc! {
                f,
                r#"
                    pub const {name}: {ty} = "{val}";
                    #[wasm_bindgen(typescript_custom_section)]
                    const TS_{name}: &'static str = "export type {name} = "{val}";"#,
                name = name,
                ty = "i32",
                val = v
            },
            (_, Literal::Str(v)) => write!(
                f,
                r#"pub const {name}: {ty} = "{val}";"#,
                name = name,
                ty = "'static",
                val = v
            ),
            (Language::Typescript, Literal::Char(v)) => writedoc! {
                f,
                r#"
                    pub const {name}: {ty} = '{val}';
                    #[wasm_bindgen(typescript_custom_section)]
                    const TS_{name}: &'static str = "export type {name} = '{val}';"#,
                name = name,
                ty = "i32",
                val = v
            },
            (_, Literal::Char(v)) => write!(
                f,
                "pub const {name}: {ty} = '{val}';",
                name = name,
                ty = "char",
                val = v
            ),
            (_, Literal::Bytes(_)) => Err(fmt::Error),
        }
    }
}
