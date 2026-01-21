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

pub fn version() -> versions::SemVer {
    versions::SemVer::new(VERSION).expect("VERSION constant must be valid semver format")
}

/// Decrypt a record using version-aware decryption
///
/// This function handles backwards compatibility by using the appropriate
/// decryption method based on the original database version, then migrating
/// the decrypted record to the current version.
pub fn decrypt_versioned(
    encrypted: &EncryptedRecord,
    store_pwd: String,
    salt: String,
    db_version: versions::SemVer,
) -> anyhow::Result<DecryptedRecord> {
    use anyhow::anyhow;

    let trimmed_version = shared::trim_version(db_version.clone());
    log::debug!(
        "decrypt_versioned: db_version={}, trimmed={}",
        db_version,
        trimmed_version
    );

    // v0.7.0 introduced the Secrets struct (replacing Creds)
    // v0.4.0 through v0.6.0 all used Creds
    let v070 = shared::version("0.7.0").map_err(|e| anyhow!("{}", e))?;

    if trimmed_version < v070 {
        log::debug!("Using old decrypt path for version < 0.7.0");
        // Old format: encrypted data contains Creds, not Secrets
        // We need to use the old decrypt method

        // Convert v090::EncryptedRecord to v060::EncryptedRecord for decryption
        let old_record = v060::EncryptedRecord {
            key: encrypted.key(),
            value: encrypted.value(),
            metadata: v060::Metadata {
                kind: match encrypted.metadata().kind {
                    Kind::Password => v060::Kind::Password,
                    Kind::Account => v060::Kind::Account,
                    _ => v060::Kind::Password, // Default for other types
                },
                url: encrypted.metadata().url.clone(),
                created: encrypted.metadata().created.clone(),
                imported: encrypted.metadata().imported.clone(),
                updated: encrypted.metadata().updated.clone(),
                password_changed: encrypted.metadata().password_changed.clone(),
                last_used: encrypted.metadata().last_used.clone(),
                access_count: encrypted.metadata().access_count,
            },
        };

        // Decrypt using v060's method (which expects Creds)
        log::debug!("Calling v060::decrypt()");
        let decrypted_v060 = old_record.decrypt(store_pwd, salt)?;
        log::debug!("v060::decrypt() succeeded, migrating to current version");

        // Migrate from v060 to v070
        let decrypted_v070 = v070::migrate_decrypted_record_from_v060(decrypted_v060);

        // Migrate from v070 to v080
        let decrypted_v080 = v080::migrate_decrypted_record_from_v070(decrypted_v070);

        // Migrate from v080 to v090
        Ok(v090::migrate_decrypted_record_from_v080(decrypted_v080))
    } else {
        // New format: use current decrypt method
        log::debug!("Using current decrypt path for version >= 0.7.0");
        encrypted.decrypt(store_pwd, salt)
    }
}
