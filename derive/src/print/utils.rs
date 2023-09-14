use crate::parse::Language;
use heck::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn method_attrs(lang: Language, method: Ident) -> (TokenStream, TokenStream) {
    use proc_macro2::Literal;
    let attrs = match lang {
        Language::C => quote! {#[no_mangle] pub extern "C"},
        Language::Rust => quote! {pub},
        Language::Typescript => {
            let n = Literal::string(&format!("{}", method.to_string().to_lower_camel_case()));
            quote! {
                #[wasm_bindgen(js_name=#n)]
                #[no_mangle]
                pub extern "C"
            }
        }
    };
    (quote! {#method}, quote! {#attrs})
}
