mod parse;
mod print;
use parse::attr::{self};
use parse::item::{self, Item};
use print::{ffi, vtable};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn seedle(attrs: TokenStream, toks: TokenStream) -> TokenStream {
    let orig = toks.clone();
    let attrs = syn::parse_macro_input!(attrs with attr::parse);
    let item = syn::parse_macro_input!(toks with item::parse);

    match item {
        Item::Struct(i) => {
            let ffi = ffi::build(i, attrs.language, attrs.prefix);
            let toks = proc_macro2::TokenStream::from(orig);
            quote! {
                #ffi
                #toks
            }
            .into()
        }
        Item::Enum(e) => {
            let vtable = vtable::build(e, attrs.language, attrs.prefix);
            let toks = proc_macro2::TokenStream::from(orig);
            quote! {
                #vtable
                #toks
            }
            .into()
        }
    }
}
