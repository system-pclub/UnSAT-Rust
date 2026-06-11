use crate::decoder::{DecodeContext, DecodeError};
use crate::serialization::wire_type::NonPrefabWireType;

/// Prefab loader, used to bind GS11N to your resource system
pub trait PrefabLoader {
    /// Skip certain data, which represents to a prefab.
    ///
    /// GS11N do not know the format of a prefab, how to decode the data ups to the implementation
    /// of a `PrefabLoader`, so the implementation should tell GS11N how to skip a prefab data.
    fn skip_wire_type(&self) -> NonPrefabWireType;
    /// Handle a prefab data, , take an address of encoded data,
    /// and must return another address of encoded data.
    fn handle_prefab(&self, ptr: &mut *const u8, ctx: &DecodeContext)
        -> Result<&[u8], DecodeError>;
}
