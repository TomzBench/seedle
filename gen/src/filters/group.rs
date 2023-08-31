use super::error::invalid_input;
use liquid_core::model::KStringCow;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Object, Value, ValueView};
use liquid_core::{Result, Runtime};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "group",
    description = "group an object of keyed objects by their prefix, ${GROUP}_...",
    parsed(GroupFilter)
)]
pub struct Group;

#[derive(Debug, Default, Display_filter)]
#[name = "group"]
pub struct GroupFilter {}
impl Filter for GroupFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        Ok(input
            .as_object()
            .ok_or_else(|| invalid_input("Object expected"))?
            .iter()
            .fold(Object::new(), fold_group)
            .to_value())
    }
}

fn fold_group<'a>(mut state: Object, (key, val): (KStringCow<'a>, &dyn ValueView)) -> Object {
    if let Some((group, rest)) = key.split_once("_") {
        insert(&mut state, group, rest, val);
    } else {
        insert(&mut state, "_", &key, val);
    }
    state
}

fn insert(obj: &mut Object, group: &str, key: &str, node: &dyn ValueView) {
    if let Some(Value::Object(inner)) = obj.get_mut(group) {
        inner.insert(key.to_string().into(), node.to_value());
    } else {
        let mut inner = Object::new();
        inner.insert(key.to_string().into(), node.to_value());
        obj.insert(group.to_string().into(), inner.to_value());
    }
}
