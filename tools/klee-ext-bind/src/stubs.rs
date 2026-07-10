/// Stub (no-op) implementations of KLEE extern functions.
///
/// Enabled via `features = ["stub"]`.  Allows the crate – and crates that
/// depend on it – to link successfully in normal `cargo test` / `cargo build`
/// invocations that do not run under KLEE symbolic execution.
use core::ffi::{c_char, c_void};

#[no_mangle]
pub unsafe extern "C" fn klee_ext_callsite(_site_id: *const c_char) {}

#[no_mangle]
pub unsafe extern "C" fn klee_ext_bind_arg_u64(_index: u64, _value: u64) {}

/// Dumb stub: leaves the memory unchanged rather than making it symbolic.
#[no_mangle]
pub unsafe extern "C" fn klee_make_symbolic(
    _ptr: *mut c_void,
    _size: usize,
    _name: *const c_char,
) {
}

/// No-op Miri runtime shims used only to let `--cfg=miri` test binaries link
/// while emitting LLVM IR for KLEE. KLEE handles these as externals/no-ops when
/// interpreting the linked IR.
#[no_mangle]
pub unsafe extern "C" fn miri_promise_symbolic_alignment(_ptr: *const c_void, _align: usize) {}

#[no_mangle]
pub unsafe extern "C" fn miri_resolve_frame(
    _out: *mut c_void,
    _ptr: *const c_void,
    _flags: usize,
) {
}

#[no_mangle]
pub unsafe extern "C" fn miri_resolve_frame_names(
    _ptr: *const c_void,
    _flags: usize,
    _name: *mut c_void,
    _filename: *mut c_void,
) {
}

#[no_mangle]
pub unsafe extern "C" fn miri_backtrace_size(_flags: usize) -> usize {
    0
}

#[no_mangle]
pub unsafe extern "C" fn miri_get_backtrace(_flags: usize, _out: *mut c_void) {}
