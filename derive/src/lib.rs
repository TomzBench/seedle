mod parse;
mod print;
use parse::item::Item;
use print::{cddl, ffi, vtable};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn seedle(attrs: TokenStream, toks: TokenStream) -> TokenStream {
    do_seedle(attrs, toks).map_or_else(
        |e| TokenStream::from(e.to_compile_error()),
        TokenStream::from,
    )
}

fn do_seedle(attrs: TokenStream, toks: TokenStream) -> syn::Result<TokenStream2> {
    let attrs: parse::Attributes = syn::parse(attrs)?;
    let item: parse::Item = syn::parse(toks)?;

    match item {
        Item::Struct(s) => Ok(ffi::build(s, attrs.language, attrs.prefix).into()),
        Item::Enum(e) => Ok(vtable::build(e, attrs.language, attrs.prefix).into()),
        Item::Mod(m) => cddl::build(m, attrs).map(|toks| toks.into()),
    }
}
