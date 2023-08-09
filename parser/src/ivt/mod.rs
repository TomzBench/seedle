#[cfg(test)]
mod tests;

mod error;
mod flatten;
mod link;
mod node;

pub(crate) use error::{FlattenError, FlattenResult};
pub(crate) use node::{
    Array, ConstrainedPrimative, Fields, Group, KeyVal, LinkedArray, LinkedKeyVal, LinkedNode,
    Literal, Node, Primative,
};
