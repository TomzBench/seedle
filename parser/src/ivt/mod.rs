#[cfg(test)]
mod tests;

mod flatten;
mod literal;
mod primative;
use crate::error;
use cddl_cat::{self, ast};
use error::FlattenError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub(crate) type FlattenResult<T> = std::result::Result<T, FlattenError>;
pub(crate) use flatten::flatten;
pub(crate) use literal::Literal;
pub(crate) use primative::ConstrainedPrimative;
pub(crate) use primative::Primative;

#[derive(Debug, PartialEq)]
pub struct KeyVal(pub(crate) String, pub(crate) Box<Node>);
impl KeyVal {
    pub fn new<'a, K: Into<Cow<'a, str>>>(key: K, node: Node) -> KeyVal {
        KeyVal(key.into().into(), Box::new(node))
    }
}

#[derive(Debug, PartialEq)]
pub struct Array {
    pub len: usize,
    pub ty: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Group {
    pub members: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    /// A Literal type such as "true" or 3 or "hello"
    Literal(Literal),
    /// IE: uint .size 1 ; a u8
    Primative(ConstrainedPrimative),
    /// A CDDL array defined using square brackets [ ]
    /// IE: [ 3*3 u8 ] ; [u8, u8, u8]
    Array(Array),
    /// A CDDL group defined using braces ( ) and intended used for composing larger types
    /// IE: network-group = (address tstr .size 16, port: uint .size 2)
    Group(Group),
    /// A CDDL map defined using curly braces { }
    /// IE: network = { network-group }
    Map(Group),
    /// A single key: value item
    /// IE: foo: int .size 2
    KeyVal(KeyVal),
    /// An unresovoved primative expects to be resolved via second pass when creating a LinkedNode
    /// String is a key to a Node::Foreign (or will error)
    Foreign(String),
}

impl From<ConstrainedPrimative> for Node {
    fn from(ty: ConstrainedPrimative) -> Node {
        Node::Primative(ty)
    }
}

impl From<Literal> for Node {
    fn from(value: Literal) -> Self {
        Node::Literal(value)
    }
}

impl From<KeyVal> for Node {
    fn from(kv: KeyVal) -> Node {
        Node::KeyVal(kv)
    }
}
