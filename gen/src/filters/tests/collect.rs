use crate::filters::collect::Collect;
use liquid_core::{Object, Value};
use seedle_parser::{ConstrainedPrimative, Fields, LinkedKeyVal, LinkedNode, Literal};

// TODO
//      - Collect should return an object instead of an array so we can preserve
//        the keys
#[test]
fn expect_collect() {
    const TEST_DATA: &'static str = r#"
        p0 = bool
        l0 = 3
        s0 = {address: tstr .size 16, port: uint .size 2}
		"#;
    // Create input to filter.
    let input = seedle_parser::parse(TEST_DATA).unwrap().into_iter().fold(
        Object::new(),
        |mut state, (key, val)| {
            state.insert(key.into(), val.into());
            state
        },
    );
    let expect_literals = Value::Object(liquid_core::object!({
        "l0": Value::from(LinkedNode::Literal(Literal::UInt(3)))
    }));
    let expect_primatives = Value::Object(liquid_core::object!({
        "p0": Value::from(LinkedNode::Primative(ConstrainedPrimative::Bool))
    }));
    let expect_structs = Value::Object(liquid_core::object!({
        "s0": Value::from(LinkedNode::Struct(Fields {members: vec![
              LinkedKeyVal::new("address", ConstrainedPrimative::Str(16).into()),
              LinkedKeyVal::new("port", ConstrainedPrimative::U16.into()),
        ],
    }))}));
    assert_eq!(
        expect_literals,
        liquid_core::call_filter!(Collect, input, "literal").unwrap()
    );
    assert_eq!(
        expect_primatives,
        liquid_core::call_filter!(Collect, input, "primative").unwrap()
    );
    assert_eq!(
        expect_structs,
        liquid_core::call_filter!(Collect, input, "struct").unwrap()
    );
}
