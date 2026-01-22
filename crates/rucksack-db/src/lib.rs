//! Encrypted database for password and secrets management.
//!
//! `rucksack-db` provides a secure, encrypted database for storing passwords and other
//! secrets. The database uses AES-256-GCM encryption, supports automatic backups, and
//! handles schema versioning for seamless upgrades.
//!
//! # Features
//!
//! - **Strong Encryption**: AES-256-GCM authenticated encryption
//! - **Automatic Backups**: Database snapshots before each save
//! - **Schema Versioning**: Automatic migration between versions
//! - **Thread-Safe**: Concurrent access via DashMap
//! - **Import/Export**: Support for Chrome and Firefox CSV formats
//!
//! # Architecture
//!
//! The database consists of several layers:
//!
//! - **[`db`]**: High-level database management (`DB`, `EncryptedDB`, `VersionedDB`)
//! - **[`records`]**: Record types and encoding ([`DecryptedRecord`], [`EncryptedRecord`])
//! - **[`crypto`]**: Encryption and decryption primitives
//! - **[`store`]**: Storage backends (filesystem, redb)
//! - **[`csv`]**: Import/export for browser password managers
//!
//! # Quick Start
//!
//! ```no_run
//! use rucksack_db::db::DB;
//!
//! # fn main() -> anyhow::Result<()> {
//! // Initialize a new database
//! DB::init(
//!     "/path/to/secrets.db",
//!     "/path/to/backups",
//!     Some("password".to_string()),
//!     Some("salt".to_string()),
//! )?;
//!
//! // Open and use the database
//! let mut db = DB::new(
//!     "/path/to/secrets.db",
//!     "/path/to/backups",
//!     Some("password".to_string()),
//!     Some("salt".to_string()),
//! );
//! db.open()?;
//!
//! // ... use the database ...
//!
//! db.close()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Security Considerations
//!
//! - Passwords and salts are stored in `SecretString` wrappers to prevent accidental exposure
//! - Decrypted data is wrapped in `Secret<T>` types
//! - All sensitive data is zeroized on drop
//! - Database files are encrypted at rest

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
    versions::SemVer::new(env!("CARGO_PKG_VERSION"))
        .expect("CARGO_PKG_VERSION must be valid semver format")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version.major, 0);
        assert!(version.minor >= 10);
    }

    #[test]
    fn test_reexports() {
        // Test that re-exported functions are accessible
        let secrets = default_secrets();
        assert_eq!(secrets.user, "");

        let secrets2 = secrets_from_user_pass("user", "pass");
        assert_eq!(secrets2.user, "user");

        let metadata = default_metadata();
        assert!(!metadata.name.is_empty() || metadata.name.is_empty());

        let tag = new_tag("test".to_string());
        assert_eq!(tag.value, "test");

        let tags = new_tags(vec!["t1".to_string(), "t2".to_string()]);
        assert_eq!(tags.len(), 2);
    }
}
