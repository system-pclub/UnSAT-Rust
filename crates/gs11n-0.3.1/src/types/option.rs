use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::serialization::wire_type::WireType;
use crate::unsigned::EncodeSize;
use crate::{DeSerialization, Serialization, WireTypeTrait};

impl<S> WireTypeTrait for Option<S> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<S: Serialization> Serialization for Option<S> {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        match self {
            Some(v) => {
                let size = meta_data.get(0).size;
                size.encode(ptr, meta_data);
                v.encode(ptr, meta_data.get(0))
            }
            None => {
                let size = 0usize;
                size.encode(ptr, meta_data);
            }
        }
    }

    fn record(&self, meta_data: &mut Metadata) {
        meta_data.size = match self {
            Some(v) => {
                v.record(meta_data.get(0));
                let element_size = meta_data.get(0).size;
                element_size.varint_size() + element_size
            }
            None => 1,
        };
    }
}

impl<S: DeSerialization> DeSerialization for Option<S> {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let size = usize::decode(ptr, ctx)?;
        if size == 0 {
            Ok(None)
        } else {
            let v = S::decode(ptr, ctx)?;
            Ok(Some(v))
        }
    }
}
