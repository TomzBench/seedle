mod parse;
mod print;
use parse::attr::{self};
use parse::item::{self, Item};
use print::{ffi, vtable};
use proc_macro::TokenStream;
use quote::quote;

// TODO - need to implement the ffi impls
#[proc_macro_attribute]
pub fn seedle(attrs: TokenStream, toks: TokenStream) -> TokenStream {
    let attrs = syn::parse_macro_input!(attrs with attr::parse);
    let item = syn::parse_macro_input!(toks with item::parse);

    match item {
        Item::Struct(s) => {
            let ffi = ffi::build(s, attrs.language, attrs.prefix);
            quote! {#ffi}
            .into()
        }
        Item::Enum(e) => {
            let vtable = vtable::build(e, attrs.language, attrs.prefix);
            quote! {#vtable}
            .into()
        }
    }
}
