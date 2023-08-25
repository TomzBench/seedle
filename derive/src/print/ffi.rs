use crate::parse::{Language, Struct};
use crate::print::utils::fn_attrs_split_for_impl;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

pub fn build(s: Struct, language: Language, prefix: Option<LitStr>) -> TokenStream {
    let ident = quote::format_ident!("{}", language.structify(&s.ident.to_string()));
    let attrs = s.attrs;
    let fields = s.fields;
    let ffi = Ffi {
        name: s.ident,
        language,
        prefix,
    };
    match language {
        Language::C => quote! {
            #[repr(C)]
            #[derive(Copy, Clone, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
            #[allow(non_camel_case_types)]
            #(#attrs)*
            pub struct #ident #fields
            #ffi
        },
        Language::Rust => quote! {
            #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
            #(#attrs)*
            pub struct #ident #fields
            #ffi
        },
        Language::Typescript => quote! {
            #[wasm_bindgen]
            #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
            #[serde(rename_all = "camelCase")]
            #(#attrs)*
            pub struct #ident #fields
            #ffi
        },
    }
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
            fn_attrs_split_for_impl(lang, format_ident!("{}encode_{}", prefix, fn_name));
        let (encode_array, encode_array_attrs) =
            fn_attrs_split_for_impl(lang, format_ident!("{}encode_{}_array", prefix, fn_name));
        let (decode, decode_attrs) =
            fn_attrs_split_for_impl(lang, format_ident!("{}decode_{}", prefix, fn_name));
        let (decode_array, decode_array_attrs) =
            fn_attrs_split_for_impl(lang, format_ident!("{}decode_{}_array", prefix, fn_name));
        let (len, len_attrs) =
            fn_attrs_split_for_impl(lang, format_ident!("{}len_{}", prefix, fn_name));
        let (len_array, len_array_attrs) =
            fn_attrs_split_for_impl(lang, format_ident!("{}array_len_{}", prefix, fn_name));
        quote! {
            #encode_attrs
            fn #encode(dst: *mut u8, dstlen: u32, src: &#struct_name) -> i32 {
                unsafe {
                    let slice = core::slice::from_raw_parts_mut(dst, dstlen as usize);
                    let cursor = minicbor::encode::write::Cursor::new(slice.as_mut());
                    let mut encoder = minicbor::Encoder::new(cursor);
                    encoder
                        .encode(&*(src as *const #struct_name))
                        .map_or(-1, |encoder| encoder.writer().position() as i32)
                }
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
