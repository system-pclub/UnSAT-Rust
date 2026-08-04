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
    fn klee_assume(condition: usize);
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

pub trait RawPointerValue: Copy {
    fn as_const_void(self) -> *const c_void;
}

impl<T> RawPointerValue for *const T {
    #[inline(always)]
    fn as_const_void(self) -> *const c_void {
        self.cast::<c_void>()
    }
}

impl<T> RawPointerValue for *mut T {
    #[inline(always)]
    fn as_const_void(self) -> *const c_void {
        self.cast::<c_void>() as *const c_void
    }
}

#[inline(always)]
pub fn raw_pointer_deref_typed<P: RawPointerValue>(
    site_id_nul: &'static [u8],
    pointer: P,
) -> P {
    raw_pointer_deref_raw(site_id_nul, pointer.as_const_void());
    pointer
}

#[inline(always)]
pub fn bind_arg_u64(index: u64, value: u64) {
    unsafe { klee_ext_bind_arg_u64(index, value) }
}

pub trait ArgU64Value {
    fn as_arg_u64(&self) -> Option<u64>;
}

pub trait ArgRustRangeValue {
    /// Return a half-open range `[start, end)` in slice-index units.
    ///
    /// `len` is the receiver slice length, used for open-ended range forms
    /// such as `start..` and `..`.
    fn as_rustrange_u64(&self, len: u64) -> Option<(u64, u64)>;
}

macro_rules! impl_unsigned_arg_u64 {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ArgU64Value for $ty {
                #[inline(always)]
                fn as_arg_u64(&self) -> Option<u64> {
                    Some(*self as u64)
                }
            }
        )*
    };
}

macro_rules! impl_signed_arg_u64 {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ArgU64Value for $ty {
                #[inline(always)]
                fn as_arg_u64(&self) -> Option<u64> {
                    if *self >= 0 {
                        Some(*self as u64)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

impl_unsigned_arg_u64!(usize, u8, u16, u32, u64);
impl_signed_arg_u64!(isize, i8, i16, i32, i64);

impl ArgRustRangeValue for usize {
    #[inline(always)]
    fn as_rustrange_u64(&self, _len: u64) -> Option<(u64, u64)> {
        let start = *self as u64;
        Some((start, start.checked_add(1).unwrap_or(u64::MAX)))
    }
}

impl ArgU64Value for core::ops::Range<usize> {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(self.start as u64)
    }
}

impl ArgRustRangeValue for core::ops::Range<usize> {
    #[inline(always)]
    fn as_rustrange_u64(&self, _len: u64) -> Option<(u64, u64)> {
        Some((self.start as u64, self.end as u64))
    }
}

impl ArgU64Value for core::ops::RangeFrom<usize> {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(self.start as u64)
    }
}

impl ArgRustRangeValue for core::ops::RangeFrom<usize> {
    #[inline(always)]
    fn as_rustrange_u64(&self, len: u64) -> Option<(u64, u64)> {
        Some((self.start as u64, len))
    }
}

impl ArgU64Value for core::ops::RangeTo<usize> {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(0)
    }
}

impl ArgRustRangeValue for core::ops::RangeTo<usize> {
    #[inline(always)]
    fn as_rustrange_u64(&self, _len: u64) -> Option<(u64, u64)> {
        Some((0, self.end as u64))
    }
}

impl ArgU64Value for core::ops::RangeFull {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(0)
    }
}

impl ArgRustRangeValue for core::ops::RangeFull {
    #[inline(always)]
    fn as_rustrange_u64(&self, len: u64) -> Option<(u64, u64)> {
        Some((0, len))
    }
}

impl ArgU64Value for core::ops::RangeInclusive<usize> {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(*self.start() as u64)
    }
}

impl ArgRustRangeValue for core::ops::RangeInclusive<usize> {
    #[inline(always)]
    fn as_rustrange_u64(&self, _len: u64) -> Option<(u64, u64)> {
        let start = *self.start() as u64;
        let end = (*self.end() as u64).checked_add(1)?;
        Some((start, end))
    }
}

impl ArgU64Value for core::ops::RangeToInclusive<usize> {
    #[inline(always)]
    fn as_arg_u64(&self) -> Option<u64> {
        Some(0)
    }
}

impl ArgRustRangeValue for core::ops::RangeToInclusive<usize> {
    #[inline(always)]
    fn as_rustrange_u64(&self, _len: u64) -> Option<(u64, u64)> {
        Some((0, (self.end as u64).checked_add(1)?))
    }
}

#[inline(always)]
pub fn bind_arg_u64_value<T: ArgU64Value + ?Sized>(index: u64, value: &T) {
    if let Some(value) = value.as_arg_u64() {
        bind_arg_u64(index, value);
    }
}

#[inline(always)]
pub fn bind_arg_rustrange_value<T: ArgRustRangeValue + ?Sized>(
    start_index: u64,
    value: &T,
    len: u64,
) {
    if let Some((start, end)) = value.as_rustrange_u64(len) {
        bind_arg_u64(start_index, start);
        bind_arg_u64(start_index + 1, end);
    }
}

pub trait ArgPtrValue {
    fn as_arg_ptr_u64(self) -> u64;
}

impl<T> ArgPtrValue for *const T {
    #[inline(always)]
    fn as_arg_ptr_u64(self) -> u64 {
        self as usize as u64
    }
}

impl<T> ArgPtrValue for *mut T {
    #[inline(always)]
    fn as_arg_ptr_u64(self) -> u64 {
        self as usize as u64
    }
}

impl<T> ArgPtrValue for &*const T {
    #[inline(always)]
    fn as_arg_ptr_u64(self) -> u64 {
        *self as usize as u64
    }
}

impl<T> ArgPtrValue for &*mut T {
    #[inline(always)]
    fn as_arg_ptr_u64(self) -> u64 {
        *self as usize as u64
    }
}

#[inline(always)]
pub fn bind_arg_ptr_value<T: ArgPtrValue>(index: u64, value: T) {
    bind_arg_u64(index, value.as_arg_ptr_u64());
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
    ($site_id:literal, $pointer:expr) => {
        $crate::raw_pointer_deref_typed(concat!($site_id, "\0").as_bytes(), $pointer)
    };
}

// ── klee_make_symbolic ───────────────────────────────────────────────────────

/// Mark `[ptr, ptr+size)` as symbolic under `name_nul` (null-terminated).
///
/// Prefer the [`make_symbolic!`] macro.
#[inline(always)]
pub fn make_symbolic_raw(ptr: *mut c_void, size: usize, name_nul: &'static [u8]) {
    // Safety: KLEE fills the region with a fresh symbolic value; the caller
    // ensures `ptr` points to at least `size` writable bytes.
    // The public macro passes `concat!($name, "\0").as_bytes()`, so `name_nul`
    // is null-terminated without needing a debug assertion that would pull
    // generic Option helpers into the KLEE bitcode.
    let name_ptr = name_nul as *const [u8] as *const u8;
    unsafe { klee_make_symbolic(ptr, size, name_ptr as *const c_char) }
}

#[inline(always)]
pub fn assume_raw(condition: bool) {
    unsafe { klee_assume(if condition { 1 } else { 0 }) }
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

#[macro_export]
macro_rules! assume {
    ($condition:expr) => {
        $crate::assume_raw($condition)
    };
}
