use super::{error::*, node::*};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Main (only) entry function to this module
pub(crate) fn link_node(node: Node, ctx: &BTreeMap<String, Node>) -> FlattenResult<LinkedNode> {
    match node {
        Node::Literal(lit) => Ok(LinkedNode::Literal(lit)),
        Node::Primative(t) => Ok(LinkedNode::Primative(t)),
        Node::Foreign(t) => link_foreign(t, ctx),
        Node::Group(g) => link_group(g, ctx),
        Node::Map(g) => link_struct(g, ctx),
        Node::Array(a) => link_array(a, ctx),
        Node::KeyVal(_) => Err(FlattenError::Infallible),
    }
}

fn link_array(arr: Array, ctx: &BTreeMap<String, Node>) -> FlattenResult<LinkedNode> {
    // Similar to link_foreign, we only accept certain types as an array, and we don't follow
    // nesting types so we can flatten them
    link_node(*arr.ty, ctx).and_then(|node| match node {
        // We don't accept nested arrays
        LinkedNode::Array(..) => Err(FlattenError::InvalidArray),
        // We don't accept inline fields inside an array
        LinkedNode::Fields(_) => Err(FlattenError::InvalidArray),
        // We don't accept inline structs defined inside an array
        LinkedNode::Struct(_) => Err(FlattenError::InvalidArray),
        // ConstainedType or Struct defined externally are the only acceptable array types
        n => Ok(LinkedNode::Array(LinkedArray {
            ty: Box::new(n),
            len: arr.len,
        })),
    })
}

fn link_foreign(key: String, ctx: &BTreeMap<String, Node>) -> FlattenResult<LinkedNode> {
    // When linking a "foreign" struct, we simply note it's remote name instead of
    // following the struct deeper.
    ctx.get(&key)
        .map(Node::clone)
        .ok_or_else(|| FlattenError::ForeignKey(key.clone()))
        .and_then(|node| match link_node(node, ctx)? {
            LinkedNode::Struct(_s) => Ok(LinkedNode::ForeignStruct(key)),
            node => Ok(node),
        })
}

fn link_group(map: Group, ctx: &BTreeMap<String, Node>) -> FlattenResult<LinkedNode> {
    link_field_key_values(map, ctx).map(|members| LinkedNode::Fields(Fields { members }))
}

fn link_struct(map: Group, ctx: &BTreeMap<String, Node>) -> FlattenResult<LinkedNode> {
    link_field_key_values(map, ctx).map(|members| LinkedNode::Struct(Fields { members }))
}

fn link_field_key_values(
    map: Group,
    ctx: &BTreeMap<String, Node>,
) -> FlattenResult<Vec<LinkedKeyVal>> {
    Ok(link_fields(map, ctx)?
        .into_iter()
        .map(LinkedKeyVal::from)
        .collect())
}

fn link_fields(
    map: Group,
    ctx: &BTreeMap<String, Node>,
) -> FlattenResult<Vec<(String, LinkedNode)>> {
    Ok(map
        .members
        .into_iter()
        .map(|node| link_field_member(node, ctx))
        .collect::<FlattenResult<Vec<Vec<(String, LinkedNode)>>>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn link_field_member(
    node: Node,
    ctx: &BTreeMap<String, Node>,
) -> FlattenResult<Vec<(String, LinkedNode)>> {
    match node {
        Node::KeyVal(KeyVal(k, v)) => link_node(*v, ctx).map(|n| vec![(k, n)]),
        Node::Foreign(key) => match ctx.get(&key).map(Node::clone) {
            Some(Node::Group(g)) => link_fields(g, ctx),
            //Some(Node::Map(g)) => link_struct(g, ctx).map(|n| vec![(key.clone(), n)]),
            _ => Err(FlattenError::InvalidType),
        },
        _ => Err(FlattenError::InvalidGroupMissingKey),
    }
}
