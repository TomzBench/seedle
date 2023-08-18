use crate::parse::Language;
use heck::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

pub fn build(name: Ident, language: Language, prefix: Option<LitStr>) -> TokenStream {
    let ffi = Ffi {
        name,
        language,
        prefix,
    };
    quote! {#ffi}
}

struct Ffi {
    name: Ident,
    prefix: Option<LitStr>,
    language: Language,
}

impl ToTokens for Ffi {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lang = self.language;
        let name = self.name.to_string();
        let fn_name = self.language.functionify(&name);
        let struct_name = format_ident!("{}", self.language.structify(&name));
        let prefix = match &self.prefix {
            Some(prefix) => format!("{}_", self.language.functionify(&prefix.value())),
            _ => format!(""),
        };
        let (encode, encode_attrs) =
            split_for_impl(lang, format_ident!("{}encode_{}", prefix, fn_name));
        let (encode_array, encode_array_attrs) =
            split_for_impl(lang, format_ident!("{}encode_{}_array", prefix, fn_name));
        let (decode, decode_attrs) =
            split_for_impl(lang, format_ident!("{}decode_{}", prefix, fn_name));
        let (decode_array, decode_array_attrs) =
            split_for_impl(lang, format_ident!("{}decode_{}_array", prefix, fn_name));
        let (len, len_attrs) = split_for_impl(lang, format_ident!("{}len_{}", prefix, fn_name));
        let (len_array, len_array_attrs) =
            split_for_impl(lang, format_ident!("{}array_len_{}", prefix, fn_name));
        quote! {
            #encode_attrs
            fn #encode(dst: *mut u8, dstlen: u32, src: &#struct_name) -> i32 {
                unimplemented!()
            }

            #encode_array_attrs
            fn #encode_array (dst: *mut u8, dstlen: u32, src: &#struct_name, srclen: u32) -> i32 {
                unimplemented!()
            }

            #decode_attrs
            fn #decode (dst: &mut #struct_name, src: *const u8, srclen: u32) -> i32 {
                unimplemented!()
            }

            #decode_array_attrs
            fn #decode_array (dst: &mut #struct_name, dstlen: u32, src: *const u8, srclen: u32) -> i32 {
                unimplemented!()
            }

            #len_attrs
            fn #len(src: &#struct_name) -> u32 {
                unimplemented!()
            }

            #len_array_attrs
            fn #len_array(src: &#struct_name, srclen: u32) -> u32 {
                unimplemented!()
            }
        }.to_tokens(tokens);
    }
}

fn split_for_impl(lang: Language, method: Ident) -> (TokenStream, TokenStream) {
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
