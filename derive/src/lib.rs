mod parse;
mod print;
use parse::attr::{self};
use parse::item::{self, Item};
use print::{ffi, vtable};
use proc_macro::TokenStream;

// TODO - need to implement the ffi impls
#[proc_macro_attribute]
pub fn seedle(attrs: TokenStream, toks: TokenStream) -> TokenStream {
    let attrs = syn::parse_macro_input!(attrs with attr::parse);
    let item = syn::parse_macro_input!(toks with item::parse);

    match item {
        Item::Struct(s) => ffi::build(s, attrs.language, attrs.prefix).into(),
        Item::Enum(e) => vtable::build(e, attrs.language, attrs.prefix).into(),
    }
}
