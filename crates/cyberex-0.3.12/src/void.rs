#![allow(dead_code)]

use std::{os::raw::c_void, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct HyVoid<T> {
    ptr: *mut T,
}
unsafe impl<T> Send for HyVoid<T> where T: Send {}
unsafe impl<T> Sync for HyVoid<T> where T: Sync {}

impl<T> HyVoid<T> {
    pub fn from_ref(r: &mut T) -> Self {
        Self {
            ptr: NonNull::from(r).as_ptr().cast(),
        }
    }
    pub fn from_ptr(ptr: *mut c_void) -> Self {
        Self { ptr: ptr.cast() }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.cast()
    }
    pub fn as_dptr(&mut self) -> *mut *mut c_void {
        std::ptr::addr_of_mut!(self.ptr).cast()
    }
}
impl<T> AsMut<T> for HyVoid<T> {
    fn as_mut(&mut self) -> &mut T {
        opacue_to_mut(self.ptr)
    }
}
impl<T> AsRef<T> for HyVoid<T> {
    fn as_ref(&self) -> &T {
        opacue_to_ref(self.ptr)
    }
}
#[derive(Clone, Copy)]
pub struct HyVoidConst<T> {
    ptr: *const T,
}
unsafe impl<T> Send for HyVoidConst<T> where T: Send {}
unsafe impl<T> Sync for HyVoidConst<T> where T: Sync {}

impl<T> HyVoidConst<T> {
    pub fn from_ref(r: &T) -> Self {
        Self {
            ptr: NonNull::from(r).as_ptr().cast(),
        }
    }
    pub fn from_ptr(ptr: *const c_void) -> Self {
        Self { ptr: ptr.cast() }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr.cast()
    }
    pub fn as_dptr(&mut self) -> *const *const c_void {
        std::ptr::addr_of!(self.ptr).cast()
    }
}
impl<T> AsRef<T> for HyVoidConst<T> {
    fn as_ref(&self) -> &T {
        opacue_to_ref(self.ptr)
    }
}
pub fn opacue_to_mut<'a, T>(user: *mut T) -> &'a mut T {
    if user.is_null() {
        panic!("Pointer is null")
    }
    unsafe { &mut *(user.cast()) as &mut T }
}
pub fn opacue_to_ref<'a, T>(user: *const T) -> &'a T {
    unsafe { &*(user.cast()) as &T }
}

pub fn of_addr<'a, T>(user: *const T) -> &'a T {
    opacue_to_ref(user)
}

pub fn of_mut_addr<'a, T>(user: *mut T) -> &'a mut T {
    opacue_to_mut(user)
}
pub fn mut_to_opacue<T>(r: &mut T) -> *mut c_void {
    r as *const _ as *mut _
}

pub fn ref_to_opacue<T>(r: &T) -> *const c_void {
    r as *const _ as *const _
}
pub fn delete<T>(ctx: *mut c_void) {
    drop(unsafe { Box::from_raw(ctx as *mut _ as *mut T) });
}
pub fn new<T>(t: T) -> *mut c_void {
    unsafe { &mut *(Box::into_raw(Box::new(t)) as *mut c_void) }
}

pub fn new_and_then<T, F>(t: T, op: F) -> Result<*mut c_void, anyhow::Error>
where
    F: FnOnce(&mut T) -> Result<(), anyhow::Error>,
{
    let mut b = Box::new(t);

    match op(b.as_mut()) {
        Ok(_) => Ok(unsafe { &mut *(Box::into_raw(b) as *mut c_void) }),
        Err(e) => Err(e),
    }
}
