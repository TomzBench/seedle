pub(crate) mod macros;
mod vtable;
use crate::cast::*;
#[cfg(feature = "ffi_ts")]
use wasm_bindgen::prelude::*;

pub use vtable::*;

include!(concat!(env!("OUT_DIR"), "/extra.rs"));

pub fn cbor_dec_slice<D>(
    mut dst: *mut core::ffi::c_void,
    dstlen: u32,
    src: *const u8,
    srclen: u32,
) -> Result<i32, minicbor::decode::Error>
where
    D: minicbor::CborLen<()> + for<'b> minicbor::Decode<'b, ()>,
{
    let slice = unsafe { core::slice::from_raw_parts(src, srclen as usize) };
    let mut decoder = minicbor::Decoder::new(slice);
    let len = decoder
        .probe()
        .array()?
        .ok_or_else(|| minicbor::decode::Error::message("expected fixed length array!"))?;
    let mut i = 0;
    for decoded in decoder.array_iter::<D>()? {
        if i == dstlen {
            break;
        }
        unsafe {
            *(dst as *mut D) = decoded?;
            dst = dst.add(core::mem::size_of::<D>());
        }
        i = i + 1;
    }
    Ok(len as i32)
}
#[cfg(test)]
mod tests {

    #[cfg(feature = "ffi_ts")]
    mod ts {
        use crate::ffi::*;
        #[test]
        fn test_mcbor_encode_str_ts() {
            let hello = minicbor_encode_str_ts("hello".to_owned());
            assert_eq!("hello", minicbor_decode_str_ts(&hello).unwrap());
        }

        #[test]
        fn test_mcbor_encode_num_ts() {
            let hello = minicbor_encode_num_ts(42);
            assert_eq!(42, minicbor_decode_num_ts(&hello).unwrap());
        }
    }

    #[cfg(feature = "ffi_c")]
    mod c {
        use crate::ffi::*;
        use minicbor::bytes::ByteSlice;
        use minicbor::Encoder;
        #[test]
        fn test_mcbor_encode_num() {
            let mut actual: [u8; 1] = [0; 1];
            let mut expect: [u8; 1] = [0; 1];
            let ret = cbor_encode_num(actual.as_mut_ptr(), actual.len() as u32, 2);
            Encoder::new(expect.as_mut()).i64(2).unwrap();
            assert_eq!(1, ret);
            assert_eq!(expect, actual);
        }

        #[test]
        fn test_mcbor_encode_bytes() {
            let dat = vec![0, 1, 2, 3];
            let enclen = cbor_len_bytes(dat.as_ptr(), dat.len() as u32);
            let mut actual = vec![0; enclen as usize];
            let ret = cbor_encode_bytes(
                actual.as_mut_ptr(),
                actual.len() as u32,
                dat.as_ptr(),
                dat.len() as u32,
            );
            assert_eq!(enclen, ret as u32);
            assert_eq!(
                vec![0, 1, 2, 3],
                minicbor::decode::<&ByteSlice>(actual.as_ref())
                    .unwrap()
                    .to_vec()
            );
        }

        #[test]
        fn test_mcbor_encode_str() {
            let dat = vec![b'h', b'e', b'l', b'l', b'o', b'\0'];
            let enclen = cbor_len_str(dat.as_ptr() as *const i8);
            let mut actual = vec![0; enclen as usize];
            let ret = cbor_encode_str(
                actual.as_mut_ptr(),
                actual.len() as u32,
                dat.as_ptr() as *const i8,
            );
            assert_eq!(enclen, ret as i32);
            assert_eq!("hello", minicbor::decode::<&str>(actual.as_ref()).unwrap());
        }

        #[test]
        fn test_mcbor_encode_fixed_array() {
            let mut actual: [u8; 3] = [0; 3];
            let ret = cbor_encode_fixed_array(actual.as_mut_ptr(), 3, 2);
            assert_eq!(1, ret);
            let ret = cbor_encode_num(actual[1..].as_mut_ptr(), 2, 4);
            assert_eq!(1, ret);
            let ret = cbor_encode_num(actual[2..].as_mut_ptr(), 1, 2);
            assert_eq!(1, ret);
            assert_eq!([4, 2], minicbor::decode::<[u8; 2]>(&actual).unwrap());
        }

        #[test]
        fn test_mcbor_encode_map() {
            let mut actual: [u8; 3] = [0; 3];
            let ret = cbor_encode_map(actual.as_mut_ptr(), 3, 2);
            assert_eq!(1, ret);
            let ret = cbor_encode_num(actual[1..].as_mut_ptr(), 2, 4);
            assert_eq!(1, ret);
            let ret = cbor_encode_num(actual[2..].as_mut_ptr(), 1, 2);
            assert_eq!(1, ret);
            let mut decoder = minicbor::decode::Decoder::new(&actual);
            assert!(decoder.map().is_ok());
            assert_eq!(4, decoder.u8().unwrap());
            assert_eq!(2, decoder.u8().unwrap());
        }

        #[test]
        fn test_mcbor_decode_bool() {
            let mut buf = [0; 1];
            let mut uut = false;
            let ret = cbor_encode_bool(buf.as_mut_ptr(), 1, true);
            assert_eq!(1, ret);
            let ret = cbor_decode_bool(&mut uut as *mut bool, buf.as_mut_ptr(), 1);
            assert_eq!(1, ret);
            assert_eq!(true, uut);

            let ret = cbor_encode_bool(buf.as_mut_ptr(), 1, false);
            assert_eq!(1, ret);
            let ret = cbor_decode_bool(&mut uut as *mut bool, buf.as_mut_ptr(), 1);
            assert_eq!(1, ret);
            assert_eq!(false, uut);
        }

        #[test]
        fn test_mcbor_decode_map() {
            let mut buf = [0; 1];
            let ret = cbor_encode_map(buf.as_mut_ptr(), 1, 2);
            assert_eq!(1, ret);

            let ret = cbor_decode_map(buf.as_mut_ptr(), 1);
            assert_eq!(2, ret);
        }

        #[test]
        fn test_mcbor_decode_fixed_array() {
            let mut buf = [0; 1];
            let ret = cbor_encode_fixed_array(buf.as_mut_ptr(), 1, 2);
            assert_eq!(1, ret);

            let ret = cbor_decode_fixed_array(buf.as_mut_ptr(), 1);
            assert_eq!(2, ret);
        }

        #[test]
        fn test_mcbor_decode_bytes() {
            let mut actual = [0; 3];
            let mut data = [0; 4];
            let mut encoder = Encoder::new(data.as_mut());
            encoder.bytes(&[0, 1, 2]).unwrap();
            let ret = cbor_decode_bytes(actual.as_mut_ptr(), 3, data.as_ptr(), 4);
            assert_eq!(3, ret);
            assert_eq!([0, 1, 2], actual);
        }

        #[test]
        fn test_mcbor_decode_str() {
            let mut actual = [0; 3];
            let mut data = [0; 4];
            let mut encoder = Encoder::new(data.as_mut());
            encoder.str("hii").unwrap();
            let ret = cbor_decode_str(actual.as_mut_ptr(), 3, data.as_ptr(), 4);
            assert_eq!(3, ret);
            assert_eq!("hii".as_bytes(), actual);
        }
    }
}
