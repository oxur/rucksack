pub mod shared;
pub mod v020;
pub mod v030;
pub mod v040;
pub mod v050;
pub mod v060;
pub mod v070;
pub mod v080;
pub mod v090;

// The aliases here are taken from the most recent version:
pub use v090::{
    decode_hashmap, default_metadata, default_secrets, key, new_tag, new_tags,
    secrets_from_user_pass, types, DecryptedRecord, EncryptedRecord, HashMap, History, Kind,
    Metadata, Secrets, Status, Tag, ANY_CATEGORY, DEFAULT_CATEGORY, VERSION,
};
