//! This crate is core of GS11N, it contains serialization/deserialization code for
//! most common used types, and utils used for gs11n_derive and hand writing encode/decode
//! functions.

pub mod plugin;
pub mod serialization;
pub mod types;
pub mod utils;

pub use serialization::*;
pub use types::*;
