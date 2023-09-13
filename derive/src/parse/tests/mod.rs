use super::item::*;
use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::matches;
use syn::Ident;
fn mock_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
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

    let item: Mod = syn::parse2(tokens).unwrap();
    assert!(matches!(item, Mod{ident,..} if ident == "foo"));
}
