use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::unsigned::EncodeSize;
use crate::wire_type::WireType;
use crate::{DeSerialization, Serialization, WireTypeTrait};
use std::mem::{size_of, MaybeUninit};

fn encode_slice<S: Serialization>(value: &[S], ptr: &mut *mut u8, meta_data: &mut Metadata) {
    // encode size first
    value.len().encode_raw(ptr);

    if (S::WIRE_TYPE.is_fixed_type() && S::WIRE_TYPE == WireType::Bits8)
        || (S::WIRE_TYPE.is_fixed_type() && !cfg!(target_endian = "big"))
    {
        unsafe {
            let mut p = *ptr;
            std::ptr::copy_nonoverlapping(value.as_ptr(), p as *mut S, value.len());
            p = ((p as *mut S).add(value.len())) as *mut u8;
            *ptr = p;
        }
    } else {
        for i in 0..value.len() {
            value[i].encode(ptr, meta_data.get(i));
        }
    }
}

fn record_slice<S: Serialization>(value: &[S], meta_data: &mut Metadata) {
    let mut size = value.len().varint_size();
    if S::WIRE_TYPE.is_fixed_type() {
        size += value.len() * std::mem::size_of::<S>();
    } else {
        for i in 0..value.len() {
            value[i].record(meta_data.get(i));
            size += meta_data.get(i).size;
        }
    }
    meta_data.size = size;
}

impl<S> WireTypeTrait for Vec<S> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<S> WireTypeTrait for &[S] {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<S, const N: usize> WireTypeTrait for [S; N] {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<S: Serialization> Serialization for &[S] {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        encode_slice(self, ptr, meta_data)
    }

    fn record(&self, meta_data: &mut Metadata) {
        record_slice(self, meta_data)
    }
}

impl<S: Serialization, const N: usize> Serialization for [S; N] {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        encode_slice(self.as_slice(), ptr, meta_data)
    }

    fn record(&self, meta_data: &mut Metadata) {
        record_slice(self.as_slice(), meta_data)
    }
}

impl<S: Serialization> Serialization for Vec<S> {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        encode_slice(self.as_slice(), ptr, meta_data)
    }

    fn record(&self, meta_data: &mut Metadata) {
        record_slice(self.as_slice(), meta_data)
    }
}

impl<S: DeSerialization> DeSerialization for Vec<S> {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let counts = usize::decode(ptr, ctx)?;

        let mut vec: Vec<S> = Vec::new();

        if (S::WIRE_TYPE.is_fixed_type() && S::WIRE_TYPE == WireType::Bits8)
            || (S::WIRE_TYPE.is_fixed_type() && !cfg!(target_endian = "big"))
        {
            unsafe {
                let start = *ptr;
                let end = (start).add(counts * size_of::<S>());
                ctx.bounds_checker.check_bounds(end.sub(1))?;
                vec.resize_with(counts, || MaybeUninit::uninit().assume_init());
                std::ptr::copy_nonoverlapping(start as *mut S, vec.as_mut_ptr(), counts);
                *ptr = end;
                Ok(vec)
            }
        } else {
            vec.reserve(counts);
            for _ in 0..counts {
                let element = S::decode(ptr, ctx)?;
                vec.push(element);
            }
            Ok(vec)
        }
    }
}

impl<S: DeSerialization, const N: usize> DeSerialization for [S; N] {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let counts = usize::decode(ptr, ctx)?;

        let mut array: [S; N] = unsafe { MaybeUninit::uninit().assume_init() };

        if (S::WIRE_TYPE.is_fixed_type() && S::WIRE_TYPE == WireType::Bits8)
            || (S::WIRE_TYPE.is_fixed_type() && !cfg!(target_endian = "big"))
        {
            unsafe {
                let start = *ptr;
                let end = (start).add(counts * size_of::<S>());
                ctx.bounds_checker.check_bounds(end.sub(1))?;
                std::ptr::copy_nonoverlapping(start as *mut S, array.as_mut_ptr(), counts);
                *ptr = end;
                Ok(array)
            }
        } else {
            for i in 0..counts {
                let element = S::decode(ptr, ctx)?;
                array[i] = element;
            }
            Ok(array)
        }
    }
}
