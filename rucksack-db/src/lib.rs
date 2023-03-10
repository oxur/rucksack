pub mod crypto;
pub mod csv;
pub mod db;
pub mod records;
pub mod store;
pub mod testing;

pub use records::{
    default_metadata, default_secrets, key, new_tag, new_tags, secrets_from_user_pass,
    DecryptedRecord, EncryptedRecord, Metadata, Secrets, Status, Tag,
};

// This is the library version and shouldn't be used for schema versions. Instead,
// use crate::db::version (which points to crate::records::version).
pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
