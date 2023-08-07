use crate::error;
use cddl_cat::{self, ast};
use error::FlattenError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub type FlattenResult<T> = std::result::Result<T, FlattenError>;

/// TODO Should be an enum of type Resolved or unresolved
enum PrimativeType {
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConstrainedType {
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

#[derive(Debug, PartialEq)]
pub struct KeyVal(pub(crate) String, pub(crate) Box<Node>);
impl KeyVal {
    pub fn new<'a, K: Into<Cow<'a, str>>>(key: K, node: Node) -> KeyVal {
        KeyVal(key.into().into(), Box::new(node))
    }
}

impl From<KeyVal> for Node {
    fn from(kv: KeyVal) -> Node {
        Node::KeyVal(kv)
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
    ConstrainedType(ConstrainedType),
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

impl From<ConstrainedType> for Node {
    fn from(ty: ConstrainedType) -> Node {
        Node::ConstrainedType(ty)
    }
}

pub(crate) fn flatten(cddl: &str) -> FlattenResult<BTreeMap<String, Node>> {
    let ast = cddl_cat::parse_cddl(cddl).map_err(FlattenError::from)?;
    ast.rules.into_iter().map(flatten_rule).collect()
}

fn flatten_rule(rule: ast::Rule) -> FlattenResult<(String, Node)> {
    let node = match rule.val {
        ast::RuleVal::AssignType(t) => flatten_type(t)?,
        ast::RuleVal::AssignGroup(g) => flatten_groupentry(g)?,
    };
    Ok((rule.name, node))
}

fn flatten_type(ty: ast::Type) -> FlattenResult<Node> {
    let choices =
        ty.0.into_iter()
            .map(flatten_type1)
            .collect::<FlattenResult<Vec<Node>>>()?;
    match choices.len() {
        0 => Err(FlattenError::InvalidEnum0),
        1 => Ok(choices.into_iter().next().unwrap()),
        _ => Err(FlattenError::TodoEnums),
    }
}

fn flatten_type1(ty1: ast::Type1) -> FlattenResult<Node> {
    match ty1 {
        ast::Type1::Simple(ty2) => flatten_type2(ty2),
        ast::Type1::Range(_) => Err(FlattenError::NotSupportedRange),
        ast::Type1::Control(ctrl) => flatten_control(ctrl),
    }
}

fn flatten_type2(ty2: ast::Type2) -> FlattenResult<Node> {
    use ast::Type2;
    match ty2 {
        Type2::Value(v) => flatten_value(v),
        Type2::Typename(t) => flatten_typename(t),
        Type2::Parethesized(t) => flatten_type(t),
        Type2::Map(g) => flatten_map(g),
        Type2::Array(g) => flatten_array(g),
        Type2::Unwrap(_) => Err(FlattenError::NotSupportedGenerics),
        Type2::Choiceify(_) | Type2::ChoiceifyInline(_) => Err(FlattenError::NotSupportedChoice),
    }
}

// TODO flatten values into a Literal type instead of a constrained type
fn flatten_value(val: ast::Value) -> FlattenResult<Node> {
    use ast::Value;
    match val {
        Value::Text(s) => Ok(Node::Literal(flatten_literal_text(s)?)),
        Value::Nint(i) => Ok(Node::Literal(Literal::Int(i))),
        Value::Uint(i) => Ok(Node::Literal(Literal::UInt(i))),
        Value::Bytes(b) => Ok(Node::Literal(Literal::Bytes(b))),
        _ => Err(FlattenError::InvalidLiteral),
    }
}

fn flatten_literal_text(val: String) -> FlattenResult<Literal> {
    use FlattenError::*;
    if val.len() == 1 {
        Ok(Literal::Char(val.chars().next().ok_or(Infallible)?))
    } else {
        Ok(Literal::Str(val.to_string()))
    }
}

/// If we flatten a type2 typename we must do so via a control statement. Otherwize we assume we
/// are an unresolved named type
fn flatten_typename(name: ast::NameGeneric) -> FlattenResult<Node> {
    match flatten_primative(name.name) {
        PrimativeType::Int | PrimativeType::UInt | PrimativeType::TStr | PrimativeType::BStr => {
            Err(FlattenError::InvalidUnconstrainedPrimative)
        }
        PrimativeType::Bool => Ok(Node::ConstrainedType(ConstrainedType::Bool)),
        PrimativeType::Unresolved(s) => Ok(Node::Foreign(s)),
    }
}

/// A first pass when resolving a primative might refer to a type defined further
/// in the rule set. Therefore we may return an enum which resolves the type or
/// must be resolved in the final stage of validation
fn flatten_primative(prim: String) -> PrimativeType {
    match prim.as_ref() {
        "int" => PrimativeType::Int,
        "uint" => PrimativeType::UInt,
        "tstr" | "text" => PrimativeType::TStr,
        "bstr" | "bytes" => PrimativeType::BStr,
        "bool" | "boolean" => PrimativeType::Bool,
        s => PrimativeType::Unresolved(s.into()),
    }
}

fn flatten_control(ctl: ast::TypeControl) -> FlattenResult<Node> {
    match ctl.op.as_str() {
        "size" => flatten_control_size(ctl),
        ctrl => Err(FlattenError::NotSupportedControl(ctl.op)),
    }
}

/// Take a control type, and turn it into a constrained type
fn control_to_constrained_type(ctrl: ast::TypeControl) -> FlattenResult<ConstrainedType> {
    use ast::{Type2, Value};
    if let Type2::Typename(s) = ctrl.target {
        match (flatten_primative(s.name), &ctrl.arg) {
            (PrimativeType::Int, Type2::Value(Value::Uint(1))) => Ok(ConstrainedType::I8),
            (PrimativeType::Int, Type2::Value(Value::Uint(2))) => Ok(ConstrainedType::I16),
            (PrimativeType::Int, Type2::Value(Value::Uint(4))) => Ok(ConstrainedType::I32),
            (PrimativeType::Int, Type2::Value(Value::Uint(8))) => Ok(ConstrainedType::I64),
            (PrimativeType::UInt, Type2::Value(Value::Uint(1))) => Ok(ConstrainedType::U8),
            (PrimativeType::UInt, Type2::Value(Value::Uint(2))) => Ok(ConstrainedType::U16),
            (PrimativeType::UInt, Type2::Value(Value::Uint(4))) => Ok(ConstrainedType::U32),
            (PrimativeType::UInt, Type2::Value(Value::Uint(8))) => Ok(ConstrainedType::U64),
            (PrimativeType::TStr, Type2::Value(Value::Uint(n))) => Ok(ConstrainedType::Str(*n)),
            (PrimativeType::BStr, Type2::Value(Value::Uint(n))) => Ok(ConstrainedType::Bytes(*n)),
            _ => Err(FlattenError::InvalidControl),
        }
    } else {
        Err(FlattenError::InvalidControl)
    }
}

fn flatten_control_size(ctrl: ast::TypeControl) -> FlattenResult<Node> {
    control_to_constrained_type(ctrl).map(Node::ConstrainedType)
}

fn flatten_map(group: ast::Group) -> FlattenResult<Node> {
    flatten_group(group).map(|members| Node::Map(Group { members }))
}

fn flatten_array(group: ast::Group) -> FlattenResult<Node> {
    use ast::Occur;
    get_group_entries(group).and_then(|mut entries| {
        if entries.len() == 1 {
            let entry = entries.pop().ok_or(FlattenError::Infallible)?;
            let ty = Box::new(flatten_groupentry(entry)?);
            match entries[0].occur {
                Some(Occur::Numbered(a, len)) if a == len => Ok(Node::Array(Array { len, ty })),
                _ => Err(FlattenError::InvalidArraySize),
            }
        } else {
            Err(FlattenError::InvalidArray)
        }
    })
}

fn flatten_group(group: ast::Group) -> FlattenResult<Vec<Node>> {
    get_group_entries(group)?
        .into_iter()
        .map(flatten_groupentry)
        .collect()
}

// We don't support "choices" or "enums", therefore we assume GrpChoice==1
fn get_group_entries(mut group: ast::Group) -> FlattenResult<Vec<ast::GrpEnt>> {
    if group.0.len() == 1 {
        let groups = group.0.pop().ok_or(FlattenError::Infallible)?;
        Ok(groups.0)
    } else {
        Err(FlattenError::InvalidEnum0)
    }
}

fn flatten_groupentry(group_entry: ast::GrpEnt) -> FlattenResult<Node> {
    use ast::GrpEntVal;
    match group_entry.val {
        GrpEntVal::Member(m) => flatten_group_member(m),
        GrpEntVal::Parenthesized(g) => {
            flatten_group(g).map(|members| Node::Group(Group { members }))
        }
        GrpEntVal::Groupname(name) => Err(FlattenError::NotSupportedGroupname(name)),
    }
}

fn flatten_group_member(member: ast::Member) -> FlattenResult<Node> {
    use ast::MemberKeyVal;
    match &member.key {
        Some(key) => match &key.val {
            MemberKeyVal::Bareword(s) => {
                Ok(Node::KeyVal(KeyVal::new(s, flatten_type(member.value)?)))
            }
            _ => Err(FlattenError::InvalidGroupMissingKey),
        },
        None => assume_foreign_value(member.value),
    }
}

fn assume_foreign_value(ty: ast::Type) -> FlattenResult<Node> {
    match flatten_type(ty) {
        Ok(Node::Foreign(s)) => Ok(Node::Foreign(s)),
        _ => Err(FlattenError::InvalidType),
    }
}
