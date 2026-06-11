pub mod decoder;
pub mod dynamic;
pub mod encoder;
pub mod meta_data;
pub mod prefab_loader;
pub mod swap_bytes;
pub mod wire_type;

use crate::decoder::{DecodeContext, DecodeError};
use crate::serialization::meta_data::Metadata;
use wire_type::WireType;

/// Illustrate what kind of wire type a type is, check wire_type.rs for details
pub trait WireTypeTrait {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

/// This trait must be implemented if a type can be serialized.
pub trait Serialization: WireTypeTrait + Sized {
    /// Encode a type into bytes, meta_data is used to know the required space for the type
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata);

    /// Record information which may be used in encoding.
    /// This function is used to speed up encoding by caching information (currently is the space
    /// needed by a value)
    ///
    /// consider a struct like this:
    /// ```no_run
    /// struct A {
    /// }
    ///
    /// struct B {
    ///   a: A,
    /// }
    ///
    /// struct C {
    ///   b: B,
    /// }
    ///
    /// struct D {
    ///  c: C,
    /// }
    ///```
    /// when encoding, each struct type need to first encode the size of itself, then encode it's
    /// fields. in the case above, field `a` in struct B will be calculated three times, if we don't
    /// cache the size. The deeper a struct is, the more times the same field will be calculated.
    ///
    /// Check meta_data.rs for details of how to use `Metadata`
    fn record(&self, meta_data: &mut Metadata);
}

/// This trait must be implemented if a type can be deserialized.
///
/// Since the `decode` function return a value, a struct which implement this should also implement
/// trait `Default`
///
/// So why not add the `Default` constraint? Because currently Default trait has problems on
/// const generic arrays, check this discussion for details:
/// `<https://users.rust-lang.org/t/implement-default-trait-on-const-genric-array/69894>`
pub trait DeSerialization: WireTypeTrait + Sized {
    /// Get value from a given encoded data.
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError>;
}
