mod c;
mod ts;
#[test]
fn test_derive(){
    let t = trybuild::TestCases::new();
    t.pass("examples/c.rs");
    t.pass("examples/ts.rs");
}
