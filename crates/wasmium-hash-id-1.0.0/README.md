## WasmiumHashID

A way to create Cryptographically Secure ID's chronologically using TAI64N as the timestamp and Blake3 as the Cryptographic Hashing Algorithm. 

This is useful where you need sorted IDs according to time. The first 12 bytes are the TAI64N timestamp while the rest 32 bytes represent a Blake3 hash.

To extract the secure hash, just read the array from the 12th byte.

 The main data structure for the hash ID 

### Structure 
```rust 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)] 
pub struct WasmiumHashID { 
    // TAI64N timestamp converted to a byte array
  tai_timestamp: [u8; 12], 
    // Blake3 `blake3::Hash` converted into a byte array
  blake3hash: [u8; 32], 
} 
```

#### Usage 
```rust 
use wasmium_hash_id::WasmiumHashID; 

// The message in bytes 
let message_id = [0u8; 8]; 

// Hash the message to get the `blake3::Hash` 
let blake3_hash = blake3::hash(&message_id); 

// Generate the hash 
let hash_id = WasmiumHashID::new(blake3_hash).build(); 

// Get the original TAI64N timestamp 
WasmiumHashID::get_timestamp(hash_id); 

// Get the orignal Blake3 Hash 
 WasmiumHashID::get_blake3_hash(hash_id); 
```
