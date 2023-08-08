use crate::error;
use cddl_cat::{self, ast};
use error::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use super::{Array, ConstrainedPrimative, Group, KeyVal, Literal, Node, Primative};

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
        Value::Text(v) => Ok(Literal::from(v).into()),
        Value::Nint(v) => Ok(Literal::from(v).into()),
        Value::Uint(v) => Ok(Literal::from(v).into()),
        Value::Bytes(v) => Ok(Literal::from(v).into()),
        _ => Err(FlattenError::InvalidLiteral),
    }
}

/// If we flatten a type2 typename we must do so via a control statement. Otherwize we assume we
/// are an unresolved named type
fn flatten_typename(name: ast::NameGeneric) -> FlattenResult<Node> {
    match Primative::from(name.name) {
        Primative::Int | Primative::UInt | Primative::TStr | Primative::BStr => {
            Err(FlattenError::InvalidUnconstrainedPrimative)
        }
        Primative::Bool => Ok(Node::Primative(ConstrainedPrimative::Bool)),
        Primative::Unresolved(s) => match s.as_str() {
            "false" => Ok(Node::Literal(Literal::Bool(false))),
            "true" => Ok(Node::Literal(Literal::Bool(false))),
            _ => Ok(Node::Foreign(s)),
        },
    }
}

fn flatten_control(ctl: ast::TypeControl) -> FlattenResult<Node> {
    use ast::{Type2, Value};
    let size = match ctl.arg {
        Type2::Value(Value::Uint(n)) => Ok(n),
        _ => Err(FlattenError::InvalidControl),
    }?;
    match ctl.op.as_str() {
        "size" => match ctl.target {
            Type2::Typename(s) => Primative::from(s.name).constrain(size).map(Node::Primative),
            _ => Err(FlattenError::InvalidControl),
        },
        ctrl => Err(FlattenError::NotSupportedControl(ctl.op)),
    }
}

fn flatten_map(group: ast::Group) -> FlattenResult<Node> {
    flatten_group(group).map(|members| Node::Map(Group { members }))
}

fn flatten_array(group: ast::Group) -> FlattenResult<Node> {
    use ast::Occur;
    get_group_entries(group).and_then(|mut entries| {
        if entries.len() == 1 {
            let entry = entries.pop().ok_or(FlattenError::Infallible)?;
            match entry.occur {
                Some(Occur::Numbered(a, len)) if a == len => Ok(Node::Array(Array {
                    len,
                    ty: Box::new(flatten_groupentry(entry)?),
                })),
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
