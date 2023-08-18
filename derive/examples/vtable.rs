use seedle_derive::seedle;

#[seedle]
pub enum Keys {
    Foo,
    //FooFi,
    //Bar,
    //BarBop,
    //Fee,
}

#[seedle]
#[allow(non_camel_case_types)]
pub struct foo {
    a: u8
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
