use crate::parse::Language;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use seedle_parser::*;

pub struct NodeTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
}
impl<'a> ToTokens for NodeTokens<'a> {
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
        let node = NodeTokens {
            language: self.language,
            node: self.node.ty.as_ref(),
        };
        let len = proc_macro2::Literal::usize_unsuffixed(self.node.len);
        quote! {[ #node; #len ]}.to_tokens(tokens);
    }
}

pub struct FieldDefaultTokens<'a>(&'a LinkedNode);
impl<'a> ToTokens for FieldDefaultTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            LinkedNode::Primative(ConstrainedPrimative::Str(n)) => {
                let init = proc_macro2::Literal::u8_unsuffixed(0);
                let len = proc_macro2::Literal::u64_unsuffixed(*n);
                quote! {[ #init; #len ]}.to_tokens(tokens)
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
                    quote! {[ #init; #len ]}.to_tokens(tokens)
                }
                _ => quote! {[Default::default(); #len]}.to_tokens(tokens),
            },
            _ => quote! {Default::default()}.to_tokens(tokens),
        }
    }
}
