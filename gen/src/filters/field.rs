use liquid_core::{Display_filter, Filter, FilterParameters, FilterReflection, ParseFilter};
use liquid_core::{Expression, Result, Runtime};
use liquid_core::{Value, ValueView};

// TODO Input is of tyep Value, arguments are of type Expression
//      Therefore we need to impl From<LinkedKeyVal> for Value
//      Both value and ivt implement serialize/deserialize
#[derive(Debug, FilterParameters)]
struct FieldArgs {
    #[parameter(description = "")]
    language: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "field",
    description = "render a field member of a struct",
    parsed(FieldFilter)
)]
pub struct Field;

#[derive(Debug, Default, Display_filter)]
#[name = "field"]
pub struct FieldFilter {}
impl Filter for FieldFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
        unimplemented!()
    }
}
