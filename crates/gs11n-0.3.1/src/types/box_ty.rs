use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::wire_type::WireType;
use crate::{DeSerialization, Serialization, WireTypeTrait};

impl<T> WireTypeTrait for Box<T>
where
    T: Sized,
{
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<T> Serialization for Box<T>
where
    T: Sized + Serialization,
{
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        T::encode(&**self, ptr, meta_data);
    }

    fn record(&self, meta_data: &mut Metadata) {
        T::record(&**self, meta_data);
    }
}

impl<T> DeSerialization for Box<T>
where
    T: Sized + DeSerialization,
{
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let t = T::decode(ptr, ctx)?;
        Ok(Box::new(t))
    }
}
