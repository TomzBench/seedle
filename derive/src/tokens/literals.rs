use super::Tokens;
use crate::parse::Language;
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Display;

pub struct LitToks<'a> {
    pub name: &'a str,
    pub lit: &'a seedle_parser::Literal,
    pub language: Language,
}

impl<'a> Tokens for LitToks<'a> {
    fn tokens(&self) -> TokenStream {
        match self.lit {
            seedle_parser::Literal::Bool(lit) => CopyToks {
                lit,
                name: self.name,
                language: self.language,
                ty: "bool",
            }
            .tokens(),
            seedle_parser::Literal::Int(lit) => CopyToks {
                lit,
                name: self.name,
                language: self.language,
                ty: "i64",
            }
            .tokens(),
            seedle_parser::Literal::UInt(lit) => CopyToks {
                lit,
                name: self.name,
                language: self.language,
                ty: "u64",
            }
            .tokens(),
            seedle_parser::Literal::Char(lit) => CharToks {
                lit: *lit,
                name: self.name,
                language: self.language,
            }
            .tokens(),
            seedle_parser::Literal::Str(lit) => StrToks {
                lit,
                name: self.name,
                language: self.language,
            }
            .tokens(),
            _ => unimplemented!(),
        }
    }
}

struct CopyToks<'a, T: ToTokens + Copy + Display> {
    name: &'a str,
    ty: &'a str,
    lit: T,
    language: Language,
}
impl<'a, T: ToTokens + Copy + Display> Tokens for CopyToks<'a, T> {
    fn tokens(&self) -> TokenStream {
        let name = quote::format_ident!("{}", self.language.enumify(self.name));
        let ts_name = quote::format_ident!("TS_{}", self.language.enumify(self.name));
        let ty = quote::format_ident!("{}", self.ty);
        let val = self.lit;
        match self.language {
            Language::Typescript => {
                let s = Literal::string(&format!("export type {} = {}", self.name, self.lit));
                quote! {
                    pub const #name: #ty = #val;
                    #[wasm_bindgen(typescript_custom_section)]
                    const #ts_name: &'static str = #s ;
                }
            }
            _ => quote! {pub const #name: #ty = #val;},
        }
    }
}

struct CharToks<'a> {
    name: &'a str,
    lit: char,
    language: Language,
}
impl<'a> Tokens for CharToks<'a> {
    fn tokens(&self) -> TokenStream {
        let name = quote::format_ident!("{}", self.language.enumify(self.name));
        let ts_name = quote::format_ident!("TS_{}", self.language.enumify(self.name));
        let val = Literal::character(self.lit);
        match self.language {
            Language::Typescript => {
                let s = Literal::string(&format!("export type {} = '{}'", self.name, self.lit));
                quote! {
                    pub const #name: char = #val;
                    #[wasm_bindgen(typescript_custom_section)]
                    const #ts_name: &'static str = #s ;
                }
            }
            _ => quote! {pub const #name: char = #val;},
        }
    }
}

struct StrToks<'a> {
    name: &'a str,
    lit: &'a str,
    language: Language,
}
impl<'a> Tokens for StrToks<'a> {
    fn tokens(&self) -> TokenStream {
        let name = quote::format_ident!("{}", self.language.enumify(self.name));
        let ts_name = quote::format_ident!("TS_{}", self.language.enumify(self.name));
        let val = quote::format_ident!("\"{}\"'", self.lit);
        match self.language {
            Language::Typescript => {
                let s = Literal::string(&format!("export type {} = \"{}\"", self.name, self.lit));
                quote! {
                    pub const #name: &'static str = #val;
                    #[wasm_bindgen(typescript_custom_section)]
                    const #ts_name: &'static str = #s ;
                }
            }
            _ => quote! {pub const #name: &'static str = #val;},
        }
    }
}
