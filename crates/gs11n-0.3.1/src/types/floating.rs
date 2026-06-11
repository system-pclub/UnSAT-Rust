use crate::decoder::DecodeContext;
use crate::meta_data::Metadata;
use crate::serialization::decoder::DecodeError;
use crate::serialization::wire_type::WireType;
use crate::serialization::{DeSerialization, Serialization, WireTypeTrait};
use crate::swap_bytes::SwapBytes;
use std::mem::{size_of, MaybeUninit};

impl SwapBytes for f32 {
    fn swap_bytes(&self) -> Self {
        unsafe {
            let ptr = self as *const Self as *const u32;
            let u = (*ptr).swap_bytes();
            let ptr = &u as *const u32 as *const Self;
            *ptr
        }
    }
}

impl SwapBytes for f64 {
    fn swap_bytes(&self) -> Self {
        unsafe {
            let ptr = self as *const Self as *const u64;
            let u = (*ptr).swap_bytes();
            let ptr = &u as *const u64 as *const Self;
            *ptr
        }
    }
}

trait FloatWireType {
    const WIRE_TYPE: WireType;
}

impl FloatWireType for f32 {
    const WIRE_TYPE: WireType = WireType::Bits32;
}

impl FloatWireType for f64 {
    const WIRE_TYPE: WireType = WireType::Bits64;
}

macro_rules! s11n_for_floating {
    ($($t:ty)*) => ($(
        impl WireTypeTrait for $t {
            const WIRE_TYPE: WireType = <Self as FloatWireType>::WIRE_TYPE;
        }
        impl Serialization for $t {
            fn encode(&self, ptr: &mut *mut u8, _meta_data: &mut Metadata) {
                   unsafe {
                       let p = *ptr;
                       let mut value = *self;
                       if cfg!(target_endian = "big") {
                           value = (*self).swap_bytes();
                       }
                       let src = &value as *const Self as *const u8;
                       let size = std::mem::size_of::<$t>();
                       std::ptr::copy_nonoverlapping(src, p, size);
                       *ptr = p.add(size);
                   }
            }

            fn record(&self, meta_data: &mut Metadata) {
                let size = size_of::<Self>() as usize;
                meta_data.size = size
            }
        }
        impl DeSerialization for $t {
            fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
                unsafe {
                    let p = *ptr;
                    let mut value = MaybeUninit::<Self>::uninit();
                    let dst : *mut u8 = value.as_mut_ptr() as *mut u8;
                    let size = std::mem::size_of::<$t>();
                    ctx.bounds_checker.check_bounds(p)?;
                    std::ptr::copy_nonoverlapping(p, dst, size);
                    let mut value = value.assume_init();
                    if cfg!(target_endian = "big") {
                        value = value.swap_bytes();
                    }
                    *ptr = p.add(size);
                    return Result::Ok(value);
                }
            }
        }
    )*)
}

s11n_for_floating!(f32 f64);
