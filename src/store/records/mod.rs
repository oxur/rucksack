pub mod shared;
pub mod v020;
pub mod v030;
pub mod v040;
pub mod v050;
pub mod v060;
pub mod v070;

// The aliases here are taken from the most recent versions:
pub use v070::{
    decode_hashmap, default_metadata, default_secrets, key, DecryptedRecord, EncryptedRecord,
    HashMap, Kind, Metadata, Secrets, Status, DEFAULT_CATEGORY, VERSION,
};
