use crate::decoder::DecodeError;
use crate::serialization::Serialization;
use std::marker::PhantomData;

/// Enum to illustrate what kind a serializable is.
/// `BitsX` means the type take a fixed size of X bytes.
/// `Varint` means the type can be encoded with varint encoding.
/// `LengthDelimited` means the value is a varint encoded length followed by the specified
/// number of bytes of data.
/// `Prefab` is only used in decoding, which means the value must be passed to a prefab loader,
/// to get the real encoded data. Check prefab_loader.rs for details.
#[derive(PartialEq)]
pub enum WireType {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
    Bits64 = 3,
    Bits128 = 4,
    Prefab = 5,
    Varint = 6,
    LengthDelimited = 7,
}

impl WireType {
    /// Get a wire type from a number
    pub fn from(v: u8) -> Result<WireType, DecodeError> {
        match v {
            0 => Result::Ok(WireType::Bits8),
            1 => Result::Ok(WireType::Bits16),
            2 => Result::Ok(WireType::Bits32),
            3 => Result::Ok(WireType::Bits64),
            4 => Result::Ok(WireType::Bits128),
            5 => Result::Ok(WireType::Prefab),
            6 => Result::Ok(WireType::Varint),
            7 => Result::Ok(WireType::LengthDelimited),
            _ => Result::Err(DecodeError::InvalidType),
        }
    }

    /// is the wired type a fixed type, which means the size is know at compile time.
    pub fn is_fixed_type(&self) -> bool {
        match self {
            WireType::Bits8 => true,
            WireType::Bits16 => true,
            WireType::Bits32 => true,
            WireType::Bits64 => true,
            WireType::Bits128 => true,
            WireType::Prefab => false,
            WireType::Varint => false,
            WireType::LengthDelimited => false,
        }
    }
}

pub struct WiredIdConstant<S: Serialization, const ID: u8> {
    phantom: PhantomData<S>,
}

impl<S: Serialization, const ID: u8> WiredIdConstant<S, ID> {
    pub const WIRED_ID: u8 = wired_id_constant_from(ID, S::WIRE_TYPE);
}

const fn wired_id_constant_from(id: u8, wire_type: WireType) -> u8 {
    if id > 0b11111 {
        panic!("wired id constant is used for common use, and only support id which is less than 32");
    }
    ((wire_type as u8) << 5) | id
}

pub(crate) fn deformmat_wired_id(wired_id: u8) -> Result<(u8, WireType), DecodeError> {
    let wire_type = WireType::from(wired_id >> 5)?;
    let id = wired_id & 0b00011111;
    Result::Ok((id, wire_type))
}

/// Safe Type used for encoding, which do not contains `Prefab` type.
pub enum NonPrefabWireType {
    Bits8,
    Bits16,
    Bit32,
    Bit64,
    Bit128,
    Varint,
    LengthDelimited,
}

impl NonPrefabWireType {
    /// Convert a NonPrefabWireType to WireType
    pub const fn to_wire_type(&self) -> WireType {
        match self {
            NonPrefabWireType::Bits8 => WireType::Bits8,
            NonPrefabWireType::Bits16 => WireType::Bits16,
            NonPrefabWireType::Bit32 => WireType::Bits32,
            NonPrefabWireType::Bit64 => WireType::Bits64,
            NonPrefabWireType::Bit128 => WireType::Bits128,
            NonPrefabWireType::Varint => WireType::Varint,
            NonPrefabWireType::LengthDelimited => WireType::LengthDelimited,
        }
    }
}
