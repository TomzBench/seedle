use crate::parse::Language;
use crate::print::utils::method_attrs;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use seedle_parser::Fields;
use std::borrow::Cow;
use syn::{Ident, LitStr};

/*
pub fn build(item: Enum, language: Language, oprefix: Option<LitStr>) -> TokenStream {
    let key = format_ident!("{}", language.enumify(&item.ident.to_string()));
    let prefix = match &oprefix {
        Some(prefix) => format!("{}_", language.functionify(&prefix.value())),
        _ => format!(""),
    };
    let lang_attr = match language {
        Language::C => quote! {},
        Language::Typescript => quote! {#[wasm_bindgen]},
        _ => quote! {},
    };
    let enumeration = item.items.iter().enumerate().map(|(idx, val)| {
        let name = format_ident!("{}", language.enumify(&val.to_string()));
        let idx = proc_macro2::Literal::i8_unsuffixed(idx as i8);
        quote! {#name = #idx}
    });
    let vtable = VTable {
        entries: &item.items,
        language,
        prefix: oprefix,
    };
    let (enc, enc_attrs) = method_attrs(language, format_ident!("{}encode", prefix));
    let (enc_arr, enc_arr_attrs) = method_attrs(language, format_ident!("{}encode_array", prefix));
    let (dec, dec_attrs) = method_attrs(language, format_ident!("{}decode", prefix));
    let (dec_arr, dec_arr_attrs) = method_attrs(language, format_ident!("{}decode_array", prefix));
    let (len, len_attrs) = method_attrs(language, format_ident!("{}len", prefix));
    let (len_arr, len_arr_attrs) = method_attrs(language, format_ident!("{}array_len", prefix));

    quote! {
        #vtable
        #lang_attr
        #[repr(u8)]
        pub enum #key {
            #(#enumeration),*
        }

        #enc_attrs
        fn #enc(dst: *mut u8, dstlen: u32, key: #key, src: *const core::ffi::c_void) -> i32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].encode)(dst, dstlen, &*src)}
        }

        #enc_arr_attrs
        fn #enc_arr(dst: *mut u8, dstlen: u32, key: #key, src: *const core::ffi::c_void, srclen: u32) -> i32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].encode_array)(dst, dstlen, &*src, srclen)}
        }

        #dec_attrs
        fn #dec (dst: *mut core::ffi::c_void, key: #key, src: *const u8, srclen: u32) -> i32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].decode)(&mut *dst, src, srclen)}
        }

        #dec_arr_attrs
        fn #dec_arr (dst: *mut core::ffi::c_void, dstlen: u32, key: #key, src: *const u8, srclen: u32) -> i32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].decode_array)(&mut *dst, dstlen, src, srclen)}
        }

        #len_attrs
        fn #len(key: #key, src: *const core::ffi::c_void) -> u32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].len)(&*src)}
        }

        #len_arr_attrs
        fn #len_arr(key: #key, src: *const core::ffi::c_void, srclen: u32) -> u32 {
            unsafe {(__SEEDLE_VTABLE[key as u8 as usize].array_len)(&*src, srclen)}
        }
    }
}
*/

pub struct VTable<'i> {
    pub structs: Vec<(&'i String, Cow<'i, Fields>)>,
    pub ident: &'i Ident,
    pub language: Language,
    pub prefix: &'i Option<LitStr>,
}
impl<'i> ToTokens for VTable<'i> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lang = self.language;
        let key = format_ident!("{}", lang.enumify(&self.ident.to_string()));
        let n = proc_macro2::Literal::usize_unsuffixed(self.structs.len());
        let table = self.structs.iter().map(|(key, _)| VTableEntry {
            entry: format_ident!("{}", lang.enumify(key)),
            language: lang,
            prefix: &self.prefix,
        });
        let prefix = match self.prefix {
            Some(prefix) => format!("{}_", lang.functionify(&prefix.value())),
            _ => format!(""),
        };
        let lang_attr = match lang {
            Language::Typescript => Some(quote! {#[wasm_bindgen]}),
            _ => None,
        };
        let enumeration = self.structs.iter().enumerate().map(|(idx, (key, _node))| {
            let name = format_ident!("{}", lang.enumify(key));
            let idx = proc_macro2::Literal::i8_unsuffixed(idx as i8);
            quote! {#name = #idx}
        });
        let (enc, enc_attrs) = method_attrs(lang, format_ident!("{}encode", prefix));
        let (enc_arr, enc_arr_attrs) = method_attrs(lang, format_ident!("{}encode_array", prefix));
        let (dec, dec_attrs) = method_attrs(lang, format_ident!("{}decode", prefix));
        let (dec_arr, dec_arr_attrs) = method_attrs(lang, format_ident!("{}decode_array", prefix));
        let (len, len_attrs) = method_attrs(lang, format_ident!("{}len", prefix));
        let (len_arr, len_arr_attrs) = method_attrs(lang, format_ident!("{}array_len", prefix));

        quote! {
            const __SEEDLE_VTABLE: [seedle_extra::ffi::SeedleVTableEntry; #n] = [ #(#table),* ];

            #lang_attr
            #[repr(u8)]
            pub enum #key {
                #(#enumeration),*
            }

            #enc_attrs
            fn #enc(dst: *mut u8, dstlen: u32, key: #key, src: *const core::ffi::c_void) -> i32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].encode)(dst, dstlen, &*src)}
            }

            #enc_arr_attrs
            fn #enc_arr(dst: *mut u8, dstlen: u32, key: #key, src: *const core::ffi::c_void, srclen: u32) -> i32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].encode_array)(dst, dstlen, &*src, srclen)}
            }

            #dec_attrs
            fn #dec (dst: *mut core::ffi::c_void, key: #key, src: *const u8, srclen: u32) -> i32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].decode)(&mut *dst, src, srclen)}
            }

            #dec_arr_attrs
            fn #dec_arr (dst: *mut core::ffi::c_void, dstlen: u32, key: #key, src: *const u8, srclen: u32) -> i32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].decode_array)(&mut *dst, dstlen, src, srclen)}
            }

            #len_attrs
            fn #len(key: #key, src: *const core::ffi::c_void) -> u32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].len)(&*src)}
            }

            #len_arr_attrs
            fn #len_arr(key: #key, src: *const core::ffi::c_void, srclen: u32) -> u32 {
                unsafe {(__SEEDLE_VTABLE[key as u8 as usize].array_len)(&*src, srclen)}
            }
        }
        .to_tokens(tokens);
    }
}

struct VTableEntry<'l> {
    entry: Ident,
    language: Language,
    prefix: &'l Option<LitStr>,
}
impl<'l> ToTokens for VTableEntry<'l> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.language.functionify(&self.entry.to_string());
        let struct_name = quote::format_ident!("{}", self.language.structify(&name));
        let prefix = match self.prefix {
            Some(prefix) => format!("{}_", self.language.functionify(&prefix.value())),
            _ => format!(""),
        };
        let encoder = quote::format_ident!("{}encode_{}", prefix, name);
        let decoder = quote::format_ident!("{}decode_{}", prefix, name);
        let array_encoder = quote::format_ident!("{}encode_{}_array", prefix, name);
        let array_decoder = quote::format_ident!("{}decode_{}_array", prefix, name);
        let len = quote::format_ident!("{}len_{}", prefix, name);
        let array_len = quote::format_ident!("{}array_len_{}", prefix, name);
        quote! {
            unsafe { seedle_extra::ffi::SeedleVTableEntry {
                encode: core::mem::transmute::<seedle_extra::ffi::EncodeFn<#struct_name>, seedle_extra::ffi::EncodeFnErased>(#encoder),
                encode_array: core::mem::transmute::<seedle_extra::ffi::EncodeArrayFn<#struct_name>, seedle_extra::ffi::EncodeArrayFnErased>(#array_encoder),
                decode: core::mem::transmute::<seedle_extra::ffi::DecodeFn<#struct_name>, seedle_extra::ffi::DecodeFnErased>(#decoder),
                decode_array: core::mem::transmute::<seedle_extra::ffi::DecodeArrayFn<#struct_name>, seedle_extra::ffi::DecodeArrayFnErased>(#array_decoder),
                len: core::mem::transmute::<seedle_extra::ffi::LenFn<#struct_name>, seedle_extra::ffi::LenFnErased>(#len),
                array_len: core::mem::transmute::<seedle_extra::ffi::ArrayLenFn<#struct_name>, seedle_extra::ffi::ArrayLenFnErased>(#array_len),
            }}
        }
        .to_tokens(tokens)
    }
}
