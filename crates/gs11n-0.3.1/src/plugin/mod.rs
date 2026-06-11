use crate::decoder::DecodeError;
use rustc_hash::FxHashMap;
use std::sync::Mutex;
use std::sync::RwLock;

pub static S11N_VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
fn check_gs11n_version(caller_version: &str) -> Result<(), DecodeError> {
    // TODO SemVer check when comes to 1.0
    if caller_version != S11N_VERSION {
        Err(DecodeError::VersionNotCompatible)
    } else {
        Ok(())
    }
}

pub struct TraitInfo {
    pub vtable: &'static UnsafeVTable,
    pub update_fn: unsafe fn(v_table: &'static UnsafeVTable),
}

pub type TraitRegister = FxHashMap<String, TraitInfo>;

pub type UnsafeVTable = RwLock<FxHashMap<usize, fn()>>;

lazy_static::lazy_static! {
    pub static ref REGISTERED_TRAITS: Mutex<TraitRegister> = Mutex::new(FxHashMap::default());
}

#[no_mangle]
/// Sync dyn types between caller and dynamic library.
///  # Safety
/// FFI
pub unsafe fn sync_traits(caller_register: &TraitRegister) {
    for (name, caller_trait_info) in caller_register {
        let callee_traits = REGISTERED_TRAITS.lock().unwrap();
        match callee_traits.get(name) {
            None => {
                continue;
            }
            Some(callee_trait_info) => {
                (callee_trait_info.update_fn)(caller_trait_info.vtable);
            }
        }
    }
}
