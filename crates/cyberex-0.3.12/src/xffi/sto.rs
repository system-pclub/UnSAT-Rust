use std::{ffi::CStr, os::raw::c_char};
#[allow(clippy::needless_range_loop)]
pub fn string_to_array<const COUNT: usize>(s: &str) -> [c_char; COUNT] {
    let mut a = [0 as c_char; COUNT];
    let len = std::cmp::min(a.len() - 1, s.len());
    for i in 0..len {
        a[i] = s.as_bytes()[i] as c_char;
    }
    a
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn cchar_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn string_to_buffer(s: &str, buf: *mut u8, buf_max: usize) {
    crate::xffi::xtr::string_to_buffer(s, buf, buf_max)
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn string_to_dbuffer(s: &str, buf: *mut *mut u8, buf_max: *mut usize) {
    crate::xffi::xtr::string_to_dbuffer(s, buf, buf_max)
}
