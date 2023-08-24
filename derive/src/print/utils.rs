use crate::parse::Language;
use heck::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn fn_attrs_split_for_impl(lang: Language, method: Ident) -> (TokenStream, TokenStream) {
    let attrs = match lang {
        Language::C => quote! {#[no_mangle] pub extern "C"},
        Language::Rust => quote! {pub},
        Language::Typescript => {
            let name = format_ident!("\"{}\"", method.to_string().to_lower_camel_case());
            quote! {#[wasm_bindgen(js_name=#name)] pub}
        }
    };
    (quote! {#method}, quote! {#attrs})
}
