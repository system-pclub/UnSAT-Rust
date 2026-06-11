use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::serialization::wire_type::WireType;
use crate::unsigned::EncodeSize;
use crate::{DeSerialization, Serialization, WireTypeTrait};
use std::mem::{size_of, MaybeUninit};

fn encode_str(str: &str, ptr: &mut *mut u8) {
    // encode size first
    str.len().encode_raw(ptr);
    unsafe {
        let mut p = *ptr;
        std::ptr::copy_nonoverlapping(str.as_ptr(), p as *mut u8, str.len());
        p = ((p as *mut u8).add(str.len())) as *mut u8;
        *ptr = p;
    }
}

fn record_str(str: &str, meta_data: &mut Metadata) {
    let mut size = str.len().varint_size();
    size += str.len() * std::mem::size_of::<u8>();
    meta_data.size = size;
}

impl WireTypeTrait for String {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl Serialization for String {
    fn encode(&self, ptr: &mut *mut u8, _meta_data: &mut Metadata) {
        encode_str(self, ptr);
    }

    fn record(&self, meta_data: &mut Metadata) {
        record_str(self, meta_data);
    }
}

impl DeSerialization for String {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let counts = usize::decode(ptr, ctx)?;

        let mut string = String::new();
        unsafe {
            let vec: &mut Vec<u8> = string.as_mut_vec();
            vec.reserve(counts);
            vec.set_len(counts);
            let start = *ptr;
            let end = (start).add(counts * size_of::<u8>());
            ctx.bounds_checker.check_bounds(end.sub(1))?;
            std::ptr::copy_nonoverlapping(start as *mut u8, vec.as_mut_ptr(), counts);
            *ptr = end;
            Ok(string)
        }
    }
}

impl WireTypeTrait for &str {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl Serialization for &str {
    fn encode(&self, ptr: &mut *mut u8, _meta_data: &mut Metadata) {
        encode_str(self, ptr);
    }

    fn record(&self, meta_data: &mut Metadata) {
        record_str(self, meta_data);
    }
}
