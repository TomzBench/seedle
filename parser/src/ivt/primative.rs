use crate::error::*;
use serde::{Deserialize, Serialize};

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
            s => Primative::Unresolved(value),
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
