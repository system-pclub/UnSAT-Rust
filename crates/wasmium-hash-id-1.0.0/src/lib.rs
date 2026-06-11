#![forbid(unsafe_code, missing_docs, rustdoc::missing_doc_code_examples)]

//! The main data structure for the hash ID
//!
//! ### Structure
//! ```rust
//! #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
//! pub struct WasmiumHashID {
//!     tai_timestamp: [u8; 12],
//!     blake3hash: [u8; 32],
//! }
//! ```
//!
//! #### Usage
//! ```rust
//! use wasmium_hash_id::WasmiumHashID;
//!
//! // The message in bytes
//! let message_id = [0u8; 8];
//!
//! // Hash the message to get the `blake3::Hash`
//! let blake3_hash = blake3::hash(&message_id);
//!
//! // Generate the hash
//! let hash_id = WasmiumHashID::new(blake3_hash).build();
//!
//! // Get the original TAI64N timestamp
//! WasmiumHashID::get_timestamp(hash_id);
//!
//! // Get the orignal Blake3 Hash
//!  WasmiumHashID::get_blake3_hash(hash_id);
//! ```
//!
//!

use borsh::{BorshDeserialize, BorshSerialize};
use tai64::Tai64N;

#[cfg(feature = "random_id")]
use nanorand::{BufferedRng, ChaCha8, Rng};

/// ### Structure
/// ```rust
/// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// pub struct WasmiumHashID {
///     tai_timestamp: [u8; 12],
///     blake3hash: [u8; 32],
/// }
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct WasmiumHashID {
    tai_timestamp: [u8; 12],
    blake3hash: [u8; 32],
}

impl WasmiumHashID {
    /// Generate a new `WasmiumHashID`
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // The message in bytes
    /// let message_id = [0u8; 8];
    ///
    /// // Hash the message to get the `blake3::Hash`
    /// let blake3_hash = blake3::hash(&message_id);
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::new(blake3_hash);
    /// ```
    pub fn new(blake3hash: blake3::Hash) -> WasmiumHashID {
        WasmiumHashID {
            tai_timestamp: Tai64N::now().to_bytes(),
            blake3hash: *blake3hash.as_bytes(),
        }
    }
    /// Build the `WasmiumHashID` in a byte array
    ///
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // The message in bytes
    /// let message_id = [0u8; 8];
    ///
    /// // Hash the message to get the `blake3::Hash`
    /// let blake3_hash = blake3::hash(&message_id);
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::new(blake3_hash).build();
    /// ```
    pub fn build(&self) -> [u8; 44] {
        let mut hash_id = [0u8; 44];
        hash_id[0..=11].copy_from_slice(&self.tai_timestamp);
        hash_id[12..].copy_from_slice(&self.blake3hash);

        hash_id
    }

    /// Restore the timestamp from a byte array hash
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // The message in bytes
    /// let message_id = [0u8; 8];
    ///
    /// // Hash the message to get the `blake3::Hash`
    /// let blake3_hash = blake3::hash(&message_id);
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::new(blake3_hash).build();
    ///
    /// let timestamp = WasmiumHashID::get_timestamp(hash_id);
    /// ```
    pub fn get_timestamp(hash_id: [u8; 44]) -> Result<Tai64N, tai64::Error> {
        Ok(Tai64N::from_slice(&hash_id[0..=11])?)
    }

    /// Restore the original Blake3 hash
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // The message in bytes
    /// let message_id = [0u8; 8];
    ///
    /// // Hash the message to get the `blake3::Hash`
    /// let blake3_hash = blake3::hash(&message_id);
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::new(blake3_hash).build();
    ///
    /// let hash = WasmiumHashID::get_blake3_hash(hash_id);
    /// ```
    pub fn get_blake3_hash(
        hash_id: [u8; 44],
    ) -> Result<blake3::Hash, std::array::TryFromSliceError> {
        let hash_array: [u8; 32] = match hash_id[12..].try_into() {
            Ok(hash_array) => hash_array,
            Err(error) => return Err(error),
        };

        Ok(hash_array.into())
    }

    /// Generate a blake3 hash from a byte array of 32 random bytes from CSPRNG
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::rand32().build();
    /// ```
    #[cfg(feature = "random_id")]
    pub fn rand32() -> WasmiumHashID {
        let mut buffer = [0u8; 32];
        let mut rng = BufferedRng::new(ChaCha8::new());
        rng.fill(&mut buffer);
        let blake3hash = blake3::hash(&buffer);
        WasmiumHashID {
            tai_timestamp: Tai64N::now().to_bytes(),
            blake3hash: *blake3hash.as_bytes(),
        }
    }

    /// Generate a blake3 hash from a byte array of 64 random bytes from CSPRNG
    ///
    /// //! #### Usage
    /// ```rust
    /// use wasmium_hash_id::WasmiumHashID;
    ///
    /// // Generate the hash
    /// let hash_id = WasmiumHashID::rand64().build();
    /// ```
    #[cfg(feature = "random_id")]
    pub fn rand64() -> WasmiumHashID {
        let mut buffer = [0u8; 64];
        let mut rng = BufferedRng::new(ChaCha8::new());
        rng.fill(&mut buffer);
        let blake3hash = blake3::hash(&buffer);
        WasmiumHashID {
            tai_timestamp: Tai64N::now().to_bytes(),
            blake3hash: *blake3hash.as_bytes(),
        }
    }
}
