use seedle_derive::seedle;

#[seedle(prefix = "cbor", language = "c")]
pub enum Keys {
    Foo,
    //FooFi,
    //Bar,
    //BarBop,
    //Fee,
}

#[seedle(prefix = "cbor", language = "c")]
pub struct Foo {
    #[n(0)]
    _a: u8,
}
// #[seedle(prefix = "cbor")]
// pub struct FooFi {
//     a: u8,
// }
// #[seedle(prefix = "cbor")]
// pub struct Bar {
//     a: u8,
// }
// #[seedle(prefix = "cbor")]
// pub struct BarBop {
//     a: u8,
// }
// #[seedle(prefix = "cbor")]
// pub struct Fee {
//     a: u8,
// }

fn main() {}
