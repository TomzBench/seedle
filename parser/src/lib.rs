#[cfg(test)]
mod tests;

mod error;
mod flatten;
mod link;
mod node;
pub use error::{FlattenError, FlattenResult};

use std::collections::BTreeMap;
pub fn parse(cddl: &str) -> FlattenResult<BTreeMap<String, node::LinkedNode>> {
    flatten::flatten(cddl).and_then(link::link)
}
