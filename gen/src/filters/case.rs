use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Result, Runtime};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "shouty_snake_case",
    description = "Convert input to a SHOUTY_SNAKE_CASE",
    parsed(ShoutySnakeCaseFilter)
)]
pub struct ShoutySnakeCase;

#[derive(Debug, Default, Display_filter)]
#[name = "shouty_snake_case"]
pub struct ShoutySnakeCaseFilter {}
impl Filter for ShoutySnakeCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        Ok(Value::scalar(input.to_kstr().to_shouty_snake_case()))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "lower_camel_case",
    description = "Convert input to a lowerCamelCase",
    parsed(LowerCamelCaseFilter)
)]
pub struct LowerCamelCase;

#[derive(Debug, Default, Display_filter)]
#[name = "lower_camel_case"]
pub struct LowerCamelCaseFilter {}
impl Filter for LowerCamelCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        Ok(Value::scalar(input.to_kstr().to_lower_camel_case()))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "snake_case",
    description = "Convert input to a snake_case",
    parsed(SnakeCaseFilter)
)]
pub struct SnakeCase;

#[derive(Debug, Default, Display_filter)]
#[name = "snake_case"]
pub struct SnakeCaseFilter {}
impl Filter for SnakeCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        Ok(Value::scalar(input.to_kstr().to_snake_case()))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "upper_camel_case",
    description = "Convert input to a UpperCamelCase",
    parsed(UpperCamelCaseFilter)
)]
pub struct UpperCamelCase;

#[derive(Debug, Default, Display_filter)]
#[name = "upper_camel_case"]
pub struct UpperCamelCaseFilter {}
impl Filter for UpperCamelCaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        Ok(Value::scalar(input.to_kstr().to_upper_camel_case()))
    }
}
