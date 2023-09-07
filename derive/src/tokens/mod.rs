pub(crate) mod literals;
use proc_macro2::TokenStream;

pub(crate) trait Tokens {
    fn tokens(&self) -> TokenStream;
}
