use std::borrow::Cow;
use std::{env, fs, path};
fn main() {
    let out = path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("extra.rs");
    let prefix = env::var("SEEDLE_PREFIX")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("cbor"));
    fs::write(
        out,
        format!(
            r#"#[cfg(feature = "ffi_c")]
            macros::extra!("C", {prefix});
            
            #[cfg(feature = "ffi_ts")]
            macros::extra!("TS", {prefix});"#,
            prefix = prefix
        ),
    )
    .unwrap();
}
