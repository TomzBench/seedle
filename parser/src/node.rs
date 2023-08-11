use super::error::*;
use std::borrow::Cow;

pub enum Primative {
    /// The CDDL primative type uint (an unsigned integer)
    UInt,
    /// The CDDL primative type int (a signed integer)
    Int,
    /// The CDDL primative byte string
    BStr,
    /// The CDDL primative "Text" string
    TStr,
    /// The CDDL primative "bool" type
    Bool,
    /// A CDDL type defined in another rule further in the ruleset
    Unresolved(String),
}

impl Primative {
    pub fn constrain(self, size: u64) -> FlattenResult<ConstrainedPrimative> {
        match (self, size) {
            (Primative::Int, 1) => Ok(ConstrainedPrimative::I8),
            (Primative::Int, 2) => Ok(ConstrainedPrimative::I16),
            (Primative::Int, 4) => Ok(ConstrainedPrimative::I32),
            (Primative::Int, 8) => Ok(ConstrainedPrimative::I64),
            (Primative::UInt, 1) => Ok(ConstrainedPrimative::U8),
            (Primative::UInt, 2) => Ok(ConstrainedPrimative::U16),
            (Primative::UInt, 4) => Ok(ConstrainedPrimative::U32),
            (Primative::UInt, 8) => Ok(ConstrainedPrimative::U64),
            (Primative::TStr, n) => Ok(ConstrainedPrimative::Str(n)),
            (Primative::BStr, n) => Ok(ConstrainedPrimative::Bytes(n)),
            (prim, size) => Err(FlattenError::InvalidSizeConstraint(prim.into(), size)),
        }
    }
}

impl From<String> for Primative {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "int" => Primative::Int,
            "uint" => Primative::UInt,
            "tstr" | "text" => Primative::TStr,
            "bstr" | "bytes" => Primative::BStr,
            "bool" | "boolean" => Primative::Bool,
            _ => Primative::Unresolved(value),
        }
    }
}

impl From<Primative> for String {
    fn from(value: Primative) -> Self {
        match value {
            Primative::Int => "int".to_string(),
            Primative::UInt => "uint".to_string(),
            Primative::TStr => "tstr".to_string(),
            Primative::BStr => "bstr".to_string(),
            Primative::Bool => "bool".to_string(),
            Primative::Unresolved(s) => s,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstrainedPrimative {
    /// uint .size 1
    U8,
    /// int .size 1
    I8,
    /// uint .size 2
    U16,
    /// int .size 2
    I16,
    /// uint .size 4
    U32,
    /// int .size 4
    I32,
    /// uint .size 8
    U64,
    /// int .size 8
    I64,
    /// bool
    Bool,
    /// A tstr of N size
    Str(u64),
    /// A byte array of N size
    Bytes(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// A CDDL Literal Int
    Int(i64),
    /// A CDDL Literal UInt
    UInt(u64),
    /// A CDDL literal bool, AKA false
    Bool(bool),
    /// A CDDL literal string, AKA "Site"
    Str(String),
    /// A CDDL literal char, AKA 'G'
    Char(char),
    /// A CDDL literal byte array AKA [3,2,1]
    Bytes(Vec<u8>),
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        if value.len() == 1 {
            value
                .chars()
                .next()
                .map(Literal::Char)
                .unwrap_or_else(|| Literal::Str(value))
        } else {
            Literal::Str(value)
        }
    }
}

macro_rules! from_ty {
    ($enum:ident, $ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::$enum(value)
            }
        }
    };
}

macro_rules! from_int {
    ($ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::Int(value.into())
            }
        }
    };
}

macro_rules! from_uint {
    ($ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::UInt(value.into())
            }
        }
    };
}
from_ty!(Bool, bool);
from_ty!(Char, char);
from_ty!(Bytes, Vec<u8>);
from_ty!(Int, i64);
from_ty!(UInt, u64);
from_int!(i32);
from_int!(i16);
from_int!(i8);
from_uint!(u32);
from_uint!(u16);
from_uint!(u8);

#[derive(Clone, Debug, PartialEq)]
pub struct KeyVal(pub(crate) String, pub(crate) Box<Node>);
impl KeyVal {
    pub fn new<'a, K: Into<Cow<'a, str>>>(key: K, node: Node) -> KeyVal {
        KeyVal(key.into().into(), Box::new(node))
    }
}
impl From<(&str, Node)> for KeyVal {
    fn from((key, node): (&str, Node)) -> Self {
        KeyVal::new(key, node)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub len: usize,
    pub ty: Box<Node>,
}

impl Array {
    pub fn new(node: Node, len: usize) -> Array {
        Array {
            ty: Box::new(node),
            len,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub members: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq)]
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

impl From<Array> for Node {
    fn from(value: Array) -> Node {
        Node::Array(value)
    }
}

/// Similar to a Group, but fully resolved with fields
#[derive(Debug, Clone, PartialEq)]
pub struct Fields {
    /// The Field members of a struct
    pub members: Vec<LinkedKeyVal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkedKeyVal(pub(crate) String, pub(crate) LinkedNode);
impl LinkedKeyVal {
    pub fn new<'a, K: Into<Cow<'a, str>>>(key: K, node: LinkedNode) -> LinkedKeyVal {
        LinkedKeyVal(key.into().into(), node)
    }
}

/// Helper when creating Maps from Key/Value tuples.
impl From<(String, LinkedNode)> for LinkedKeyVal {
    fn from(t: (String, LinkedNode)) -> LinkedKeyVal {
        LinkedKeyVal(t.0, t.1)
    }
}

/// A linked array, similiar to ivt::Array, except with a LinkedNode
/// NOTE LinkedKeyVal and LinkedArray could share same definition with
///      ivt::KeyVal and ivt::Array using generics however getting impls to play
///      nice with serde was over my head
#[derive(Clone, Debug, PartialEq)]
pub struct LinkedArray {
    pub len: usize,
    pub ty: Box<LinkedNode>,
}
impl LinkedArray {
    pub fn new(node: LinkedNode, len: usize) -> LinkedArray {
        LinkedArray {
            ty: Box::new(node),
            len,
        }
    }
}

/// When we have an IVT node, we lookup unresolved types and build a complete tree
#[derive(Clone, Debug, PartialEq)]
pub enum LinkedNode {
    /// A Literal type such as "true" or 3 or "hello"
    Literal(Literal),
    /// A primative type fully qualified
    Primative(ConstrainedPrimative),
    /// An array is of a fixed size of a single type
    Array(LinkedArray),
    /// A group of fields missing context (might be a struct)
    Fields(Fields),
    /// A fully qualified struct with fields (Can only exist at top level)
    Struct(Fields),
    /// If a struct contains a nested struct, we store flatten instead of nest
    ForeignStruct(String),
}

impl From<ConstrainedPrimative> for LinkedNode {
    fn from(value: ConstrainedPrimative) -> Self {
        LinkedNode::Primative(value)
    }
}

impl From<LinkedArray> for LinkedNode {
    fn from(value: LinkedArray) -> Self {
        LinkedNode::Array(value)
    }
}
