use crate::meta_data::Metadata;
use crate::unsigned::EncodeSize;
use crate::utils::branch_likely_impl::likely;
use crate::wire_type::{WireType, WiredIdConstant};
use crate::Serialization;
use std::cell::Cell;
use std::mem::MaybeUninit;

pub struct Encoder<'a, S: Serialization> {
    value: &'a S,
    meta_data: Cell<Metadata>,
}

macro_rules! wired_id_constants {
    ($id:ident, $T:ty, $($e:expr),*) => (
        match $id {
            $($e => { WiredIdConstant::<$T, $e>::WIRED_ID },)*
            _ => panic!("WTF: id is bigger than 30"),
        }
        )

}

macro_rules! wired_id_constant_from_id {
    ($id:ident, $T:ty) => {
        wired_id_constants!(
            $id, $T, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30
        )
    };
}

/// Encode wired id by a given id.
fn encode_wired_id<S: Serialization>(id: usize, ptr: &mut *mut u8) {
    let p = *ptr;
    // If the field id is less than 31, we use hard code wired id constant
    // TODO Turn parameter id as a generic parameter, and measure will this generate too much codes.
    if likely(id < 0x1F) {
        // Id less than 31, use cached wired_id for performance sake.
        let wired_id = wired_id_constant_from_id!(id, S);
        unsafe {
            *p = wired_id;
            *ptr = p.add(1);
        }
    } else {
        // 32 means the wired id is big than 31.
        // For ids bigger than 31, generate wired id at runtime
        let wired_id = WiredIdConstant::<S, 0x1F>::WIRED_ID;
        unsafe {
            *p = wired_id;
            *ptr = p.add(1);
        }
        let rest_id = id - 0x1E;
        rest_id.encode_raw(ptr);
    }
}

/// Record the space needed of a field.
pub fn size_of_field<S: Serialization>(id: usize, metadata: &mut Metadata) -> usize {
    let mut size = size_of_wired_id(id);
    size += metadata.size;
    if S::WIRE_TYPE == WireType::LengthDelimited {
        // LengthDelimited type has a size followed by wired_id
        size += metadata.size.varint_size();
    }
    size
}

/// Get the space needed to encode a wired id
pub fn size_of_wired_id(id: usize) -> usize {
    if likely(id < 0x1F) {
        1
    } else {
        (id - 0x1E).varint_size() + 1
    }
}

/// Encode a given field
pub fn encode_field<S: Serialization>(
    id: usize,
    value: &S,
    ptr: &mut *mut u8,
    meta_data: &mut Metadata,
) {
    // Encode wire type and id
    encode_wired_id::<S>(id, ptr);

    // Encode size if LengthDelimited type
    if S::WIRE_TYPE == WireType::LengthDelimited {
        meta_data.size.encode_raw(ptr);
    }

    // Encode value
    value.encode(ptr, meta_data);
}

impl<'a, S: Serialization> Encoder<'a, S> {
    /// Create a encoder from a value
    pub fn from(value: &'a S) -> Self {
        let mut meta_data = Metadata::default();
        value.record(&mut meta_data);
        Encoder {
            value,
            meta_data: Cell::new(meta_data),
        }
    }

    /// Encode the value into bytes
    pub fn encode(&self) -> Vec<u8> {
        let meta_data = self.meta_data.take();
        let total_size = meta_data.size;
        self.meta_data.replace(meta_data);

        let mut buf = Vec::with_capacity(total_size);
        unsafe {
            buf.set_len(total_size);
        }
        let mut ptr = buf.as_mut_ptr();

        let mut meta_data = self.meta_data.take();
        self.value.encode(&mut ptr, &mut meta_data);
        self.meta_data.replace(meta_data);

        buf
    }
}
