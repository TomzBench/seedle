#![allow(unused_macros)]
#![allow(unused_imports)]
#[cfg(feature = "error")]
pub mod error;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "from-bytes")]
pub mod from_bytes;

#[cfg(feature = "edit")]
pub mod edit;

#[cfg(feature = "infallible-encoder")]
pub mod infallible_encoder;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "cast")]
pub mod cast;

pub use seedle_derive::seedle;
