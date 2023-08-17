macro_rules! define_encode_len {
    ($fn:ident, $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $fn(val: $ty) -> u32 {
            minicbor::encode::CborLen::cbor_len(&val, &mut ()) as u32
        }
    };
}

macro_rules! define_encode {
    ($fn:ident, $meth:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32) -> i32 {
            let dstslice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let mut enc =
                minicbor::Encoder::new(minicbor::encode::write::Cursor::new(dstslice.as_mut()));
            minicbor::Encoder::$meth(&mut enc).map_or(-1, |enc| enc.writer().position() as i32)
        }
    };
    ($fn:ident, $meth:ident, $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32, val: $ty) -> i32 {
            let dstslice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let mut enc =
                minicbor::Encoder::new(minicbor::encode::write::Cursor::new(dstslice.as_mut()));
            minicbor::Encoder::$meth(&mut enc, val.into())
                .map_or(-1, |enc| enc.writer().position() as i32)
        }
    };
}

macro_rules! define_decode {
    ($fn:ident, $meth:ident, $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut $ty, src: *mut u8, srclen: u32) -> i32 {
            let srcslice = unsafe { core::slice::from_raw_parts_mut(src, srclen as usize) };
            let mut dec = minicbor::Decoder::new(srcslice);
            if let Ok(b) = minicbor::Decoder::$meth(&mut dec) {
                unsafe { *dst = b };
                dec.position() as i32
            } else {
                -1
            }
        }
    };
}

macro_rules! define_decode_group {
    ( $fn:ident, $meth:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(src: *mut u8, srclen: u32) -> i32 {
            let slice = unsafe { core::slice::from_raw_parts(src as *const u8, srclen as usize) };
            let mut decoder = minicbor::Decoder::new(slice);
            match minicbor::Decoder::$meth(&mut decoder) {
                Ok(Some(val)) => val as i32,
                Ok(None) => 0,
                Err(_) => -1,
            }
        }
    };
}

macro_rules! define_encode_str {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32, src: *const i8) -> i32 {
            let slice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let src = unsafe { core::ffi::CStr::from_ptr(src) };
            if let Ok(src) = src.to_str() {
                let mut encoder =
                    minicbor::Encoder::new(minicbor::encode::write::Cursor::new(slice.as_mut()));
                encoder
                    .str(src)
                    .map_or(-1, |encoder| encoder.writer().position() as i32)
            } else {
                -1
            }
        }
    };
}

macro_rules! define_encode_str_len {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(src: *const i8) -> i32 {
            let src = unsafe { core::ffi::CStr::from_ptr(src) };
            if let Ok(src) = src.to_str() {
                minicbor::encode::CborLen::cbor_len(src, &mut ()) as i32
            } else {
                -1
            }
        }
    };
}

macro_rules! define_encode_bytes {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32, src: *const u8, srclen: u32) -> i32 {
            let dstslice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let srcslice = unsafe { core::slice::from_raw_parts(src, srclen as usize) };
            let mut enc =
                minicbor::Encoder::new(minicbor::encode::write::Cursor::new(dstslice.as_mut()));
            enc.bytes(srcslice)
                .map_or(-1, |enc| enc.writer().position() as i32)
        }
    };
}

macro_rules! define_encode_bytes_len {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(src: *const u8, srclen: u32) -> u32 {
            let slice = unsafe { core::slice::from_raw_parts(src, srclen as usize) };
            minicbor::encode::CborLen::cbor_len(slice, &mut ()) as u32
        }
    };
}

macro_rules! define_decode_str {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32, src: *const u8, srclen: u32) -> i32 {
            let dstslice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let srcslice = unsafe { core::slice::from_raw_parts(src, srclen as usize) };
            let mut decoder = minicbor::Decoder::new(srcslice);
            if let Ok(bytes) = decoder.str() {
                if bytes.len() <= dstslice.len() {
                    dstslice[0..bytes.len()].copy_from_slice(bytes.as_bytes());
                    bytes.len() as i32
                } else {
                    -1
                }
            } else {
                -1
            }
        }
    };
}

macro_rules! define_decode_bytes {
    ($fn:ident) => {
        #[no_mangle]
        pub extern "C" fn $fn(dst: *mut u8, dstlen: u32, src: *const u8, srclen: u32) -> i32 {
            let dstslice = unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) };
            let srcslice = unsafe { core::slice::from_raw_parts(src, srclen as usize) };
            let mut decoder = minicbor::Decoder::new(srcslice);
            if let Ok(bytes) = decoder.bytes() {
                if bytes.len() <= dstslice.len() {
                    dstslice[0..bytes.len()].copy_from_slice(bytes);
                    bytes.len() as i32
                } else {
                    -1
                }
            } else {
                -1
            }
        }
    };
}

macro_rules! define_encode_str_ts {
    ($name:expr) => {
        #[wasm_bindgen(js_name = $name)]
        pub fn minicbor_encode_str_ts(s: String) -> Vec<u8> {
            let len = minicbor::encode::CborLen::cbor_len(s.as_str(), &mut ());
            let mut enc =
                minicbor::Encoder::new(crate::infallible_encoder::InfallibleEncoder::new(len));
            enc.encode(s.as_str()).expect("infallible!");
            enc.into_writer().into_inner()
        }
    };
}

macro_rules! define_decode_str_ts {
    ($name:expr) => {
        #[wasm_bindgen(js_name = $name)]
        pub fn minicbor_decode_str_ts(cbor: &[u8]) -> Result<String, JsValue> {
            let mut dec = minicbor::Decoder::new(cbor);
            dec.str()
                .map(|s| s.to_string())
                .map_err(|e| JsValue::from(e.to_string()))
        }
    };
}

macro_rules! define_encode_num_ts {
    ($name:expr) => {
        #[wasm_bindgen(js_name = $name)]
        pub fn minicbor_encode_num_ts(i: i64) -> Vec<u8> {
            let len = minicbor::encode::CborLen::cbor_len(&i, &mut ());
            let mut enc =
                minicbor::Encoder::new(crate::infallible_encoder::InfallibleEncoder::new(len));
            enc.encode(&i).expect("infallible!");
            enc.into_writer().into_inner()
        }
    };
}

macro_rules! define_decode_num_ts {
    ($name:expr) => {
        #[wasm_bindgen(js_name = $name)]
        pub fn minicbor_decode_num_ts(cbor: &[u8]) -> Result<i64, JsValue> {
            let mut dec = minicbor::Decoder::new(cbor);
            dec.decode().map_err(|e| JsValue::from(e.to_string()))
        }
    };
}

macro_rules! extra {
    ("C", $prefix:expr) => {
        paste::paste! {
            crate::ffi::macros::define_encode_len!([<$prefix _len_num>], i64);
            crate::ffi::macros::define_encode_len!([<$prefix _len_bool>], bool);
            crate::ffi::macros::define_encode!([<$prefix _encode_num>], i64, i64);
            crate::ffi::macros::define_encode!([<$prefix _encode_bool>], bool, bool);
            crate::ffi::macros::define_encode!([<$prefix _encode_fixed_array>], array, u32);
            crate::ffi::macros::define_encode!([<$prefix _encode_map>], map, u32);
            crate::ffi::macros::define_decode!([<$prefix _decode_num>], i64, i64);
            crate::ffi::macros::define_decode!([<$prefix _decode_bool>], bool, bool);
            crate::ffi::macros::define_decode_group!([<$prefix _decode_fixed_array>], array);
            crate::ffi::macros::define_decode_group!([<$prefix _decode_map>], map);
            crate::ffi::macros::define_encode_str!([<$prefix _encode_str>]);
            crate::ffi::macros::define_encode_str_len!([<$prefix _len_str>]);
            crate::ffi::macros::define_encode_bytes!([<$prefix _encode_bytes>]);
            crate::ffi::macros::define_encode_bytes_len!([<$prefix _len_bytes>]);
            crate::ffi::macros::define_decode_str!([<$prefix _decode_str>]);
            crate::ffi::macros::define_decode_bytes!([<$prefix _decode_bytes>]);
        }
    };
    ("TS", $prefix:ident) => {
        paste::paste! {
            crate::ffi::macros::define_encode_str_ts!([<$prefix EncodeStr>]);
            crate::ffi::macros::define_decode_str_ts!([<$prefix DecodeStr>]);
            crate::ffi::macros::define_encode_num_ts!([<$prefix EncodeNum>]);
            crate::ffi::macros::define_decode_num_ts!([<$prefix DecodeNum>]);
        }
    };
}
pub(crate) use define_decode;
pub(crate) use define_decode_bytes;
pub(crate) use define_decode_group;
pub(crate) use define_decode_str;
pub(crate) use define_encode;
pub(crate) use define_encode_bytes;
pub(crate) use define_encode_bytes_len;
pub(crate) use define_encode_len;
pub(crate) use define_encode_str;
pub(crate) use define_encode_str_len;
pub(crate) use define_encode_num_ts;
pub(crate) use define_decode_num_ts;
pub(crate) use define_decode_str_ts;
pub(crate) use define_encode_str_ts;
pub(crate) use extra;
