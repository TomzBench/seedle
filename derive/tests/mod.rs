#[test]
fn test_enum(){
    let t = trybuild::TestCases::new();
    t.pass("examples/vtable.rs");
}
