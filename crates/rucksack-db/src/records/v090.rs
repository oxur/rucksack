use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use rucksack_lib::{time, util};

use crate::crypto::{decrypt, encrypt};

use super::shared;
use super::v080;
pub use super::v080::{
    name_from_key, types, Kind, Metadata, Secrets, Status, Tag, ANY_CATEGORY, DEFAULT_CATEGORY,
};

pub const VERSION: &str = "0.9.0";

// Hashmap - the primary store data structure

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn migrate_hashmap_from_v080(hm_v080: v080::HashMap) -> HashMap {
    let hm: HashMap = dashmap::DashMap::new();
    for i in hm_v080.iter() {
        let r = i.value();
        let _ = hm.insert(
            i.key().to_string(),
            migrate_encrypted_record_from_v080(r.clone()),
        );
    }
    hm
}

pub fn decode_hashmap(bytes: &[u8], mut version: versions::SemVer) -> Result<HashMap> {
    log::debug!(version = version.to_string().as_str(), operation = "decode"; "Decoding hashmap from stored bytes");
    version = shared::trim_version(version);
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!(operation = "decode"; "Created hashmap");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!(operation = "decode"; "Created vec for sorted data");
    let current_version = shared::version(VERSION).map_err(|e| anyhow!("{}", e))?;
    if version < current_version {
        // version.
        log::info!(version = "0.8.0", operation = "migrate"; "Attempting to decode hashmap from previous version");
        let hm = v080::decode_hashmap(&bytes, version)?;
        return Ok(migrate_hashmap_from_v080(hm));
    }
    match bincode::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
        Ok((result, _len)) => {
            sorted_vec = result;
            for (key, val) in sorted_vec {
                if hm.insert(key.clone(), val).is_some() {}
            }
            Ok(hm)
        }
        Err(e) => {
            log::info!(error = e.to_string().as_str(), operation = "decode"; "couldn't deserialise bincoded hashmap bytes");
            Err(anyhow!(e))
        }
    }
}

// Secret data

pub fn default_secrets() -> Secrets {
    Secrets {
        ..Default::default()
    }
}

pub fn secrets_from_user_pass(user: &str, pwd: &str) -> Secrets {
    Secrets {
        user: user.to_string(),
        password: pwd.to_string(),
        ..Default::default()
    }
}

pub fn migrate_secrets_from_v080(s080: v080::Secrets) -> Secrets {
    Secrets {
        account_id: s080.account_id,
        user: s080.user,
        password: s080.password,
        public_key: s080.public_key,
        private_key: s080.private_key,
        public_cert: s080.public_cert,
        private_cert: s080.private_cert,
        root_cert: s080.root_cert,
        key: s080.key,
        secret: s080.secret,
    }
}

// Metadata

pub fn new_tag(value: String) -> Tag {
    Tag {
        value,
        created: time::now(),
        updated: time::epoch_zero(),

        ..Default::default()
    }
}

pub fn new_tags(values: Vec<String>) -> Vec<Tag> {
    values.into_iter().map(new_tag).collect()
}

pub fn default_metadata() -> Metadata {
    let now = time::now();
    let time_zero = time::epoch_zero();
    let mut md = Metadata {
        ..Default::default()
    };
    md.state = Status::default();
    md.kind = Kind::default();
    md.category = DEFAULT_CATEGORY.to_string();
    md.created = now.clone();
    md.updated = now;
    md.imported = time_zero.clone();
    md.password_changed = time_zero.clone();
    md.last_used = time_zero.clone();
    md.synced = time_zero;
    md
}

pub fn migrate_metadata_from_v080(md080: v080::Metadata, name: String) -> Metadata {
    Metadata {
        kind: md080.kind,
        category: md080.category,
        name,
        url: md080.url,
        created: md080.created,
        imported: md080.imported,
        updated: md080.updated,
        password_changed: md080.password_changed,
        last_used: md080.last_used,
        synced: md080.synced,
        access_count: md080.access_count,
        state: md080.state,
        tags: md080.tags,
    }
}

// Decrypted records
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct History {
    pub secrets: Secrets,
    pub metadata: Metadata,
}

pub fn new_history(secrets: Secrets, metadata: Metadata) -> History {
    History { secrets, metadata }
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub secrets: Secrets,
    pub metadata: Metadata,
    pub history: Vec<History>,
}

impl DecryptedRecord {
    pub fn new() -> DecryptedRecord {
        DecryptedRecord {
            secrets: default_secrets(),
            metadata: default_metadata(),
            history: vec![],
        }
    }

    pub fn add_tag(&mut self, value: String) {
        self.metadata.add_tag(value)
    }

    pub fn add_tags(&mut self, values: Vec<String>) {
        self.metadata.add_tags(values)
    }

    pub fn encrypt(&self, store_pwd: &str, salt: &str) -> Result<EncryptedRecord> {
        let encoded_secrets = bincode::encode_to_vec(&self.secrets, util::bincode_cfg()).unwrap();
        let encrypted_secrets = encrypt(encoded_secrets, store_pwd, salt)?;

        let encoded_history = bincode::encode_to_vec(&self.history, util::bincode_cfg()).unwrap();
        let encrypted_history = encrypt(encoded_history, store_pwd, salt)?;

        Ok(EncryptedRecord {
            key: self.key(),
            value: encrypted_secrets,
            metadata: self.metadata(),
            history: encrypted_history,
        })
    }

    pub fn history(&self) -> Vec<History> {
        self.history.clone()
    }

    pub fn key(&self) -> String {
        key(
            self.metadata.category.as_str(),
            self.metadata.kind.clone(),
            self.name_or_user().as_str(),
            self.metadata.url.as_str(),
        )
    }

    pub fn key_with_pass(&self) -> String {
        format!("{}:{}", self.key(), self.password())
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn name(&self) -> String {
        self.metadata.name.clone()
    }

    pub fn name_or_user(&self) -> String {
        let mut name = self.name();
        if name.is_empty() {
            name = self.user();
        }
        name
    }

    pub fn password(&self) -> String {
        self.secrets.password.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.metadata.updated = time::now();
        self.metadata.name = new_name;
    }

    pub fn set_password(&mut self, new_pwd: String) {
        let now = time::now();
        self.history
            .push(new_history(self.secrets.clone(), self.metadata()));
        self.secrets.password = new_pwd;
        self.metadata.password_changed = now.clone();
        self.metadata.updated = now;
    }

    pub fn set_status(&mut self, new_state: Status) {
        self.metadata.updated = time::now();
        self.metadata.state = new_state;
    }

    pub fn set_kind(&mut self, new_kind: Kind) {
        self.metadata.updated = time::now();
        self.metadata.kind = new_kind;
    }

    pub fn set_url(&mut self, new_url: String) {
        self.metadata.updated = time::now();
        self.metadata.url = new_url;
    }

    pub fn set_user(&mut self, new_user: String) {
        self.metadata.updated = time::now();
        self.secrets.user = new_user.clone();
        self.metadata.name = new_user;
    }

    pub fn url(&self) -> String {
        self.metadata.url.clone()
    }

    pub fn user(&self) -> String {
        self.secrets.user.clone()
    }
}

impl Default for DecryptedRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub fn migrate_decrypted_record_from_v080(dr: v080::DecryptedRecord) -> DecryptedRecord {
    DecryptedRecord {
        secrets: migrate_secrets_from_v080(dr.secrets.clone()),
        metadata: migrate_metadata_from_v080(dr.metadata.clone(), name_from_key(dr.key())),
        history: vec![],
    }
}

// Encrypted records

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct EncryptedRecord {
    pub key: String,
    pub value: Vec<u8>,
    pub metadata: Metadata,
    pub history: Vec<u8>,
}

impl EncryptedRecord {
    pub fn add_tag(&mut self, value: String) {
        self.metadata.add_tag(value)
    }

    pub fn add_tags(&mut self, values: Vec<String>) {
        self.metadata.add_tags(values)
    }

    pub fn history(&self) -> Vec<u8> {
        self.history.clone()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn decrypt(&self, store_pwd: &str, salt: &str) -> Result<DecryptedRecord> {
        let decrypted_secrets = decrypt(self.value.clone(), store_pwd, salt)?;
        let (decoded_secrets, _len) =
            bincode::decode_from_slice(&decrypted_secrets[..], util::bincode_cfg())
                .map_err(|e| anyhow!("failed to decode secrets: {}", e))?;

        // Handle migrated records that have empty (unencrypted) history
        let decoded_history = if self.history.is_empty() {
            // Empty history from migration - just use empty vec
            vec![]
        } else {
            // Decrypt and decode the history
            let decrypted_history = decrypt(self.history.clone(), store_pwd, salt)?;
            let (decoded, _len) =
                bincode::decode_from_slice(&decrypted_history[..], util::bincode_cfg())
                    .map_err(|e| anyhow!("failed to decode history: {}", e))?;
            decoded
        };

        Ok(DecryptedRecord {
            secrets: decoded_secrets,
            metadata: self.metadata(),
            history: decoded_history,
        })
    }
}

/// Migrate an encrypted record from v0.8.0 to v0.9.0
///
/// Creates a v0.9.0 EncryptedRecord with an empty (unencrypted) history field.
/// This is safe because the decrypt() method checks for empty history and
/// treats it as an empty vec without attempting decryption.
pub fn migrate_encrypted_record_from_v080(er: v080::EncryptedRecord) -> EncryptedRecord {
    EncryptedRecord {
        key: er.key(),
        value: er.value(),
        metadata: er.metadata(),
        history: vec![], // Empty history - decrypt() handles this specially
    }
}

// Utility functions

pub fn key(category: &str, kind: Kind, name: &str, url: &str) -> String {
    format!("{name}:{url}:{kind:?}:{category}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;
    use rucksack_lib::time;

    #[test]
    fn password_records() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let dpr = testing::data::plaintext_record_v090();
        assert_eq!(
            format!("{}", dpr.secrets),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.secrets),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(&pwd, &salt).unwrap();
        assert_eq!(118, epr.value.len());
        let re_dpr = epr.decrypt(&pwd, &salt).unwrap();
        assert_eq!(re_dpr.secrets.password, "6 s3kr1t");
    }

    #[test]
    fn tags() {
        let mut dpr = testing::data::plaintext_record_v090();
        assert_eq!(dpr.metadata().tags, vec![]);
        let tag1 = "good stuff".to_string();
        dpr.add_tag(tag1.clone());
        assert_eq!(dpr.metadata().tags.len(), 1);
        assert_eq!(dpr.metadata().tags[0].value, tag1);
        let tag2 = "only the best".to_string();
        let tag3 = "bonus".to_string();
        dpr.add_tags(vec![tag2.clone(), tag3.clone()]);
        assert_eq!(dpr.metadata().tags.len(), 3);
        assert_eq!(dpr.metadata().tag_values(), vec![tag3, tag1, tag2]);
    }

    #[test]
    fn test_default_secrets() {
        let secrets = default_secrets();
        assert_eq!(secrets.user, "");
        assert_eq!(secrets.password, "");
        assert_eq!(secrets.account_id, "");
    }

    #[test]
    fn test_secrets_from_user_pass() {
        let secrets = secrets_from_user_pass("testuser", "testpass");
        assert_eq!(secrets.user, "testuser");
        assert_eq!(secrets.password, "testpass");
        assert_eq!(secrets.account_id, "");
    }

    #[test]
    fn test_default_metadata() {
        let metadata = default_metadata();
        assert_eq!(metadata.name, "");
        assert_eq!(metadata.category, DEFAULT_CATEGORY);
        assert_eq!(metadata.state, Status::default());
        assert_eq!(metadata.kind, Kind::default());
        assert_ne!(metadata.created, time::epoch_zero());
    }

    #[test]
    fn test_new_tag() {
        let tag = new_tag("test_tag".to_string());
        assert_eq!(tag.value, "test_tag");
        assert_ne!(tag.created, time::epoch_zero());
        assert_eq!(tag.updated, time::epoch_zero());
    }

    #[test]
    fn test_new_tags() {
        let values = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let tags = new_tags(values.clone());
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].value, "tag1");
        assert_eq!(tags[1].value, "tag2");
        assert_eq!(tags[2].value, "tag3");
    }

    #[test]
    fn test_new_tags_empty() {
        let tags = new_tags(vec![]);
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_new_history() {
        let secrets = default_secrets();
        let metadata = default_metadata();
        let history = new_history(secrets.clone(), metadata.clone());
        assert_eq!(history.secrets, secrets);
        assert_eq!(history.metadata, metadata);
    }

    #[test]
    fn test_decrypted_record_new() {
        let record = DecryptedRecord::new();
        assert_eq!(record.secrets.user, "");
        assert_eq!(record.secrets.password, "");
        assert_eq!(record.history.len(), 0);
    }

    #[test]
    fn test_decrypted_record_default() {
        let record = DecryptedRecord::default();
        assert_eq!(record.secrets.user, "");
        assert_eq!(record.history.len(), 0);
    }

    #[test]
    fn test_decrypted_record_getters() {
        let mut record = DecryptedRecord::new();
        record.secrets.user = "testuser".to_string();
        record.secrets.password = "testpass".to_string();
        record.metadata.name = "Test Name".to_string();
        record.metadata.url = "https://test.com".to_string();

        assert_eq!(record.user(), "testuser");
        assert_eq!(record.password(), "testpass");
        assert_eq!(record.name(), "Test Name");
        assert_eq!(record.url(), "https://test.com");
    }

    #[test]
    fn test_decrypted_record_name_or_user_with_name() {
        let mut record = DecryptedRecord::new();
        record.metadata.name = "My Name".to_string();
        record.secrets.user = "myuser".to_string();
        assert_eq!(record.name_or_user(), "My Name");
    }

    #[test]
    fn test_decrypted_record_name_or_user_without_name() {
        let mut record = DecryptedRecord::new();
        record.secrets.user = "myuser".to_string();
        assert_eq!(record.name_or_user(), "myuser");
    }

    #[test]
    fn test_decrypted_record_set_name() {
        let mut record = DecryptedRecord::new();
        let old_updated = record.metadata.updated.clone();

        record.set_name("New Name".to_string());
        assert_eq!(record.name(), "New Name");
        assert_ne!(record.metadata.updated, old_updated);
    }

    #[test]
    fn test_decrypted_record_set_user() {
        let mut record = DecryptedRecord::new();
        let old_updated = record.metadata.updated.clone();

        record.set_user("newuser".to_string());
        assert_eq!(record.user(), "newuser");
        assert_eq!(record.name(), "newuser");
        assert_ne!(record.metadata.updated, old_updated);
    }

    #[test]
    fn test_decrypted_record_set_password() {
        let mut record = DecryptedRecord::new();
        record.secrets.password = "oldpass".to_string();
        let old_updated = record.metadata.updated.clone();

        record.set_password("newpass".to_string());
        assert_eq!(record.password(), "newpass");
        assert_eq!(record.history.len(), 1);
        assert_eq!(record.history[0].secrets.password, "oldpass");
        assert_ne!(record.metadata.updated, old_updated);
        assert_ne!(record.metadata.password_changed, time::epoch_zero());
    }

    #[test]
    fn test_decrypted_record_set_url() {
        let mut record = DecryptedRecord::new();
        let old_updated = record.metadata.updated.clone();

        record.set_url("https://example.com".to_string());
        assert_eq!(record.url(), "https://example.com");
        assert_ne!(record.metadata.updated, old_updated);
    }

    #[test]
    fn test_decrypted_record_set_status() {
        let mut record = DecryptedRecord::new();
        let old_updated = record.metadata.updated.clone();

        record.set_status(Status::Active);
        assert_eq!(record.metadata.state, Status::Active);
        assert_ne!(record.metadata.updated, old_updated);
    }

    #[test]
    fn test_decrypted_record_set_kind() {
        let mut record = DecryptedRecord::new();
        let old_updated = record.metadata.updated.clone();

        record.set_kind(Kind::Account);
        assert_eq!(record.metadata.kind, Kind::Account);
        assert_ne!(record.metadata.updated, old_updated);
    }

    #[test]
    fn test_decrypted_record_key() {
        let mut record = DecryptedRecord::new();
        record.metadata.category = "test_cat".to_string();
        record.metadata.kind = Kind::Password;
        record.metadata.name = "testname".to_string();
        record.metadata.url = "test.com".to_string();

        let key = record.key();
        assert!(key.contains("testname"));
        assert!(key.contains("test.com"));
        assert!(key.contains("Password"));
        assert!(key.contains("test_cat"));
    }

    #[test]
    fn test_decrypted_record_key_with_pass() {
        let mut record = DecryptedRecord::new();
        record.metadata.name = "test".to_string();
        record.secrets.password = "mypass".to_string();

        let key_with_pass = record.key_with_pass();
        assert!(key_with_pass.ends_with(":mypass"));
    }

    #[test]
    fn test_decrypted_record_history() {
        let mut record = DecryptedRecord::new();
        record.secrets.password = "pass1".to_string();
        record.set_password("pass2".to_string());

        let history = record.history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].secrets.password, "pass1");
    }

    #[test]
    fn test_decrypted_record_metadata() {
        let record = DecryptedRecord::new();
        let metadata = record.metadata();
        assert_eq!(metadata.name, "");
        assert_eq!(metadata.category, DEFAULT_CATEGORY);
    }

    #[test]
    fn test_decrypted_record_add_tag() {
        let mut record = DecryptedRecord::new();
        record.add_tag("test_tag".to_string());
        assert_eq!(record.metadata.tags.len(), 1);
        assert_eq!(record.metadata.tags[0].value, "test_tag");
    }

    #[test]
    fn test_decrypted_record_add_tags() {
        let mut record = DecryptedRecord::new();
        record.add_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(record.metadata.tags.len(), 2);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let mut record = DecryptedRecord::new();
        record.secrets.user = "test@example.com".to_string();
        record.secrets.password = "secret123".to_string();
        record.metadata.name = "Test".to_string();

        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        assert_ne!(encrypted.value, vec![]);

        let decrypted = encrypted.decrypt(&pwd, &salt).unwrap();
        assert_eq!(decrypted.secrets.user, record.secrets.user);
        assert_eq!(decrypted.secrets.password, record.secrets.password);
        assert_eq!(decrypted.metadata.name, record.metadata.name);
    }

    #[test]
    fn test_encrypted_record_getters() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v090();
        let encrypted = record.encrypt(&pwd, &salt).unwrap();

        assert!(!encrypted.key().is_empty());
        assert!(!encrypted.value().is_empty());
        assert!(!encrypted.history().is_empty());
        assert_eq!(encrypted.metadata().name, record.metadata.name);
    }

    #[test]
    fn test_encrypted_record_add_tag() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = DecryptedRecord::new();
        let mut encrypted = record.encrypt(&pwd, &salt).unwrap();

        encrypted.add_tag("new_tag".to_string());
        assert_eq!(encrypted.metadata.tags.len(), 1);
        assert_eq!(encrypted.metadata.tags[0].value, "new_tag");
    }

    #[test]
    fn test_encrypted_record_add_tags() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = DecryptedRecord::new();
        let mut encrypted = record.encrypt(&pwd, &salt).unwrap();

        encrypted.add_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(encrypted.metadata.tags.len(), 2);
    }

    #[test]
    fn test_key_function() {
        let key = key("category", Kind::Password, "user", "example.com");
        assert!(key.contains("user"));
        assert!(key.contains("example.com"));
        assert!(key.contains("Password"));
        assert!(key.contains("category"));
    }

    #[test]
    fn test_key_function_empty_fields() {
        let key = key("", Kind::Password, "", "");
        assert!(key.contains("Password"));
    }

    #[test]
    fn test_migrate_secrets_from_v080() {
        let v080_secrets = v080::Secrets {
            account_id: "acc123".to_string(),
            user: "user@test.com".to_string(),
            password: "pass".to_string(),
            public_key: "pubkey".as_bytes().to_vec(),
            private_key: "privkey".as_bytes().to_vec(),
            public_cert: "pubcert".as_bytes().to_vec(),
            private_cert: "privcert".as_bytes().to_vec(),
            root_cert: "rootcert".as_bytes().to_vec(),
            key: "key".to_string(),
            secret: "secret".to_string(),
        };

        let migrated = migrate_secrets_from_v080(v080_secrets.clone());
        assert_eq!(migrated.account_id, v080_secrets.account_id);
        assert_eq!(migrated.user, v080_secrets.user);
        assert_eq!(migrated.password, v080_secrets.password);
    }

    #[test]
    fn test_encrypt_with_history() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let mut record = DecryptedRecord::new();
        record.secrets.password = "pass1".to_string();
        record.set_password("pass2".to_string());

        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        let decrypted = encrypted.decrypt(&pwd, &salt).unwrap();

        assert_eq!(decrypted.history.len(), 1);
        assert_eq!(decrypted.history[0].secrets.password, "pass1");
    }

    #[test]
    fn test_decode_hashmap_v090() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let hm: HashMap = dashmap::DashMap::new();

        let record = DecryptedRecord::new();
        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        hm.insert("test_key".to_string(), encrypted);

        // Serialize hashmap
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in hm.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by(|a, b| a.0.cmp(&b.0));
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        // Decode it
        let version = shared::version(VERSION).unwrap();
        let decoded_hm = decode_hashmap(&bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_empty() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = shared::version(VERSION).unwrap();
        let decoded_hm = decode_hashmap(&bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 0);
    }

    #[test]
    fn test_password_history_preserves_metadata() {
        let mut record = DecryptedRecord::new();
        record.metadata.name = "Test".to_string();
        record.secrets.password = "oldpass".to_string();

        record.set_password("newpass".to_string());

        assert_eq!(record.history[0].metadata.name, "Test");
        assert_eq!(record.history[0].secrets.password, "oldpass");
    }

    #[test]
    fn test_multiple_password_changes() {
        let mut record = DecryptedRecord::new();
        record.secrets.password = "pass1".to_string();
        record.set_password("pass2".to_string());
        record.set_password("pass3".to_string());

        assert_eq!(record.history.len(), 2);
        assert_eq!(record.history[0].secrets.password, "pass1");
        assert_eq!(record.history[1].secrets.password, "pass2");
        assert_eq!(record.secrets.password, "pass3");
    }
}
