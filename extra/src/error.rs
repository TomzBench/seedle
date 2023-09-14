struct ErrMsg<'a>(&'a mut [u8]);
impl<'a> core::fmt::Write for ErrMsg<'a> {
    fn write_str(&mut self, msg: &str) -> core::fmt::Result {
        let min = core::cmp::min(self.0.len() - 1, msg.len());
        self.0[0..min].copy_from_slice(&msg[0..min].as_bytes());
        self.0[min..].fill(0);
        Ok(())
    }
}

#[inline]
#[cfg_attr(not(test), allow(unused_must_use))]
#[cfg_attr(test, allow(unused))]
pub fn decode_error_msg(e: minicbor::decode::Error, errmsg: *mut u8, errmsg_len: &mut u32) -> i32 {
    let err = unsafe { core::slice::from_raw_parts_mut(errmsg, *errmsg_len as usize) };
    core::fmt::write(&mut ErrMsg(err), format_args!("{}", e));
    *errmsg_len = err.len() as u32;
    -1
}
