use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use liquid_core::model::ScalarCow;
use liquid_core::{Error, Value, ValueView};

#[derive(Debug)]
pub enum Language {
    C,
    Typescript,
    Rust,
}

impl Language {
    pub fn structify(&self, name: &str) -> String {
        match self {
            Language::C => name.to_snake_case(),
            _ => name.to_upper_camel_case(),
        }
    }

    pub fn fieldify(&self, name: &str) -> String {
        match self {
            Language::C => name.to_snake_case(),
            Language::Rust => name.to_snake_case(),
            Language::Typescript => name.to_lower_camel_case(),
        }
    }
}

impl From<Language> for &'static str {
    fn from(value: Language) -> Self {
        match value {
            Language::C => "c",
            Language::Rust => "rust",
            Language::Typescript => "typescript",
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "c" => Ok(Language::C),
            "C" => Ok(Language::C),
            "rust" => Ok(Language::Rust),
            "Rust" => Ok(Language::Rust),
            "RUST" => Ok(Language::Rust),
            "typescript" => Ok(Language::Typescript),
            "Typescript" => Ok(Language::Typescript),
            "TYPESCRIPT" => Ok(Language::Typescript),
            s => Err(Error::with_msg(format!("invalid language {}", s))),
        }
    }
}

impl From<Language> for ScalarCow<'static> {
    fn from(value: Language) -> Self {
        let s: &'static str = value.into();
        ScalarCow::from(s)
    }
}

impl From<Language> for Value {
    fn from(value: Language) -> Self {
        Value::Scalar(value.into())
    }
}

impl TryFrom<Value> for Language {
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Language::try_from(value.to_kstr().as_str())
    }
}
