use crate::parse::Language;
use heck::*;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use seedle_parser::*;
use std::borrow::Cow;
use syn::LitStr;

pub struct Struct<'a> {
    pub name: &'a str,
    pub prefix: Option<&'a LitStr>,
    pub fields: Cow<'a, Fields>,
    pub language: Language,
    pub partial: bool,
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let language = self.language;
        let partial = self.partial;
        let name = language.structify(&format!(
            "{}{}",
            self.prefix
                .map(|v| Cow::Owned(v.value()))
                .unwrap_or(Cow::Borrowed("")),
            self.name
        ));
        let ident = quote::format_ident!("{}", name);
        let serde_rename_ts = proc_macro2::Literal::string("camelCase");
        let fields = self
            .fields
            .members
            .iter()
            .enumerate()
            .map(|(n, LinkedKeyVal(key, node))| {
                let attrs = AttrTokens {
                    node,
                    language,
                    n,
                    partial,
                };
                let field = FieldTokens {
                    language,
                    key,
                    partial,
                    node,
                };
                quote! {
                    #attrs
                    #field
                }
            });
        let default_impl = DefaultImpl {
            ident: &ident,
            fields: &self.fields,
            language,
        };
        let struct_impl = quote! {
                pub struct #ident {
                    #(#fields),*
                }
        };
        match self.language {
            Language::C => quote! {
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Copy, Clone, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
                #struct_impl
                #default_impl
            },
            Language::Rust => quote! {
                #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
                #struct_impl
                #default_impl
            },
            Language::Typescript => quote! {
                #[wasm_bindgen]
                #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
                #[serde(rename_all=#serde_rename_ts)]
                #struct_impl
                #default_impl
            },
        }
        .to_tokens(tokens);
    }
}

pub struct AttrTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
    n: usize,
    partial: bool,
}
impl<'a> ToTokens for AttrTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = proc_macro2::Literal::usize_unsuffixed(self.n);
        let bytes = proc_macro2::Literal::string("minicbor::bytes");
        let ser = proc_macro2::Literal::string("seedle_extra::serde::ser_bytes_as_str");
        let de;
        let def;
        if self.partial {
            de = proc_macro2::Literal::string("seedle_extra::serde::de_option_str_as_bytes");
            def = proc_macro2::Literal::string("seedle_extra::serde::make_option_default_bytes");
        } else {
            de = proc_macro2::Literal::string("seedle_extra::serde::de_str_as_bytes");
            def = proc_macro2::Literal::string("seedle_extra::serde::make_default_bytes");
        }
        match self.language {
            Language::C => match self.node {
                LinkedNode::Array(LinkedArray { ty, .. }) => match ty.as_ref() {
                    LinkedNode::Primative(ConstrainedPrimative::U8) => {
                        quote! {#[cbor(n(#n), with=#bytes)]}.to_tokens(tokens)
                    }
                    _ => quote! {#[n(#n)]}.to_tokens(tokens),
                },
                LinkedNode::Primative(ConstrainedPrimative::Str(_)) => {
                    quote! {#[cbor(n(#n), with=#bytes)]}.to_tokens(tokens)
                }
                _ => quote! {#[n(#n)]}.to_tokens(tokens),
            },
            Language::Rust | Language::Typescript => match self.node {
                LinkedNode::Array(LinkedArray { ty, len }) => match ty.as_ref() {
                    LinkedNode::Primative(ConstrainedPrimative::U8) if *len <= 32 => {
                        quote! {#[cbor(n(#n), with=#bytes)]}.to_tokens(tokens)
                    }
                    LinkedNode::Primative(ConstrainedPrimative::U8) => quote! {
                        #[cbor(n(#n), with=#bytes)]
                        #[serde(serialize_with=#ser)]
                        #[serde(deserialize_with=#de)]
                    }
                    .to_tokens(tokens),
                    _ => quote! {#[n(#n)]}.to_tokens(tokens),
                },
                LinkedNode::Primative(ConstrainedPrimative::Str(_)) => quote! {
                    #[cbor(n(#n), with=#bytes)]
                    #[serde(default=#def)]
                    #[serde(serialize_with=#ser)]
                    #[serde(deserialize_with=#de)]
                }
                .to_tokens(tokens),
                _ => quote! {#[n(#n)]}.to_tokens(tokens),
            },
        }
    }
}

pub struct FieldTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
    key: &'a str,
    partial: bool,
}
impl<'a> ToTokens for FieldTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = quote::format_ident!("{}", self.key.to_snake_case());
        let ty = TypeTokens {
            node: self.node,
            language: self.language,
        };
        match self.partial {
            true => quote! {#key: Option<#ty>},
            false => quote! {#key: #ty},
        }
        .to_tokens(tokens);
    }
}

pub struct TypeTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
}
impl<'a> ToTokens for TypeTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let language = self.language;
        match self.node {
            LinkedNode::Primative(node) => PrimativeTokens {
                language,
                node: *node,
            }
            .to_tokens(tokens),
            LinkedNode::Array(node) => ArrayTokens { language, node }.to_tokens(tokens),
            LinkedNode::ForeignStruct(node) => StructTokens { language, node }.to_tokens(tokens),
            field => syn::Error::new(Span::call_site(), format!("Invalid field! {:?}", field))
                .to_compile_error()
                .to_tokens(tokens),
        }
    }
}

pub struct PrimativeTokens {
    language: Language,
    node: ConstrainedPrimative,
}
impl ToTokens for PrimativeTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.node {
            ConstrainedPrimative::U8 => quote::format_ident!("u8").to_tokens(tokens),
            ConstrainedPrimative::U16 => quote::format_ident!("u16").to_tokens(tokens),
            ConstrainedPrimative::U32 => quote::format_ident!("u32").to_tokens(tokens),
            ConstrainedPrimative::U64 => quote::format_ident!("u64").to_tokens(tokens),
            ConstrainedPrimative::I8 => quote::format_ident!("i8").to_tokens(tokens),
            ConstrainedPrimative::I16 => quote::format_ident!("i16").to_tokens(tokens),
            ConstrainedPrimative::I32 => quote::format_ident!("i32").to_tokens(tokens),
            ConstrainedPrimative::I64 => quote::format_ident!("i64").to_tokens(tokens),
            ConstrainedPrimative::Bool => quote::format_ident!("bool").to_tokens(tokens),
            ConstrainedPrimative::Bytes(n) | ConstrainedPrimative::Str(n) => ArrayTokens {
                language: self.language,
                node: &LinkedArray::new(ConstrainedPrimative::U8.into(), n as usize),
            }
            .to_tokens(tokens),
        };
    }
}

pub struct StructTokens<'a> {
    node: &'a str,
    language: Language,
}
impl<'a> ToTokens for StructTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote::format_ident!("{}", self.language.structify(self.node)).to_tokens(tokens)
    }
}

pub struct ArrayTokens<'a> {
    language: Language,
    node: &'a LinkedArray,
}
impl<'a> ToTokens for ArrayTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let node = TypeTokens {
            language: self.language,
            node: self.node.ty.as_ref(),
        };
        let len = proc_macro2::Literal::usize_unsuffixed(self.node.len);
        quote! {[ #node; #len ]}.to_tokens(tokens);
    }
}

pub struct DefaultImpl<'a> {
    ident: &'a syn::Ident,
    fields: &'a Fields,
    language: Language,
}
impl<'a> ToTokens for DefaultImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let fields = self
            .fields
            .members
            .iter()
            .map(|LinkedKeyVal(key, node)| {
                let key = quote::format_ident!("{}", self.language.fieldify(key));
                let default_impl = match node {
                    LinkedNode::Primative(ConstrainedPrimative::Str(n)) => {
                        let init = proc_macro2::Literal::u8_unsuffixed(0);
                        let len = proc_macro2::Literal::u64_unsuffixed(*n);
                        quote! {[ #init; #len ]}
                    }
                    LinkedNode::Array(LinkedArray { ty, len }) => match ty.as_ref() {
                        LinkedNode::Primative(ConstrainedPrimative::U8)
                        | LinkedNode::Primative(ConstrainedPrimative::U16)
                        | LinkedNode::Primative(ConstrainedPrimative::U32)
                        | LinkedNode::Primative(ConstrainedPrimative::U64)
                        | LinkedNode::Primative(ConstrainedPrimative::I8)
                        | LinkedNode::Primative(ConstrainedPrimative::I16)
                        | LinkedNode::Primative(ConstrainedPrimative::I32)
                        | LinkedNode::Primative(ConstrainedPrimative::I64) => {
                            let init = proc_macro2::Literal::u8_unsuffixed(0);
                            let len = proc_macro2::Literal::u64_unsuffixed(*len as u64);
                            quote! {[ #init; #len ]}
                        }
                        _ => quote! {[Default::default(); #len]},
                    },
                    _ => quote! {Default::default()},
                };
                quote! {#key: #default_impl}
            })
            .collect::<Vec<TokenStream>>();
        quote! {
            impl Default for #ident {
                fn default() -> #ident {
                    #ident {
                        #(#fields),*
                    }
                }
            }
        }
        .to_tokens(tokens)
    }
}
