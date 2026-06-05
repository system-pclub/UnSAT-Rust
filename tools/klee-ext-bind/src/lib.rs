//! Safe Rust wrappers around KLEE special functions used by the ext DSL.
//!
//! ## `klee_ext_bind`
//!
//! Before calling the monitored callee, call [`bind!`] once per argument
//! that the DSL `task3` section maps to a selector:
//!
//! ```rust,ignore
//! // JSON task3: [{ "operator": "get_arg(0)", "expression": "p@69:41" }, ...]
//! klee_ext_bind::bind!(&p, "p@69:41");
//! klee_ext_bind::bind!(&layout, "layout@69:44");
//! unsafe { ALLOCATOR.dealloc(p, layout) }
//! ```
//!
//! KLEE's `--ext.dsl` checker resolves each `source_ref` selector by looking
//! up the registered address in `s_extBindMap` before falling back to DWARF
//! debug-info.
//!
//! ## `klee_make_symbolic`
//!
//! Use [`make_symbolic!`] to mark a variable as symbolic inside a KLEE
//! harness.  This is equivalent to calling `klee_make_symbolic` from C.
//!
//! ## Notes
//!
//! Both extern symbols are provided by the KLEE runtime during symbolic
//! execution.  This crate is intended to be compiled only for KLEE bitcode
//! generation (`./x llvmir`); the symbols are not available in normal builds.
#![no_std]

use core::ffi::{c_char, c_void};

extern "C" {
    /// KLEE special function: record `ptr` as the runtime address of `var_id`.
    ///
    /// Provided by KLEE's `SpecialFunctionHandler` when running under symbolic
    /// execution.
    fn klee_ext_bind(ptr: *const c_void, var_id: *const c_char);

    fn klee_ext_callsite(site_id: *const c_char);

    /// KLEE special function: mark `[ptr, ptr+size)` as a symbolic value named
    /// `name`.
    fn klee_make_symbolic(ptr: *mut c_void, size: usize, name: *const c_char);
}

// ── klee_ext_bind ────────────────────────────────────────────────────────────

/// Register `ptr` as the KLEE runtime address expression for selector
/// `var_id_nul`.
///
/// `var_id_nul` **must** be a null-terminated byte slice.  Prefer the
/// [`bind!`] macro which appends the NUL automatically.
#[inline(always)]
pub fn bind_raw(ptr: *const c_void, var_id_nul: &'static [u8]) {
    // Safety: klee_ext_bind only records the address; it never dereferences
    // `ptr`.  `var_id_nul` is a valid C string by the invariant above.
    unsafe { klee_ext_bind(ptr, var_id_nul.as_ptr() as *const c_char) }
}

#[inline(always)]
pub fn callsite_raw(site_id_nul: &'static [u8]) {
    // Safety: klee_ext_callsite only records the callsite; it never dereferences
    // `site_id_nul`.  `site_id_nul` is a valid C string by the invariant above.
    unsafe { klee_ext_callsite(site_id_nul.as_ptr() as *const c_char) }
}

/// Register the address of `$value` under DSL selector `$var_id`.
///
/// Pass a *reference* to the variable so KLEE receives the address of the
/// stack slot / heap field, from which it can read the variable's value:
///
/// ```rust,ignore
/// bind!(&p,      "p@69:41");
/// bind!(&layout, "layout@69:44");
/// ```
///
/// `$var_id` must be a string literal; a NUL terminator is appended by the
/// macro.
#[macro_export]
macro_rules! bind {
    ($value:expr, $var_id:literal) => {
        $crate::bind_raw(
            $value as *const _ as *const ::core::ffi::c_void,
            // concat! appends a NUL byte; as_bytes() gives &[u8] including it.
            concat!($var_id, "\0").as_bytes(),
        )
    };
}

#[macro_export]
macro_rules! callsite {
    ($site_id:literal) => {
        $crate::callsite_raw(concat!($site_id, "\0").as_bytes())
    };
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
    unsafe { klee_make_symbolic(ptr, size, name_nul.as_ptr() as *const c_char) }
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
