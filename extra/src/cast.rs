use minicbor::{Decode, Encode};

pub trait Cast: Encode<()> + for<'b> Decode<'b, ()> {
    fn cast(ptr: *const core::ffi::c_void) -> &'static Self;
    fn cast_mut(ptr: *mut core::ffi::c_void) -> &'static mut Self;
}

pub trait Dispatcher<C: Cast>: Sized {
    fn dispatch() -> C;
}
