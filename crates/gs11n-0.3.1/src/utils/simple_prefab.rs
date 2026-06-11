use crate::decoder::{DecodeContext, DecodeError};
use crate::meta_data::Metadata;
use crate::wire_type::WireType;
use crate::{DeSerialization, Serialization, WireTypeTrait};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct SimplePrefab(u64);

impl WireTypeTrait for SimplePrefab {
    const WIRE_TYPE: WireType = WireType::Prefab;
}

impl Serialization for SimplePrefab {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        self.0.encode(ptr, meta_data)
    }

    fn record(&self, meta_data: &mut Metadata) {
        self.0.record(meta_data)
    }
}

impl DeSerialization for SimplePrefab {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let v = u64::decode(ptr, ctx)?;
        Ok(Self { 0: v })
    }
}

impl Deref for SimplePrefab {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimplePrefab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SimplePrefab {
    pub fn new(index: u64) -> Self {
        Self { 0: index }
    }
}
