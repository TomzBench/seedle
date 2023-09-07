use crate::node::{Fields, LinkedNode, Literal};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::Hash;
pub struct Context(BTreeMap<String, LinkedNode>);

pub type Grouped<'a, T> = BTreeMap<String, Vec<(String, &'a T)>>;

pub fn structs<K: Hash>(kv: (K, LinkedNode)) -> Option<(K, Fields)> {
    filter_structs((kv.0, Cow::Owned(kv.1))).map(|(k, v)| (k, v.into_owned()))
}

pub fn borrowed_structs<'a, K: Hash>(kv: (K, &'a LinkedNode)) -> Option<(K, Cow<'a, Fields>)> {
    filter_structs((kv.0, Cow::Borrowed(kv.1)))
}

pub fn literals<K: Hash>(kv: (K, LinkedNode)) -> Option<(K, Literal)> {
    filter_literals((kv.0, Cow::Owned(kv.1))).map(|(k, v)| (k, v.into_owned()))
}

#[inline]
fn filter_structs<'a, K, V>((key, cow): (K, V)) -> Option<(K, Cow<'a, Fields>)>
where
    K: Hash,
    V: Into<Cow<'a, LinkedNode>>,
{
    let val = cow.into();
    match val {
        Cow::Borrowed(LinkedNode::Struct(s)) => Some((key, Cow::Borrowed(s))),
        Cow::Owned(LinkedNode::Struct(s)) => Some((key, Cow::Owned(s))),
        _ => None,
    }
}

#[inline]
fn filter_literals<'a, K, V>((key, cow): (K, V)) -> Option<(K, Cow<'a, Literal>)>
where
    K: Hash,
    V: Into<Cow<'a, LinkedNode>>,
{
    let val = cow.into();
    match val {
        Cow::Borrowed(LinkedNode::Literal(l)) => Some((key, Cow::Borrowed(l))),
        Cow::Owned(LinkedNode::Literal(l)) => Some((key, Cow::Owned(l))),
        _ => None,
    }
}

pub fn fold_group<'a, T>(
    split: &'a str,
) -> impl Fn(Grouped<'a, T>, (&'a String, &'a T)) -> Grouped<'a, T> {
    move |mut state, kv| {
        if let Some((group, rest)) = kv.0.split_once(split) {
            insert(&mut state, group, rest, kv.1);
        } else {
            insert(&mut state, split, &kv.0, kv.1);
        }
        state
    }
}

fn insert<'a, T>(obj: &mut Grouped<'a, T>, group: &str, key: &str, node: &'a T) {
    if let Some(inner) = obj.get_mut(group) {
        inner.push((key.to_string(), node));
    } else {
        let mut inner = Vec::new();
        inner.push((key.to_string(), node));
        obj.insert(group.to_string(), inner);
    }
}
