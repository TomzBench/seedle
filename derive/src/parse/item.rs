use syn::parse::{Parse, ParseStream, Result};
use syn::token::Brace;
use syn::{Ident, Token};

pub struct Mod {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Option<Token![pub]>,
    pub ident: Ident,
    pub rest: (Brace, Vec<syn::Item>),
}

impl Parse for Mod {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = syn::Attribute::parse_outer(input)?;
        let vis = input.parse::<Token![pub]>().ok();
        let _ = input.parse::<Token![mod]>()?;
        let ident = input.parse()?;
        let inner;
        let brace = syn::braced!(inner in input);
        let mut items = Vec::new();
        while !inner.is_empty() {
            items.push(inner.parse()?);
        }
        Ok(Mod {
            attrs,
            vis,
            ident,
            rest: (brace, items),
        })
    }
}
