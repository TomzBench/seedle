use super::error::{invalid_argument, invalid_input};
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Expression, Result, Runtime};
use liquid_core::{Value, ValueView};

// TODO Input is of tyep Value, arguments are of type Expression
//      Therefore we need to impl From<LinkedKeyVal> for Value
//      Both value and ivt implement serialize/deserialize
#[derive(Debug, FilterParameters)]
struct CollectArgs {
    #[parameter(description = "")]
    name: Expression,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "collect",
    description = "render a collect member of a struct",
    parameters(CollectArgs),
    parsed(CollectFilter)
)]
pub struct Collect;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "collect"]
pub struct CollectFilter {
    #[parameters]
    args: CollectArgs,
}
impl Filter for CollectFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let name = args
            .name
            .as_scalar()
            .ok_or_else(|| invalid_argument("name", "string expected"))?
            .into_cow_str();
        let obj = input
            .as_object()
            .ok_or_else(|| invalid_input("Object expected"))?;
        let values = obj
            .values()
            .filter_map(|val| {
                let inner = val.as_object()?;
                match inner.get("type")?.as_scalar()?.into_cow_str().as_ref() {
                    n if n == name => Some(inner.to_value()),
                    _ => None,
                }
            })
            .collect::<Vec<Value>>();
        Ok(Value::Array(values))
    }
}
