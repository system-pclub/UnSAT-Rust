use crate::decoder::DecodeContext;
use crate::meta_data::Metadata;
use crate::serialization::decoder::DecodeError;
use crate::serialization::wire_type::WireType;
use crate::utils::branch_likely_impl::unlikely;
use crate::{DeSerialization, Serialization, WireTypeTrait};
use std::mem::size_of;

pub trait EncodeSize {
    fn varint_size(&self) -> usize;
    fn encode_raw(&self, ptr: &mut *mut u8);
}
macro_rules! s11n_for_unsigned {
        ($($t:ty)*) => ($(
        impl EncodeSize for $t {
            fn varint_size(&self) -> usize {
                let value = *self | 0x1; // avoid value == 0
                let bits = (size_of::<Self>() * 8) as u32;
                // Revert bit scan.
                let rbs = bits - value.leading_zeros() - 1;
                let rbs = rbs as usize;
                // Same as (((rbs / 7) + rbs) / 8) + 1
                let size = (rbs * 9 + 73) / 64;
                return size;
            }

            fn encode_raw(&self, ptr: &mut *mut u8) {
                unsafe {
                    let mut temp = *ptr;
                    let mut value = *self;
                    if cfg!(target_endian = "big") {
                        value = (*self).swap_bytes();
                    }
                    let mut n: u8;
                    // TODO bench with if likely instead of loop
                    loop {
                        let p = &mut value as *mut Self as *mut u8;
                        if value >= 0x80 {
                            n = *p;
                            n |= 0x80;
                            *temp = n;
                            temp = temp.add(1);
                            value >>= 7;
                        } else {
                            n = *p;
                            *temp = n;
                            temp = temp.add(1);
                            *ptr = temp;
                            return;
                        }
                    }
                }
            }
        }

        impl WireTypeTrait for $t {
            const WIRE_TYPE: WireType = WireType::Varint;
        }

        impl Serialization for $t {
            fn encode(&self, ptr: &mut *mut u8, _meta_data: &mut Metadata) {
                self.encode_raw(ptr)
            }

            fn record(&self, meta_data: &mut Metadata) {
                meta_data.size = self.varint_size()
            }
        }

        impl DeSerialization for $t {
            fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
                unsafe {
                    let mut p = *ptr;
                    ctx.bounds_checker.check_bounds(p)?;
                    let mut v = *p as Self;
                    if (v & 0x80) != 0 {
                        let mut i = 1;
                        loop {
                            p = p.add(1);
                            ctx.bounds_checker.check_bounds(p)?;
                            let n : Self = *p as Self;
                            let add = (n - 1) << (7 * i);
                            let w = v.wrapping_add(add);
                            if unlikely(w < v) {
                                return Result::Err(DecodeError::AddOverflow)
                            } else {
                                v = w;
                            }
                            i += 1;
                            if n < 0x80 {
                                break;
                            }
                        }
                    }
                    if cfg!(target_endian = "big") {
                        v = v.swap_bytes();
                    }
                    p = p.add(1);
                    *ptr = p;
                    return Result::Ok(v);
                }
            }
        }
    )*)
}

s11n_for_unsigned!(usize u8 u16 u32 u64);
