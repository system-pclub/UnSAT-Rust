use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::serialization::wire_type::WireType;
use crate::unsigned::EncodeSize;
use crate::{DeSerialization, Serialization, WireTypeTrait};
use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};

impl<K, V, S> WireTypeTrait for HashMap<K, V, S> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<K, V, S> Serialization for HashMap<K, V, S>
where
    K: Serialization,
    V: Serialization,
{
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        // encode length
        self.len().encode_raw(ptr);
        let mut i = 0usize;
        for (key, value) in self {
            key.encode(ptr, meta_data.get(i));
            value.encode(ptr, meta_data.get(i + 1));
            i += 2;
        }
    }

    fn record(&self, meta_data: &mut Metadata) {
        let mut size = 0;
        let mut i = 0usize;
        for (key, value) in self {
            key.record(meta_data.get(i));
            value.record(meta_data.get(i + 1));
            size += meta_data.get(i).size;
            size += meta_data.get(i + 1).size;
            i += 2;
        }
        meta_data.size = size + self.len().varint_size();
    }
}

impl<K, V, S> DeSerialization for HashMap<K, V, S>
where
    K: DeSerialization + std::cmp::Eq + Hash,
    V: DeSerialization,
    S: Default + BuildHasher,
{
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let mut map = Self::default();
        let len = usize::decode(ptr, ctx)?;

        for _ in 0..len {
            let key = K::decode(ptr, ctx)?;
            let value = V::decode(ptr, ctx)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<K, V> WireTypeTrait for BTreeMap<K, V> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<K, V> Serialization for BTreeMap<K, V>
where
    K: Serialization,
    V: Serialization,
{
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        // encode length
        self.len().encode_raw(ptr);
        let mut i = 0usize;
        for (key, value) in self {
            key.encode(ptr, meta_data.get(i));
            value.encode(ptr, meta_data.get(i + 1));
            i += 2;
        }
    }

    fn record(&self, meta_data: &mut Metadata) {
        let mut size = 0;
        let mut i = 0usize;
        for (key, value) in self {
            key.record(meta_data.get(i));
            value.record(meta_data.get(i + 1));
            size += meta_data.get(i).size;
            size += meta_data.get(i + 1).size;
            i += 2;
        }
        meta_data.size = size + self.len().varint_size();
    }
}

impl<K, V> DeSerialization for BTreeMap<K, V>
where
    K: DeSerialization + std::cmp::Ord,
    V: DeSerialization,
{
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let mut map = Self::default();
        let len = usize::decode(ptr, ctx)?;

        for _ in 0..len {
            let key = K::decode(ptr, ctx)?;
            let value = V::decode(ptr, ctx)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}
