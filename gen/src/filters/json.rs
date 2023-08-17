use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Error, Result, Runtime};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "json",
    description = "Convert a JSON string into a liquid object",
    parsed(JsonFilter)
)]
pub struct Json;

#[derive(Debug, Default, Display_filter)]
#[name = "shouty_snake_case"]
pub struct JsonFilter {}
impl Filter for JsonFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        serde_json::from_str(&input.to_kstr().as_str()).map_err(|e| Error::with_msg(e.to_string()))
    }
}
