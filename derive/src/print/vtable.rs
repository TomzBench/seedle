use crate::parse::{Enum, Language};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, Token};

pub fn build(item: Enum, language: Language, prefix: Option<LitStr>) -> TokenStream {
    let vtable = VTable {
        entries: item.items,
        language,
        prefix,
    };
    quote! {#vtable}
}

struct VTable {
    entries: Punctuated<Ident, Token![,]>,
    language: Language,
    prefix: Option<LitStr>,
}
impl ToTokens for VTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let len = self.entries.len();
        let table = self.entries.iter().map(|ident| VTableEntry {
            entry: &ident,
            language: self.language,
            prefix: &self.prefix,
        });
        quote! {
            const __SEEDLE_VTABLE: [seedle_extra::ffi::SeedleVTableEntry; #len] = [ #(#table),* ];
        }
        .to_tokens(tokens);
    }
}

struct VTableEntry<'l> {
    entry: &'l Ident,
    language: Language,
    prefix: &'l Option<LitStr>,
}
impl<'l> ToTokens for VTableEntry<'l> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.language.functionify(&self.entry.to_string());
        let struct_name = quote::format_ident!("{}", name);
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
