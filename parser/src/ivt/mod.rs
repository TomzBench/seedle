#[cfg(test)]
mod tests;

mod error;
mod flatten;
mod link;
mod node;

pub(crate) use error::{FlattenError, FlattenResult};
pub(crate) use flatten::flatten;
pub(crate) use node::{Array, ConstrainedPrimative, Group, KeyVal, Literal, Node, Primative};
