use serde::{Deserialize, Serialize};

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

impl From<String> for Literal {
    fn from(value: String) -> Self {
        if value.len() == 1 {
            value
                .chars()
                .next()
                .map(Literal::Char)
                .unwrap_or_else(|| Literal::Str(value))
        } else {
            Literal::Str(value)
        }
    }
}

macro_rules! from_ty {
    ($enum:ident, $ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::$enum(value)
            }
        }
    };
}

macro_rules! from_int {
    ($ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::Int(value.into())
            }
        }
    };
}

macro_rules! from_uint {
    ($ty:ty) => {
        impl From<$ty> for Literal {
            fn from(value: $ty) -> Self {
                Literal::UInt(value.into())
            }
        }
    };
}
from_ty!(Bool, bool);
from_ty!(Char, char);
from_ty!(Bytes, Vec<u8>);
from_ty!(Int, i64);
from_ty!(UInt, u64);
from_int!(i32);
from_int!(i16);
from_int!(i8);
from_uint!(u32);
from_uint!(u16);
from_uint!(u8);
