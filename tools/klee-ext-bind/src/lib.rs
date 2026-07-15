//! Rust wrappers around KLEE special functions used by the ext DSL.
//!
//! [`callsite!`] marks the monitored call. KLEE obtains `get_arg(...)` and
//! `get_result()` directly from the following target LLVM `CallInst`.
//!
//! ## `klee_make_symbolic`
//!
//! Use [`make_symbolic!`] to mark a variable as symbolic inside a KLEE
//! harness.  This is equivalent to calling `klee_make_symbolic` from C.
//!
//! ## Notes
//!
//! The extern symbols are provided by the KLEE runtime during symbolic
//! execution.  This crate is intended to be compiled only for KLEE bitcode
//! generation (`./x llvmir`); the symbols are not available in normal builds.
#![no_std]


mod stubs;

use core::ffi::{c_char, c_void};

extern "C" {
    fn klee_ext_callsite(site_id: *const c_char);
    fn klee_ext_raw_pointer_deref(site_id: *const c_char, pointer: *const c_void);
    fn klee_ext_bind_arg_u64(index: u64, value: u64);

    /// KLEE special function: mark `[ptr, ptr+size)` as a symbolic value named
    /// `name`.
    fn klee_make_symbolic(ptr: *mut c_void, size: usize, name: *const c_char);
}

#[inline(always)]
pub fn callsite_raw(site_id_nul: &'static [u8]) {
    // Safety: klee_ext_callsite only records the callsite; it never dereferences
    // `site_id_nul`.  `site_id_nul` is a valid C string by the invariant above.
    let name_ptr = site_id_nul as *const [u8] as *const u8;
    unsafe { klee_ext_callsite(name_ptr as *const c_char) }
}

/// Mark the exact pointer operand of a source-level raw-pointer dereference.
///
/// Unlike [`callsite_raw`], this does not rely on finding the next LLVM load:
/// the pointer value is evaluated first and passed directly to KLEE.
#[inline(always)]
pub fn raw_pointer_deref_raw(site_id_nul: &'static [u8], pointer: *const c_void) {
    let name_ptr = site_id_nul as *const [u8] as *const u8;
    // Safety: the KLEE special function only observes the pointer expression;
    // it does not dereference either argument.
    unsafe { klee_ext_raw_pointer_deref(name_ptr as *const c_char, pointer) }
}

#[inline(always)]
pub fn bind_arg_u64(index: u64, value: u64) {
    unsafe { klee_ext_bind_arg_u64(index, value) }
}

#[macro_export]
macro_rules! callsite {
    ($site_id:literal) => {
        $crate::callsite_raw(concat!($site_id, "\0").as_bytes())
    };
}

/// Evaluate a raw pointer once, report that exact value to KLEE, and return it
/// unchanged for the surrounding `*pointer` source expression.
#[macro_export]
macro_rules! raw_pointer_deref {
    ($site_id:literal, $pointer:expr) => {{
        let __klee_raw_pointer = $pointer;
        $crate::raw_pointer_deref_raw(
            concat!($site_id, "\0").as_bytes(),
            __klee_raw_pointer as *const ::core::ffi::c_void,
        );
        __klee_raw_pointer
    }};
}

// ── klee_make_symbolic ───────────────────────────────────────────────────────

/// Mark `[ptr, ptr+size)` as symbolic under `name_nul` (null-terminated).
///
/// Prefer the [`make_symbolic!`] macro.
#[inline(always)]
pub fn make_symbolic_raw(ptr: *mut c_void, size: usize, name_nul: &'static [u8]) {
    debug_assert_eq!(
        name_nul.last().copied(),
        Some(0u8),
        "name_nul must be null-terminated"
    );
    // Safety: KLEE fills the region with a fresh symbolic value; the caller
    // ensures `ptr` points to at least `size` writable bytes.
    let name_ptr = name_nul as *const [u8] as *const u8;
    unsafe { klee_make_symbolic(ptr, size, name_ptr as *const c_char) }
}

/// Mark `$value` (passed as `&mut`) as a KLEE symbolic variable named
/// `$name`.
///
/// ```rust,ignore
/// let mut i: i32 = 0;
/// make_symbolic!(&mut i, "i");
/// ```
#[macro_export]
macro_rules! make_symbolic {
    ($value:expr, $name:literal) => {
        $crate::make_symbolic_raw(
            $value as *mut _ as *mut ::core::ffi::c_void,
            ::core::mem::size_of_val($value),
            concat!($name, "\0").as_bytes(),
        )
    };
}
