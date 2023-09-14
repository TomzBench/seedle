mod parse;
mod print;
use print::cddl;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn seedle(attrs: TokenStream, toks: TokenStream) -> TokenStream {
    do_seedle(attrs, toks).map_or_else(
        |e| TokenStream::from(e.to_compile_error()),
        TokenStream::from,
    )
}

fn do_seedle(attrs: TokenStream, toks: TokenStream) -> syn::Result<TokenStream2> {
    let attrs: parse::Attributes = syn::parse(attrs)?;
    let item: parse::Mod = syn::parse(toks)?;
    cddl::build(item, attrs).map(|toks| toks.into())
}
