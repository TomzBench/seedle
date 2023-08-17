use crate::filters::json::*;
use liquid_core::{self, Value};

#[test]
fn expect_shouty_snake_case() {
    let input = r#"{
        "a-bool": true,
        "a-num": 32,
        "a-str": "hello",
        "a-arr": [1, 2, 3],
        "a-obj": {
            "inner": 42
        }}"#;
    let expect = Value::Object(liquid_core::object!({
        "a-bool": true,
        "a-num": 32,
        "a-str": "hello",
        "a-arr": [1,2,3],
        "a-obj": {
            "inner": 42
        }
    }));
    assert_eq!(liquid_core::call_filter!(Json, input).unwrap(), expect);
}
