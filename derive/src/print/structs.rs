use crate::parse::Language;
use proc_macro2::Literal;
use quote::ToTokens;
use seedle_parser::{Fields, LinkedKeyVal};
use syn::Ident;

pub struct Struct<'a> {
    name: &'a str,
    prefix: &'a str,
    fields: &'a Fields,
    language: Language,
    partial: bool,
}
impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        unimplemented!()
    }
}

pub struct FieldC<'a> {
    member: &'a LinkedKeyVal,
}
impl<'a> ToTokens for FieldC<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        unimplemented!()
    }
}

struct MinicborConverters {
    deserialize: Literal,
    serialize: Literal,
    default: Literal,
    bytes: Literal,
}
impl MinicborConverters {
    fn new(partial: bool) -> Self {
        match partial {
            true => MinicborConverters {
                deserialize: Literal::string("de_option_str_as_bytes"),
                serialize: Literal::string("ser_bytes_as_str"),
                default: Literal::string("make_option_default_bytes"),
                bytes: Literal::string("minicbor::bytes"),
            },
            false => MinicborConverters {
                deserialize: Literal::string("de_str_as_bytes"),
                serialize: Literal::string("ser_bytes_as_str"),
                default: Literal::string("make_default_bytes"),
                bytes: Literal::string("minicbor::bytes"),
            },
        }
    }

    fn split_for_impl(self) -> (Literal, Literal, Literal, Literal) {
        (self.serialize, self.deserialize, self.default, self.bytes)
    }
}
