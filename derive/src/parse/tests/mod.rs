use super::item::*;
use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::matches;
use syn::Ident;
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
        Item::Mod(_) => panic!("impossible"),
        Item::Enum(Enum { items: n, .. }) => {
            let inner = n
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            assert_eq!(inner, vec!["FOO", "BAR"]);
        }
    }
}

#[test]
fn item_parse_mod() {
    let tokens = TokenStream::from_iter(vec![
        TokenTree::Ident(mock_ident("pub")),
        TokenTree::Ident(mock_ident("mod")),
        TokenTree::Ident(mock_ident("foo")),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter(vec![
                TokenTree::Ident(mock_ident("pub")),
                TokenTree::Ident(mock_ident("const")),
                TokenTree::Ident(mock_ident("FOO")),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(mock_ident("u32")),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Literal(Literal::u32_unsuffixed(42)),
                TokenTree::Punct(Punct::new(';', Spacing::Alone)),
            ]),
        )),
    ]);

    let item: Item = syn::parse2(tokens).unwrap();
    assert!(matches!(item, Item::Mod(Mod{ident,..}) if ident == "foo"));
}
