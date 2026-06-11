use crate::serialization::prefab_loader::PrefabLoader;
use crate::utils::branch_likely_impl::unlikely;
use crate::wire_type::{deformmat_wired_id, NonPrefabWireType, WireType};
use crate::DeSerialization;

struct DefaultPrefabLoader {}

impl PrefabLoader for DefaultPrefabLoader {
    fn skip_wire_type(&self) -> NonPrefabWireType {
        panic!("You didn't set a prefab loader for a decoder!");
    }

    fn handle_prefab(
        &self,
        _ptr: &mut *const u8,
        _ctx: &DecodeContext,
    ) -> Result<&[u8], DecodeError> {
        panic!("You didn't set a prefab loader for a decoder!");
    }
}

static DEFAULT_PREFAB_LOADER: DefaultPrefabLoader = DefaultPrefabLoader {};

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    DecodeOutOfBounds,
    InvalidType,
    PrefabNotExist,
    PrefabToAnotherPrefab,
    AddOverflow,
    VersionNotCompatible,
}

/// Decode a wired id into an field id and wired type
pub fn decode_wired_id(
    ptr: &mut *const u8,
    ctx: &DecodeContext,
) -> Result<(usize, WireType), DecodeError> {
    let wired_id = unsafe {
        ctx.bounds_checker.check_bounds(*ptr)?;
        let wired_id = **ptr;
        *ptr = ptr.add(1);
        wired_id
    };
    let (id, wire_type) = deformmat_wired_id(wired_id)?;
    if unlikely(id == 0x1F) {
        let rest_id = usize::decode(ptr, ctx)?;
        let id = 0x1E + rest_id;
        Result::Ok((id, wire_type))
    } else {
        Result::Ok((id as usize, wire_type))
    }
}

/// Decode a field data into a value
/// # Arguments
/// * `is_prefab` - if true, the data will be sent to the prefab loader, to get the real data.
pub fn decode_field<S: DeSerialization>(
    ptr: &mut *const u8,
    ctx: &DecodeContext,
    is_prefab: bool,
) -> Result<S, DecodeError> {
    if is_prefab {
        unsafe {
            let buffer = ctx.prefab_loader.handle_prefab(ptr, ctx)?;
            let mut new_ptr = buffer.as_ptr();
            let new_bounds_checker = BoundsChecker {
                bound: new_ptr.add(buffer.len()),
            };
            let new_ctx = DecodeContext {
                bounds_checker: new_bounds_checker,
                prefab_loader: ctx.prefab_loader,
            };
            let v = S::decode(&mut new_ptr, &new_ctx)?;
            Ok(v)
        }
    } else {
        if S::WIRE_TYPE == WireType::LengthDelimited {
            let size = usize::decode(ptr, ctx)?;
            unsafe {
                ctx.bounds_checker.check_bounds((*ptr).add(size))?;
            }
        }
        let v = S::decode(ptr, ctx)?;
        Ok(v)
    }
}

pub struct BoundsChecker {
    bound: *const u8,
}

impl BoundsChecker {
    /// Get the boundary of the encoded data.
    pub fn get_bound(&self) -> *const u8 {
        self.bound
    }

    /// Check if the given address is out of the boundary.
    pub fn check_bounds(&self, ptr: *const u8) -> Result<(), DecodeError> {
        if unlikely(ptr.ge(&self.bound)) {
            Result::Err(DecodeError::DecodeOutOfBounds)
        } else {
            Result::Ok(())
        }
    }
}

pub struct DecodeContext<'a> {
    pub bounds_checker: BoundsChecker,
    pub prefab_loader: &'a dyn PrefabLoader,
}

impl<'a> DecodeContext<'a> {
    /// Skip the data, used when the field id is not recognized.
    pub fn skip(&self, ptr: &mut *const u8, wire_type: WireType) -> Result<(), DecodeError> {
        unsafe {
            match wire_type {
                WireType::Bits8 => *ptr = ptr.add(1),
                WireType::Bits16 => *ptr = ptr.add(2),
                WireType::Bits32 => *ptr = ptr.add(4),
                WireType::Bits64 => *ptr = ptr.add(8),
                WireType::Bits128 => *ptr = ptr.add(16),
                WireType::LengthDelimited => {
                    let size = usize::decode(ptr, self)?;
                    let end = ptr.add(size);
                    self.bounds_checker.check_bounds(end)?;
                    *ptr = end;
                }
                WireType::Varint => {
                    let mut p = *ptr;
                    loop {
                        self.bounds_checker.check_bounds(p)?;
                        if *p < 0x80 {
                            break;
                        } else {
                            p = p.add(1);
                        }
                    }
                    *ptr = p;
                }
                // same as above, except checking prefab type
                WireType::Prefab => match self.prefab_loader.skip_wire_type().to_wire_type() {
                    WireType::Bits8 => *ptr = ptr.add(1),
                    WireType::Bits16 => *ptr = ptr.add(2),
                    WireType::Bits32 => *ptr = ptr.add(4),
                    WireType::Bits64 => *ptr = ptr.add(8),
                    WireType::Bits128 => *ptr = ptr.add(16),
                    WireType::LengthDelimited => {
                        let size = usize::decode(ptr, self)?;
                        let end = ptr.add(size);
                        self.bounds_checker.check_bounds(end)?;
                        *ptr = end;
                    }
                    WireType::Varint => {
                        let mut p = *ptr;
                        loop {
                            self.bounds_checker.check_bounds(p)?;
                            if *p < 0x80 {
                                break;
                            } else {
                                p = p.add(1);
                            }
                        }
                        *ptr = p;
                    }
                    WireType::Prefab => return Err(DecodeError::PrefabToAnotherPrefab),
                },
            }
            Ok(())
        }
    }
}

pub struct Decoder<'a> {
    buf: &'a [u8],
    ctx: DecodeContext<'a>,
}

impl<'a> Decoder<'a> {
    /// Create a decoder from a given data buffer
    pub fn from_data(data: &'a [u8]) -> Self {
        Self::from_data_with_preloader(data, &DEFAULT_PREFAB_LOADER)
    }

    /// Create a decoder from a given data buffer, and a prefab loader
    pub fn from_data_with_preloader(data: &'a [u8], prefab_loader: &'a dyn PrefabLoader) -> Self {
        unsafe {
            let buf = data;
            let bound = buf.as_ptr().add(buf.len());
            Self {
                buf,
                ctx: DecodeContext {
                    bounds_checker: BoundsChecker { bound },
                    prefab_loader,
                },
            }
        }
    }

    /// Decode the data into a value.
    pub fn decode<S: DeSerialization>(&self) -> Result<S, DecodeError> {
        let mut ptr = self.buf.as_ptr();
        let v = S::decode(&mut ptr, &self.ctx)?;
        Result::Ok(v)
    }

    /// Get the context of a decoder, used for testing, you probably shouldn't use it
    pub fn get_context(&self) -> &DecodeContext {
        &self.ctx
    }
}
