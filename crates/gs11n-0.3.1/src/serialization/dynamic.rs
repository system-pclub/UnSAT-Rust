use crate::decoder::{DecodeContext, DecodeError};
use rustc_hash::FxHashMap;
use std::sync::RwLock;

pub type DecodeFn<T> = fn(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Box<T>, DecodeError>;

pub type VTable<T> = RwLock<FxHashMap<usize, DecodeFn<T>>>;
