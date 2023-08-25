use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub tok_vis: Option<Token![pub]>,
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
}

pub fn parse(i: ParseStream) -> Result<Item> {
    Item::parse(i)
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = syn::Attribute::parse_outer(input)?;
        let tok_vis = input.parse::<Token![pub]>().ok();
        if input.peek(Token![struct]) {
            let _ = input.parse::<Token![struct]>()?;
            let ident = input.parse()?;
            let fields = input.parse::<syn::FieldsNamed>()?;
            Ok(Item::Struct(Struct {
                attrs,
                tok_vis,
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
        } else {
            Err(Error::new(input.span(), "Expected struct or enum"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};
    use std::matches;
    fn mock_ident(name: &str) -> Ident {
        Ident::new(name, Span::call_site())
    }
    #[test]
    fn item_parse_struct() {
        let tokens = TokenStream::from_iter(vec![
            TokenTree::Ident(mock_ident("pub")),
            TokenTree::Ident(mock_ident("struct")),
            TokenTree::Ident(mock_ident("Thing")),
            TokenTree::Group(Group::new(
                Delimiter::Brace,
                TokenStream::from_iter(vec![
                    TokenTree::Ident(mock_ident("Foo")),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(mock_ident("u8")),
                ]),
            )),
        ]);
        let item: Item = syn::parse2(tokens).unwrap();
        assert!(matches!(item, Item::Struct(Struct{ident,..}) if ident == "Thing"));
    }

    #[test]
    fn item_parse_enum() {
        let tokens = TokenStream::from_iter(vec![
            TokenTree::Ident(mock_ident("pub")),
            TokenTree::Ident(mock_ident("enum")),
            TokenTree::Ident(mock_ident("Thing")),
            TokenTree::Group(Group::new(
                Delimiter::Brace,
                TokenStream::from_iter(vec![
                    TokenTree::Ident(mock_ident("FOO")),
                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                    TokenTree::Ident(mock_ident("BAR")),
                ]),
            )),
        ]);
        let item: Item = syn::parse2(tokens).unwrap();
        assert!(matches!(&item, Item::Enum(Enum{ident,..}) if ident == "Thing"));
        match item {
            Item::Struct(_) => panic!("impossible"),
            Item::Enum(Enum { items: n, .. }) => {
                let inner = n
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                assert_eq!(inner, vec!["FOO", "BAR"]);
            }
        }
    }
}
