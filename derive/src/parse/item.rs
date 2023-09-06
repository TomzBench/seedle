use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{Ident, Token};

pub struct Mod {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Option<Token![pub]>,
    pub ident: Ident,
    pub rest: (Brace, Vec<syn::Item>),
}

pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Option<Token![pub]>,
    pub ident: Ident,
    pub fields: syn::FieldsNamed,
}

pub struct Enum {
    pub ident: Ident,
    pub items: Punctuated<Ident, Token![,]>,
}
pub enum Item {
    Struct(Struct),
    Enum(Enum),
    Mod(Mod),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = syn::Attribute::parse_outer(input)?;
        let vis = input.parse::<Token![pub]>().ok();
        if input.peek(Token![struct]) {
            let _ = input.parse::<Token![struct]>()?;
            let ident = input.parse()?;
            let fields = input.parse::<syn::FieldsNamed>()?;
            Ok(Item::Struct(Struct {
                attrs,
                vis,
                ident,
                fields,
            }))
        } else if input.peek(Token![enum]) {
            let _ = input.parse::<Token![enum]>()?;
            let ident = input.parse()?;
            let inner;
            let _brace = syn::braced!(inner in input);
            Ok(Item::Enum(Enum {
                ident,
                items: inner.parse_terminated(Ident::parse, Token![,])?,
            }))
        } else if input.peek(Token![mod]) {
            let _ = input.parse::<Token![mod]>()?;
            let ident = input.parse()?;
            let inner;
            let brace = syn::braced!(inner in input);
            let mut items = Vec::new();
            while !inner.is_empty() {
                items.push(inner.parse()?);
            }
            Ok(Item::Mod(Mod {
                attrs,
                vis,
                ident,
                rest: (brace, items),
            }))
        } else {
            Err(Error::new(input.span(), "Expected struct or enum"))
        }
    }
}
