use crate::parse::Attributes;
use crate::parse::Mod;
use proc_macro2::{Span, TokenStream};
use std::path::PathBuf;

use quote::quote;
use std::fs;
use syn::Error as SynError;

pub fn build(s: Mod, attrs: Attributes) -> syn::Result<TokenStream> {
    let ident = s.ident;
    let outer_attrs = s.attrs;

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

    let cddl = fs::read(path)
        .map_err(|e| into_syn_error(file.span(), e))
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| into_syn_error(file.span(), e)))?;

    let _ctx = seedle_parser::parse(&cddl).map_err(|e| into_syn_error(file.span(), e))?;

    Ok(quote! {
        #(#outer_attrs)*
        pub mod #ident {}
    })
}

fn into_syn_error<E: std::error::Error>(span: Span, e: E) -> SynError {
    SynError::new(span, e.to_string())
}
