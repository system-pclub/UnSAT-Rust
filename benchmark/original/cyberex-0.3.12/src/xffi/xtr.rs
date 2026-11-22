use std::ptr::copy_nonoverlapping;
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn string_to_buffer(s: &str, buf: *mut u8, buf_max: usize) {
    if buf.is_null() || buf_max == 0 {
        return;
    }
    let len = std::cmp::min(buf_max - 1, s.len());

    unsafe {
        copy_nonoverlapping(s.as_ptr(), buf as *mut _, len);
        buf.add(len).write_bytes(0_u8, 1);
    }
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn string_to_dbuffer(s: &str, buf: *mut *mut u8, buf_max: *mut usize) {
    let len = s.len();

    unsafe {
        *buf_max = len;
        *buf = s.as_ptr() as *mut _;
    }
}
