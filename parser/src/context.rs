use crate::node::{Fields, LinkedNode, Literal};
use std::collections::BTreeMap;
pub struct Context(BTreeMap<String, LinkedNode>);

impl Context {
    pub fn structs(&self) -> Vec<(&String, &Fields)> {
        self.0
            .iter()
            .filter_map(|(key, val)| match val {
                LinkedNode::Struct(s) => Some((key, s)),
                _ => None,
            })
            .collect()
    }

    pub fn literals(&self) -> Vec<(&String, &Literal)> {
        self.0
            .iter()
            .filter_map(|(key, val)| match val {
                LinkedNode::Literal(l) => Some((key, l)),
                _ => None,
            })
            .collect()
    }
}

impl From<BTreeMap<String, LinkedNode>> for Context {
    fn from(value: BTreeMap<String, LinkedNode>) -> Self {
        Self(value)
    }
}

impl From<Context> for BTreeMap<String, LinkedNode> {
    fn from(value: Context) -> Self {
        value.0
    }
}
