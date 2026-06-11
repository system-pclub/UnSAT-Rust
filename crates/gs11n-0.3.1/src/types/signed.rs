use super::unsigned::EncodeSize;
use crate::decoder::DecodeContext;
use crate::meta_data::Metadata;
use crate::serialization::decoder::DecodeError;
use crate::serialization::wire_type::WireType;
use crate::serialization::{DeSerialization, Serialization, WireTypeTrait};
use std::mem::size_of;

pub trait ZigZag {
    type UnsignedTy;
    fn zigzag(&self) -> Self::UnsignedTy;
}

pub trait UnZigZag {
    type SignedTy;
    fn unzigzag(&self) -> Self::SignedTy;
}

macro_rules! impl_zigzag_for {
    ($signed:ty , $unsigned:ty) => {
        impl ZigZag for $signed {
            type UnsignedTy = $unsigned;
            fn zigzag(&self) -> Self::UnsignedTy {
                ((*self << 1) ^ (*self >> (size_of::<$signed>() * 8 - 1))) as Self::UnsignedTy
            }
        }

        impl UnZigZag for $unsigned {
            type SignedTy = $signed;
            fn unzigzag(&self) -> Self::SignedTy {
                ((*self >> 1) ^ ((!(*self & 1)).wrapping_add(1))) as Self::SignedTy
            }
        }
    };
}

impl_zigzag_for!(i8, u8);
impl_zigzag_for!(i16, u16);
impl_zigzag_for!(i32, u32);
impl_zigzag_for!(i64, u64);
impl_zigzag_for!(isize, usize);
#[cfg(has_i128)]
impl_zigzag_for!(i128, u128);

macro_rules! s11n_for_signed {
        ($($t:ty)*) => ($(
        impl WireTypeTrait for $t {
            const WIRE_TYPE: WireType = WireType::Varint;
        }
        impl Serialization for $t {
            fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
                let zigzag = (*self).zigzag();
                zigzag.encode(ptr, meta_data)
            }

            fn record(&self, meta_data: &mut Metadata) {
                let zigzag = (*self).zigzag();
                let size = zigzag.varint_size();
                meta_data.size = size
            }
        }
        impl DeSerialization for $t {
            fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
                let zigzag = <Self as ZigZag>::UnsignedTy::decode(ptr, ctx)?;
                let value = zigzag.unzigzag();
                Result::Ok(value)
            }
        }
    )*)
}

s11n_for_signed!(isize i8 i16 i32 i64);
#[cfg(has_i128)]
s11n_for_signed!(i128);
