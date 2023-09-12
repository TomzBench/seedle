use super::literals::LitToks;
use crate::parse::Attributes;
use crate::parse::Language;
use crate::parse::Mod;
use crate::print::structs::Struct;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::path::PathBuf;

use quote::quote;
use std::fs;
use syn::Error as SynError;

pub fn build(s: Mod, attrs: Attributes) -> syn::Result<TokenStream> {
    let ident = s.ident;
    let outer_attrs = s.attrs;
    let language = attrs.language;
    let prefix = attrs.prefix;

    // Make sure we have the required "file" attribute to parse the cddl
    let file = attrs
        .file
        .ok_or_else(|| SynError::new(ident.span(), "File attribute required for modules"))?;

    // Normalize the path so that it is abosolute or relative to CARGO_MANIFEST_DIR
    let path = PathBuf::from(file.value());
    let path = match path.is_absolute() {
        true => path,
        _ => {
            let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root.push(path);
            root
        }
    };

    // Parse the users type definitions as CDDL file
    let cddl = fs::read(path)
        .map_err(|e| into_syn_error(file.span(), e))
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| into_syn_error(file.span(), e)))?;
    let ctx = seedle_parser::parse(&cddl).map_err(|e| into_syn_error(file.span(), e))?;

    // Get the prelude for the module
    let prelude = match language {
        Language::C => quote! {},
        Language::Typescript => quote! {use wasm_bindgen::prelude::*;},
        _ => quote! {},
    };

    // Generate bindings to export constants literals
    let literals: Vec<TokenStream> = ctx
        .iter()
        .filter_map(seedle_parser::literals_borrowed)
        .map(|(name, lit)| {
            LitToks {
                name,
                lit: lit.as_ref(),
                language,
            }
            .into_token_stream()
        })
        .collect();

    let structs: Vec<Struct> = ctx
        .iter()
        .filter_map(seedle_parser::structs_borrowed)
        .map(|(name, fields)| Struct {
            name,
            prefix: prefix.as_ref(),
            fields,
            language,
        })
        .collect();

    // TODO
    //      - generate wasm_constans from grouped literals
    //      - For every struct if C
    //          - struct impl
    //          - struct default impls
    //          - struct ffi impls
    //      - For every struct if rust
    //          - struct impls
    //          - struct default impls
    //      - For every struct if ts
    //          - struct impls
    //          - struct impls partial
    //          - struct from partial
    //          - struct default impl
    //          - struct wasm impl

    Ok(quote! {
        #(#outer_attrs)*
        pub mod #ident {
            #prelude
            #(#literals)*
            #(#structs)*
        }
    })
}

fn into_syn_error<E: std::error::Error>(span: Span, e: E) -> SynError {
    SynError::new(span, e.to_string())
}
