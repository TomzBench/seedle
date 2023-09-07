use crate::node::{Fields, LinkedNode, Literal};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::Hash;
pub struct Context(BTreeMap<String, LinkedNode>);

pub type GroupedBorrowed<'a, T> = BTreeMap<String, Vec<(String, &'a T)>>;
pub type GroupedCow<'a, T> = BTreeMap<String, Vec<(String, Cow<'a, T>)>>;
pub type Grouped<T> = BTreeMap<String, Vec<(String, T)>>;

pub fn structs_owned<K: Hash>(kv: (K, LinkedNode)) -> Option<(K, Fields)> {
    filter_structs((kv.0, Cow::Owned(kv.1))).map(|(k, v)| (k, v.into_owned()))
}

pub fn structs_borrowed<'a, K: Hash>(kv: (K, &'a LinkedNode)) -> Option<(K, Cow<'a, Fields>)> {
    filter_structs((kv.0, Cow::Borrowed(kv.1)))
}

pub fn literals_owned<K: Hash>(kv: (K, LinkedNode)) -> Option<(K, Literal)> {
    filter_literals((kv.0, Cow::Owned(kv.1))).map(|(k, v)| (k, v.into_owned()))
}

pub fn literals_borrowed<'a, K: Hash>(kv: (K, &'a LinkedNode)) -> Option<(K, Cow<'a, Literal>)> {
    filter_literals((kv.0, Cow::Borrowed(kv.1)))
}

pub fn fold_group_owned<'a, K, T>(
    split: &'a str,
) -> impl Fn(GroupedCow<'a, T>, (K, T)) -> GroupedCow<'a, T>
where
    K: Into<Cow<'a, str>>,
    T: Clone + 'a,
{
    move |state, kv| {
        let folder = fold_group(split);
        folder(state, (kv.0, Cow::Owned(kv.1)))
    }
}

pub fn fold_group_borrowed<'a, T>(
    split: &'a str,
) -> impl Fn(GroupedCow<'a, T>, (&'a String, Cow<'a, T>)) -> GroupedCow<'a, T>
where
    T: Clone + 'a,
{
    move |state, kv| {
        let folder = fold_group(split);
        folder(state, (kv.0, kv.1))
    }
}

#[inline]
fn fold_group<'a, K, T>(
    split: &'a str,
) -> impl Fn(GroupedCow<'a, T>, (K, Cow<'a, T>)) -> GroupedCow<'a, T>
where
    K: Into<Cow<'a, str>>,
    T: Clone,
{
    move |mut state, kv| {
        let key = kv.0.into();
        if let Some((group, rest)) = key.split_once(split) {
            insert(&mut state, group, rest, kv.1);
        } else {
            insert(&mut state, "_", &key, kv.1);
        }
        state
    }
}

#[inline]
fn insert<'a, T: Clone>(obj: &mut GroupedCow<'a, T>, group: &str, key: &str, node: Cow<'a, T>) {
    if let Some(inner) = obj.get_mut(group) {
        inner.push((key.to_string(), node));
    } else {
        let mut inner = Vec::new();
        inner.push((key.to_string(), node));
        obj.insert(group.to_string(), inner);
    }
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
