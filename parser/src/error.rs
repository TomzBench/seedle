use cddl_cat::parser::ParseError;
use std::{error, fmt};

pub type FlattenResult<T> = std::result::Result<T, FlattenError>;

#[derive(Debug)]
pub enum FlattenError {
    Parser(ParseError),
    InvalidEnum0,
    InvalidUnconstrainedPrimative,
    InvalidLiteral,
    InvalidControl,
    InvalidControlArg,
    InvalidGroupMissingKey,
    InvalidType,
    InvalidSizeConstraint(String, u64),
    InvalidArray,
    InvalidArraySize,
    TodoEnums,
    NotSupportedRange,
    NotSupportedChoice,
    NotSupportedGenerics,
    NotSupportedControl(String),
    NotSupportedGroupname(String),
    ForeignKey(String),
    Infallible,
}

impl fmt::Display for FlattenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FlattenError::*;
        match self {
            Parser(e) => e.fmt(f),
            InvalidEnum0 => write!(f, "Enum type with 0 members unsupported"),
            InvalidUnconstrainedPrimative => write!(f, "type must be constrained"),
            InvalidLiteral => write!(f, "invalid literal"),
            InvalidControl => write!(f, "size control only supported on primative types"),
            InvalidControlArg => write!(f, "only integers supported for control args"),
            InvalidGroupMissingKey => write!(f, "all group members must have a key"),
            InvalidType => write!(f, "invalid type"),
            InvalidSizeConstraint(p, s) => write!(f, "invalid size constraint {} for {}", s, p),
            InvalidArray => write!(f, "invalid array"),
            InvalidArraySize => write!(f, "invalid array size"),
            TodoEnums => write!(f, "enums not supported"),
            NotSupportedRange => write!(f, "ranges are not supported"),
            NotSupportedChoice => write!(f, "choices are not supported"),
            NotSupportedGenerics => write!(f, "generics are not supported"),
            NotSupportedControl(ctrl) => write!(f, "control [{}] not supported", ctrl),
            NotSupportedGroupname(name) => write!(f, "group names not supported, found [{}]", name),
            ForeignKey(key) => write!(f, "foreign key not defined [{}]", key),
            Infallible => write!(f, "infallible"),
        }
    }
}

impl From<ParseError> for FlattenError {
    fn from(value: ParseError) -> Self {
        Self::Parser(value)
    }
}

impl error::Error for FlattenError {}
