mod attr;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn seedle_bindgen(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let _attributes = syn::parse_macro_input!(attrs with attr::parse);
    unimplemented!()
}
