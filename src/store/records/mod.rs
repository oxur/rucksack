pub mod shared;
pub mod v020;
pub mod v030;
pub mod v040;
pub mod v050;
pub mod v060;
pub mod v070;

// The aliases here are taken from the most recent versions:
pub use shared::key;
pub use v020::Creds;
pub use v060::{Kind, DEFAULT_KIND};
pub use v070::{DecryptedRecord, EncryptedRecord, Metadata};
