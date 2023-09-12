use crate::parse::Language;
use heck::*;
use indoc::formatdoc;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use seedle_parser::*;
use std::borrow::Cow;
use syn::LitStr;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Totality {
    Complete,
    Partial,
}
impl Totality {
    fn is_partial(&self) -> bool {
        match self {
            Totality::Complete => false,
            Totality::Partial => true,
        }
    }
}

pub struct Struct<'a> {
    pub name: &'a str,
    pub prefix: Option<&'a LitStr>,
    pub fields: Cow<'a, Fields>,
    pub language: Language,
}
impl<'a> Struct<'a> {
    fn render(&self) -> TokenStream {
        match self.language {
            Language::C => self.render_c(),
            Language::Rust => self.render_rust(),
            Language::Typescript => self.render_typescript(),
        }
    }

    fn render_c(&self) -> TokenStream {
        let language = self.language;
        let prefix = self
            .prefix
            .map(|v| Cow::Owned(v.value()))
            .unwrap_or(Cow::Borrowed(""));
        let name = &format!("{}{}", prefix, self.name);
        let struct_ident = quote::format_ident!("{}", language.structify(name));
        let fields = self
            .fields
            .members
            .iter()
            .enumerate()
            .map(|(n, LinkedKeyVal(key, node))| FieldAttrTokens {
                node,
                language,
                key,
                totality: Totality::Complete,
                n,
            });
        let default_impl = DefaultImpl {
            ident: &struct_ident,
            fields: &self.fields,
            language,
        };
        let struct_attrs = quote! {
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
        };
        let struct_impl = quote! {
            pub struct #struct_ident{
                #(#fields),*
            }
        };
        quote! {
            #struct_attrs
            #struct_impl
            #default_impl
        }
    }

    fn render_rust(&self) -> TokenStream {
        let language = self.language;
        let prefix = self
            .prefix
            .map(|v| Cow::Owned(v.value()))
            .unwrap_or(Cow::Borrowed(""));
        let name = &format!("{}{}", prefix, self.name);
        let struct_ident = quote::format_ident!("{}", language.structify(name));
        let fields = self
            .fields
            .members
            .iter()
            .enumerate()
            .map(|(n, LinkedKeyVal(key, node))| FieldAttrTokens {
                node,
                language,
                key,
                totality: Totality::Complete,
                n,
            });
        let default_impl = DefaultImpl {
            ident: &struct_ident,
            fields: &self.fields,
            language,
        };
        let struct_attrs = quote! {
            #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
        };
        let struct_impl = quote! {
            pub struct #struct_ident{
                #(#fields),*
            }
        };
        quote! {
            #struct_attrs
            #struct_impl
            #default_impl
        }
    }

    fn render_typescript(&self) -> TokenStream {
        let language = self.language;
        let prefix = self
            .prefix
            .map(|v| Cow::Owned(v.value()))
            .unwrap_or(Cow::Borrowed(""));
        let name_complete = &format!("{}{}", prefix, self.name);
        let name_partial = &format!("partial_{}{}", prefix, self.name);
        let struct_ident_complete = quote::format_ident!("{}", language.structify(name_complete));
        let struct_ident_partial = quote::format_ident!("{}", language.structify(name_partial));
        let serde_rename_ts = proc_macro2::Literal::string("camelCase");
        let fields_complete =
            self.fields
                .members
                .iter()
                .enumerate()
                .map(|(n, LinkedKeyVal(key, node))| FieldAttrTokens {
                    node,
                    language,
                    key,
                    totality: Totality::Complete,
                    n,
                });
        let fields_partial =
            self.fields
                .members
                .iter()
                .enumerate()
                .map(|(n, LinkedKeyVal(key, node))| FieldAttrTokens {
                    node,
                    language,
                    key,
                    totality: Totality::Partial,
                    n,
                });
        let default_impl = DefaultImpl {
            ident: &struct_ident_complete,
            fields: &self.fields,
            language,
        };
        let wasm_impl = WasmImpl {
            ident: &struct_ident_complete,
            fields: &self.fields,
        };
        let struct_attrs = quote! {
            #[wasm_bindgen]
            #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, minicbor::CborLen, minicbor::Encode, minicbor::Decode)]
            #[serde(rename_all=#serde_rename_ts)]
        };
        let struct_complete_impl = quote! {
            pub struct #struct_ident_complete {
                #(#fields_complete),*
            }
        };
        let struct_partial_impl = quote! {
            pub struct #struct_ident_partial {
                #(#fields_partial),*
            }
        };
        quote! {
            #struct_attrs
            #struct_complete_impl
            #struct_attrs
            #struct_partial_impl
            #default_impl
            #wasm_impl
        }
    }
}

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.render().to_tokens(tokens);
    }
}

pub struct FieldAttrTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
    totality: Totality,
    n: usize,
    key: &'a str,
}
impl<'a> ToTokens for FieldAttrTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attr = AttrTokens {
            node: self.node,
            language: self.language,
            totality: self.totality,
            n: self.n,
        };
        let field = FieldTokens {
            node: self.node,
            language: self.language,
            totality: self.totality,
            key: self.key,
        };
        quote! {
            #attr
            #field
        }
        .to_tokens(tokens)
    }
}

pub struct AttrTokens<'a> {
    node: &'a LinkedNode,
    language: Language,
    n: usize,
    totality: Totality,
}
impl<'a> ToTokens for AttrTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = proc_macro2::Literal::usize_unsuffixed(self.n);
        let bytes = proc_macro2::Literal::string("minicbor::bytes");
        let ser = proc_macro2::Literal::string("seedle_extra::serde::ser_bytes_as_str");
        let de;
        let def;
        if self.totality.is_partial() {
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
    totality: Totality,
    // Public/private based on lang?
}
impl<'a> ToTokens for FieldTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = quote::format_ident!("{}", self.key.to_snake_case());
        let ty = TypeTokens {
            node: self.node,
            language: self.language,
        };
        match (self.language, self.totality) {
            (Language::Typescript, Totality::Partial) => quote! {#key: Option<#ty>},
            (Language::Typescript, Totality::Complete) => quote! {#key: #ty},
            (Language::C | Language::Rust, Totality::Complete) => quote! {pub #key: #ty},
            (Language::C | Language::Rust, Totality::Partial) => quote! {pub #key: Option<#ty>},
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

struct WasmImpl<'a> {
    ident: &'a syn::Ident,
    fields: &'a Fields,
}
impl<'a> ToTokens for WasmImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use proc_macro2::Literal;
        let fields = self
            .fields
            .members
            .iter()
            .map(|LinkedKeyVal(key, val)| {
                match val {
                    LinkedNode::Primative(ConstrainedPrimative::U8)
                    | LinkedNode::Primative(ConstrainedPrimative::I8)
                    | LinkedNode::Primative(ConstrainedPrimative::U16)
                    | LinkedNode::Primative(ConstrainedPrimative::I16)
                    | LinkedNode::Primative(ConstrainedPrimative::U32)
                    | LinkedNode::Primative(ConstrainedPrimative::I32)
                    | LinkedNode::Primative(ConstrainedPrimative::U64)
                    | LinkedNode::Primative(ConstrainedPrimative::I64) => {
                        quote! {#key: number}
                    }
                    LinkedNode::Primative(ConstrainedPrimative::Bool) => {
                        quote! {#key: boolean}
                    }
                    LinkedNode::Primative(ConstrainedPrimative::Str(_len)) => {
                        quote! {#key: string}
                    }
                    LinkedNode::ForeignStruct(s) => {
                        let ident = quote::format_ident!("Partial{}Props", s.to_upper_camel_case());
                        quote! {#key: #ident}
                    }
                    LinkedNode::Array(LinkedArray { ty, .. }) => match &**ty {
                        LinkedNode::Primative(ConstrainedPrimative::I8) => {
                            quote! {#key: Int8Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::U8) => {
                            quote! {#key: UInt8Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::I16) => {
                            quote! {#key: Int16Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::U16) => {
                            quote! {#key: UInt16Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::I32) => {
                            quote! {#key: Int32Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::U32) => {
                            quote! {#key: UInt32Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::I64) => {
                            quote! {#key: Int64Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::U64) => {
                            quote! {#key: UInt64Array}
                        }
                        LinkedNode::Primative(ConstrainedPrimative::Bool) => {
                            quote! {#key: boolean[]}
                        }
                        LinkedNode::ForeignStruct(s) => {
                            let ident =
                                quote::format_ident!("Partial{}Props", s.to_upper_camel_case());
                            quote! {#key: #ident[]}
                        }
                        s => syn::Error::new(
                            Span::call_site(),
                            &format!("Unexpected wasm field type {:?}", s),
                        )
                        .to_compile_error(),
                    },
                    s => syn::Error::new(
                        Span::call_site(),
                        &format!("Unexpected wasm field type {:?}", s),
                    )
                    .to_compile_error(),
                }
                .to_string()
            })
            .collect::<Vec<String>>()
            .join(",\n");

        // NOTE the value in value.#member is hard coded to match the parameter of the from impl
        let from_partial_fields = self.fields.members.iter().map(|LinkedKeyVal(key, val)| {
            let member = quote::format_ident!("{}", key.to_snake_case());
            let default_impl = match val {
                LinkedNode::Primative(ConstrainedPrimative::Str(n)) => {
                    let size = Literal::u64_unsuffixed(*n);
                    quote! {[0; #size]}
                }
                LinkedNode::Array(LinkedArray { ty, len }) => {
                    let size = Literal::usize_unsuffixed(*len);
                    match **ty {
                        LinkedNode::Primative(ConstrainedPrimative::U8)
                        | LinkedNode::Primative(ConstrainedPrimative::U16)
                        | LinkedNode::Primative(ConstrainedPrimative::U32)
                        | LinkedNode::Primative(ConstrainedPrimative::U64)
                        | LinkedNode::Primative(ConstrainedPrimative::I8)
                        | LinkedNode::Primative(ConstrainedPrimative::I16)
                        | LinkedNode::Primative(ConstrainedPrimative::I32)
                        | LinkedNode::Primative(ConstrainedPrimative::I64) => quote! {[0; #size]},
                        _ => quote! {[Default::default(); #size]},
                    }
                }
                _ => quote! {Default::default()},
            };
            quote! {#member: value.#member.unwrap_or_else(|| #default_impl)}
        });

        let member_impls = self
            .fields
            .members
            .iter()
            .map(|LinkedKeyVal(key, val)| match val {
                _ => quote! {},
            });

        let name = self.ident.to_string().to_upper_camel_case();
        let name_const = self.ident.to_string().to_shouty_snake_case();
        let struct_ident = quote::format_ident!("{}", name);
        let struct_props_ident = quote::format_ident!("{}Props", name);
        let struct_props_str = Literal::string(&struct_props_ident.to_string());
        let struct_partial_ident = quote::format_ident!("Partial{}", name);
        let struct_partial_props_ident = quote::format_ident!("Partial{}Props", name);
        let struct_partial_props_str = Literal::string(&struct_partial_props_ident.to_string());
        let struct_arr_ident = quote::format_ident!("{}Array", name);
        let struct_arr_str = Literal::string(&format!("{}[]", name));
        let ts_append_content_ident = quote::format_ident!("TS_APPEND_CONTENT_{}", name_const);
        let ts_append_content = Literal::string(&formatdoc! {
            "export type {struct_props} = {{
                {fields}
            }};
            export type Partial{struct_props} = Partial<{struct_props}>;",
            struct_props = struct_props_ident.to_string(),
            fields = fields
        });

        quote! {
            impl From<#struct_partial_ident> for #struct_ident {
                fn from(value: #struct_partial_ident) -> #struct_ident {
                    #struct_ident {
                        #(#from_partial_fields),*
                    }
                }
            }

            #[wasm_bindgen(typescript_custom_section)]
            const #ts_append_content_ident: &'static str = #ts_append_content;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(typescript_type = #struct_props_str)]
                pub type #struct_props_ident;

                #[wasm_bindgen(typescript_type = #struct_partial_props_str)]
                pub type #struct_partial_props_ident;

                #[wasm_bindgen(typescript_type = #struct_arr_str)]
                pub type #struct_arr_ident;
            }

            #[wasm_bindgen]
            impl #struct_ident {
                #[wasm_bindgen(constructor)]
                pub fn new(props: Option<#struct_partial_props_ident>) -> Result<#struct_ident, JsValue> {
                    match props {
                        Some(partial) => Ok(#struct_ident::from(serde_wasm_bindgen::from_value::<#struct_partial_ident>(partial.into())?)),
                        _=> Ok(#struct_ident::default())
                    }
                }

                #[wasm_bindgen(js_name="fromJson")]
                pub fn from_json(json: &str) -> Result<#struct_ident, JsValue> {
                    serde_json::from_str(json).map_err(|e| JsValue::from(&e.to_string()))
                }

                #[wasm_bindgen(js_name="fromCbor")]
                pub fn from_cbor(cbor: &[u8]) -> Result<#struct_ident, JsValue> {
                    let mut dec = minicbor::Decoder::new(cbor);
                    dec.decode().map_err(|e| JsValue::from(&e.to_string()))
                }

                #[wasm_bindgen(js_name="fromCborArray")]
                pub fn from_cbor_array(cbor: &[u8]) -> Result<#struct_arr_ident, JsValue> {
                    let mut dec = minicbor::Decoder::new(cbor);
                    let len = dec.probe()
                                 .array()
                                 .map_err(|e| JsValue::from_str(&e.to_string()))?
                                 .ok_or_else(|| JsValue::from_str("invalid array"))?;
                    let mut vec: Vec<#struct_ident> = Vec::with_capacity(len as usize);
                    for decoded in dec.array_iter::<#struct_ident>().map_err(|e| JsValue::from_str(&e.to_string()))? {
                        vec.push(decoded.map_err(|e| JsValue::from_str(&e.to_string()))?);
                    }
                    Ok(vec
                          .into_iter()
                          .map(JsValue::from)
                          .collect::<js_sys::Array>()
                          .unchecked_into::<#struct_arr_ident>())
                }

                #[wasm_bindgen(js_name="toCborArray")]
                pub fn to_cbor_array(arr: Vec<#struct_props_ident>) -> Result<Vec<u8>, JsValue> {
                    Self::cbor_array(arr)
                }

                #[wasm_bindgen(js_name="cborArray")]
                pub fn cbor_array(arr: Vec<#struct_props_ident>) -> Result<Vec<u8>, JsValue> {
                    let vec: Vec<#struct_ident> = arr.into_iter()
                                                  .map(|item| serde_wasm_bindgen::from_value::<#struct_ident>(item.into()))
                                                  .collect::<Result<Vec<#struct_ident>, serde_wasm_bindgen::Error>>()
                                                  .map_err(JsValue::from)?;
                    let len = minicbor::CborLen::cbor_len(vec.as_slice(), &mut ());
                    let mut enc = minicbor::Encoder::new(seedle_extra::infallible_encoder::InfallibleEncoder::new(len));
                    enc.encode(vec.as_slice()).expect("infallible!");
                    Ok(enc.into_writer().into_inner())
                }

                #[wasm_bindgen(js_name="toJson")]
                pub fn to_json(&self) -> Result<JsValue, JsValue> {
                    self.as_json()
                }

                #[wasm_bindgen(js_name="json")]
                pub fn as_json(&self) -> Result<JsValue, JsValue> {
                    serde_wasm_bindgen::to_value(self).map_err(|e| e.into())
                }

                #[wasm_bindgen(js_name="toCbor")]
                pub fn to_cbor(&self) -> Vec<u8> {
                    self.as_cbor()
                }

                #[wasm_bindgen(js_name="cbor")]
                pub fn as_cbor(&self) -> Vec<u8> {
                    use minicbor::CborLen;
                    let mut enc = minicbor::Encoder::new(seedle_extra::infallible_encoder::InfallibleEncoder::new(self.cbor_len(&mut ())));
                    enc.encode(self).expect("infallible!");
                    enc.into_writer().into_inner()
                }

                #[wasm_bindgen(js_name="len")]
                pub fn len(&self) -> usize {
                    use minicbor::CborLen;
                    self.cbor_len(&mut ())
                }
            }
        }
        .to_tokens(tokens);
    }
}

macro_rules! wasm_copyable {
    ($name:expr, $ty:ty) => {{
        let snaked = $name.to_snake_case();
        let member = quote::format_ident!("{}", $name);
        let js_name = quote::format_ident!("{}", $name.to_lower_camel_case());
        let fn_getter = quote::format_ident!("{}", snaked);
        let fn_setter = quote::format_ident!("set_{}", snaked);
        WasmSetterGetter {
            fn_getter,
            fn_setter,
            js_name,
            getter: quote! {self.val},
            getter_ty: quote! {$ty},
            setter: quote! {self.#member=val},
            setter_ty: quote! {$ty},
        }
    }};
}

macro_rules! wasm_str {
    ($name:expr, $len:expr) => {{
        let snaked = $name.to_snake_case();
        let member = quote::format_ident!("{}", $name);
        let js_name = quote::format_ident!("{}", $name.to_lower_camel_case());
        let fn_getter = quote::format_ident!("{}", snaked);
        let fn_setter = quote::format_ident!("set_{}", snaked);
        WasmSetterGetter {
            fn_getter,
            fn_setter,
            js_name,
            getter: quote! {
                let ascii = self.#member
                    .iter()
                    .position(|&x| x==0)
                    .map(|pos| self.#member.split_at(pos).0)
                    .unwrap_or(&self.#member);
                std::str::from_utf8(ascii)
                    .expect("invalid utf8")
                    .to_string()
            },
            setter: quote! {
                let min = core::cmp::min(val.len(), $len);
                self.#member[0..min].copy_from_slice(&val.as_bytes()[0..min]);
                self.#member[min..].fill(0);
            },
            getter_ty: quote! {String},
            setter_ty: quote! {&str},
        }
    }};
}

struct WasmSetterGetter {
    fn_getter: syn::Ident,
    fn_setter: syn::Ident,
    js_name: syn::Ident,
    getter: TokenStream,
    getter_ty: TokenStream,
    setter: TokenStream,
    setter_ty: TokenStream,
}
impl WasmSetterGetter {
    fn new(name: &str, node: &LinkedNode) -> Self {
        match node {
            LinkedNode::Primative(ConstrainedPrimative::U8) => wasm_copyable!(name, u8),
            LinkedNode::Primative(ConstrainedPrimative::U16) => wasm_copyable!(name, u16),
            LinkedNode::Primative(ConstrainedPrimative::U32) => wasm_copyable!(name, u32),
            LinkedNode::Primative(ConstrainedPrimative::U64) => wasm_copyable!(name, u64),
            LinkedNode::Primative(ConstrainedPrimative::I8) => wasm_copyable!(name, i8),
            LinkedNode::Primative(ConstrainedPrimative::I16) => wasm_copyable!(name, i16),
            LinkedNode::Primative(ConstrainedPrimative::I32) => wasm_copyable!(name, i32),
            LinkedNode::Primative(ConstrainedPrimative::I64) => wasm_copyable!(name, i64),
            LinkedNode::Primative(ConstrainedPrimative::Bool) => wasm_copyable!(name, bool),
            LinkedNode::Primative(ConstrainedPrimative::Str(n)) => wasm_str!(name, n),
            _ => unimplemented!(),
        }
    }
}
impl ToTokens for WasmSetterGetter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fn_getter = &self.fn_getter;
        let fn_setter = &self.fn_setter;
        let js_name = &self.js_name;
        let getter = &self.getter;
        let getter_ty = &self.getter_ty;
        let setter = &self.setter;
        let setter_ty = &self.setter_ty;
        quote! {
            #[wasm_bindgen(getter, js_name=#js_name)]
            pub fn #fn_getter(&self) -> #getter_ty {
                #getter
            }

            #[wasm_bindgen(setter, js_name=#js_name)]
            pub fn #fn_setter(&self, val: #setter_ty) -> &Self {
                #setter;
                self
            }
        }
        .to_tokens(tokens);
    }
}
