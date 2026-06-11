/// Stub (no-op) implementations of KLEE extern functions.
///
/// Enabled via `features = ["stub"]`.  Allows the crate – and crates that
/// depend on it – to link successfully in normal `cargo test` / `cargo build`
/// invocations that do not run under KLEE symbolic execution.
use core::ffi::{c_char, c_void};

#[no_mangle]
pub unsafe extern "C" fn klee_ext_bind(_ptr: *const c_void, _var_id: *const c_char) {}

#[no_mangle]
pub unsafe extern "C" fn klee_ext_callsite(_site_id: *const c_char) {}

/// Dumb stub: leaves the memory unchanged rather than making it symbolic.
#[no_mangle]
pub unsafe extern "C" fn klee_make_symbolic(
    _ptr: *mut c_void,
    _size: usize,
    _name: *const c_char,
) {
}
