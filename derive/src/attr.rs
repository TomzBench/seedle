use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Token;

struct Attr {
    pub ident: syn::Ident,
    pub _eq: syn::Token![=],
    pub val: syn::LitStr,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Attr {
            ident: input.parse()?,
            _eq: input.parse()?,
            val: input.parse()?,
        })
    }
}

pub enum Language {
    C,
    Rust,
    Typescript,
}
impl Default for Language {
    fn default() -> Self {
        Language::C
    }
}

impl From<syn::LitStr> for Language {
    fn from(value: syn::LitStr) -> Self {
        match value.value().as_str() {
            "ts" | "typescript" | "TYPESCRIPT" => Language::Typescript,
            "rs" | "rust" | "RUST" => Language::Rust,
            "c" | "C" => Language::C,
            _ => Language::default(),
        }
    }
}

#[derive(Default)]
pub struct Attributes {
    prefix: Option<syn::LitStr>,
    language: Language,
}
pub fn parse(i: ParseStream) -> Result<Attributes> {
    let p = Punctuated::<Attr, Token![,]>::parse_terminated_with(i, Attr::parse)?;
    let attrs = p
        .into_iter()
        .fold(Attributes::default(), |mut state, attr| {
            if attr.ident == "prefix" {
                state.prefix = Some(attr.val);
            } else if attr.ident == "language" {
                state.language = attr.val.into()
            }
            state
        });
    Ok(attrs)
}
