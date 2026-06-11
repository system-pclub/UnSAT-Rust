use crate::decoder::DecodeContext;
use crate::meta_data::Metadata;
use crate::serialization::decoder::DecodeError;
use crate::serialization::wire_type::WireType;
use crate::serialization::{DeSerialization, Serialization, WireTypeTrait};
use crate::swap_bytes::SwapBytes;
use std::mem::{size_of, MaybeUninit};

impl SwapBytes for char {
    fn swap_bytes(&self) -> Self {
        unsafe {
            let ptr = self as *const char as *const u32;
            let u = (*ptr).swap_bytes();
            let ptr = &u as *const u32 as *const char;
            *ptr
        }
    }
}

impl WireTypeTrait for char {
    const WIRE_TYPE: WireType = WireType::Bits32;
}

impl Serialization for char {
    fn encode(&self, ptr: &mut *mut u8, _meta_data: &mut Metadata) {
        unsafe {
            let p = *ptr;
            let mut value = *self;
            if cfg!(target_endian = "big") {
                value = (*self).swap_bytes();
            }
            let src = &value as *const char as *const u8;
            let size = size_of::<char>();
            std::ptr::copy_nonoverlapping(src, p, size);
            *ptr = p.add(size);
        }
    }

    fn record(&self, meta_data: &mut Metadata) {
        let size = size_of::<Self>() as usize;
        meta_data.size = size
    }
}

impl DeSerialization for char {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        unsafe {
            let p = *ptr;
            let mut value = MaybeUninit::<char>::uninit();
            let dst: *mut u8 = value.as_mut_ptr() as *mut u8;
            let size = std::mem::size_of::<char>();
            ctx.bounds_checker.check_bounds(p)?;
            std::ptr::copy_nonoverlapping(p, dst, size);
            let mut value = value.assume_init();
            if cfg!(target_endian = "big") {
                value = value.swap_bytes();
            }
            *ptr = p.add(size);
            Result::Ok(value)
        }
    }
}
