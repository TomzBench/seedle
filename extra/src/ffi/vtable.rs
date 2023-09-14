use core::ffi::c_void;

pub type EncodeFn<T> = extern "C" fn(*mut u8, u32, &T) -> i32;
pub type EncodeFnErased = extern "C" fn(*mut u8, u32, &c_void) -> i32;
pub type EncodeArrayFn<T> = extern "C" fn(*mut u8, u32, &T, u32) -> i32;
pub type EncodeArrayFnErased = extern "C" fn(*mut u8, u32, &c_void, u32) -> i32;
pub type DecodeFn<T> = extern "C" fn(&mut T, *const u8, u32) -> i32;
pub type DecodeFnErased = extern "C" fn(&mut c_void, *const u8, u32) -> i32;
pub type DecodeArrayFn<T> = extern "C" fn(&mut T, u32, *const u8, u32) -> i32;
pub type DecodeArrayFnErased = extern "C" fn(&mut c_void, u32, *const u8, u32) -> i32;
pub type LenFn<T> = extern "C" fn(&T) -> u32;
pub type LenFnErased = extern "C" fn(&c_void) -> u32;
pub type ArrayLenFn<T> = extern "C" fn(&T, u32) -> u32;
pub type ArrayLenFnErased = extern "C" fn(&c_void, u32) -> u32;
pub struct SeedleVTableEntry {
    pub encode: EncodeFnErased,
    pub encode_array: EncodeArrayFnErased,
    pub decode: DecodeFnErased,
    pub decode_array: DecodeArrayFnErased,
    pub len: LenFnErased,
    pub array_len: ArrayLenFnErased,
}
