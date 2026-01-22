use std::fmt;

use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use enum_iterator::Sequence;
use heck::ToKebabCase;
use secrecy::Zeroize;
use serde::{Deserialize, Serialize};

use rucksack_lib::{time, util};

use crate::crypto::{decrypt, encrypt};

use super::shared;
use super::v060;

pub const VERSION: &str = "0.7.0";
pub const DEFAULT_CATEGORY: &str = "default";
pub const ANY_CATEGORY: &str = "any";

// Enums

#[non_exhaustive]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Sequence,
)]
pub enum Kind {
    Account,
    Any,
    AsymmetricCrypto,
    Certificates,
    #[default]
    Password,
    ServiceCredentials,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn kinds() -> Vec<Kind> {
    enum_iterator::all::<Kind>().collect::<Vec<Kind>>()
}

pub fn types() -> Vec<String> {
    kinds()
        .iter()
        .map(|t| t.to_string().to_kebab_case())
        .collect::<Vec<String>>()
}

impl Kind {
    pub fn name(&self) -> String {
        format!("{self}")
    }
}

pub fn migrate_kind_from_v060(k: v060::Kind) -> Kind {
    match k {
        v060::Kind::Account => Kind::default(),
        v060::Kind::Credential => Kind::default(),
        v060::Kind::Password => Kind::Password,
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Status {
    #[default]
    Active,
    Any,
    Inactive,
    Deleted,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Active => "active",
            Status::Inactive => "inactive",
            Status::Deleted => "deleted",
            Status::Any => "any",
        }
    }
}

// Hashmap - the primary store data structure

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn migrate_hashmap_from_v060(hm_v060: v060::HashMap) -> HashMap {
    let hm: HashMap = dashmap::DashMap::new();
    for i in hm_v060.iter() {
        let r = i.value();
        let _ = hm.insert(
            i.key().to_string(),
            migrate_encrypted_record_from_v060(r.clone()),
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
        log::info!(version = "0.6.0", operation = "migrate"; "Attempting to decode hashmap from previous version");
        let hm = v060::decode_hashmap(&bytes, version)?;
        return Ok(migrate_hashmap_from_v060(hm));
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

#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Secrets {
    // Password- and account-based records
    pub account_id: String,
    pub user: String,
    pub password: String,
    // Asymmetric cryptography-based records
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    // Certificate-based records
    pub public_cert: Vec<u8>,
    pub private_cert: Vec<u8>,
    pub root_cert: Vec<u8>,
    // Service-credentials-based records
    pub key: String,
    pub secret: String,
}

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

impl Zeroize for Secrets {
    fn zeroize(&mut self) {
        self.password.zeroize();
        self.private_key.zeroize();
        self.private_cert.zeroize();
        self.key.zeroize();
        self.secret.zeroize();
    }
}

impl std::fmt::Display for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        display_creds(self, f)
    }
}

impl std::fmt::Debug for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        display_creds(self, f)
    }
}

fn display_creds(sef: &Secrets, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if !sef.account_id.is_empty() && !sef.user.is_empty() && !sef.password.is_empty() {
        write!(
            f,
            "Creds{{account_id: {} user: {}, password: *****}}",
            sef.account_id, sef.user
        )
    } else if !sef.user.is_empty() && !sef.password.is_empty() {
        write!(f, "Creds{{user: {}, password: *****}}", sef.user)
    } else if !sef.key.is_empty() {
        write!(f, "Creds{{key: {}, secret: *****}}", sef.key)
    } else if !sef.private_cert.is_empty() {
        write!(
            f,
            "Creds{{public_cert: {:?}, private_cert: *****}}",
            sef.public_cert
        )
    } else if !sef.private_key.is_empty() {
        write!(
            f,
            "Creds{{public_key: {:?}, private_key: *****}}",
            sef.public_key
        )
    } else {
        write!(f, "Creds{{data: *****}}")
    }
}

pub fn migrate_secrets_from_v060(creds_v060: v060::Creds) -> Secrets {
    Secrets {
        user: creds_v060.user,
        password: creds_v060.password,
        ..Default::default()
    }
}

// Metadata

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Tag {
    pub display: String,
    pub value: String,
    pub created: String,
    pub updated: String,
    pub state: Status,
}

impl Tag {
    pub fn status(&self) -> &str {
        self.state.as_str()
    }
}

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

impl Tag {
    pub fn display_or_value(&self) -> String {
        if !self.display.is_empty() {
            return self.display.clone();
        }
        self.value.clone()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Metadata {
    pub kind: Kind,
    pub category: String,
    pub name: String,
    pub url: String,
    pub created: String,
    pub imported: String,
    pub updated: String,
    pub password_changed: String,
    pub last_used: String,
    pub synced: String,
    pub access_count: u64,
    pub state: Status,
    pub tags: Vec<Tag>,
}

impl Metadata {
    pub fn status(&self) -> &str {
        self.state.as_str()
    }

    pub fn add_tag(&mut self, value: String) {
        self.tags.push(new_tag(value));
        self.sort_tags()
    }

    pub fn add_tags(&mut self, values: Vec<String>) {
        self.tags.append(new_tags(values).as_mut());
        self.sort_tags()
    }

    fn sort_tags(&mut self) {
        self.tags.sort_by_key(|a| a.display_or_value())
    }

    pub fn tag_values(&self) -> Vec<String> {
        self.tags
            .clone()
            .into_iter()
            .map(|t| t.value)
            .collect::<Vec<String>>()
    }
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

pub fn migrate_metadata_from_v060(md6: v060::Metadata, name: String) -> Metadata {
    let mut md = default_metadata();
    md.kind = migrate_kind_from_v060(md6.kind);
    md.name = name;
    md.url = md6.url;
    md.created = md6.created;
    md.imported = md6.imported;
    md.updated = md6.updated;
    md.password_changed = md6.password_changed;
    md.last_used = md6.last_used;
    md.access_count = md6.access_count;
    md
}

// Decrypted records

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub secrets: Secrets,
    pub metadata: Metadata,
}

impl DecryptedRecord {
    pub fn add_tag(&mut self, value: String) {
        self.metadata.add_tag(value)
    }

    pub fn add_tags(&mut self, values: Vec<String>) {
        self.metadata.add_tags(values)
    }

    pub fn key(&self) -> String {
        let md = self.metadata.clone();
        let mut name = md.name.clone();
        if name.is_empty() {
            name = self.secrets.user.clone();
        }
        key(
            md.category.as_str(),
            md.kind.clone(),
            name.as_str(),
            md.url.as_str(),
        )
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

    pub fn user(&self) -> String {
        self.secrets.user.clone()
    }

    pub fn encrypt(&self, store_pwd: &str, salt: &str) -> Result<EncryptedRecord> {
        let encoded = bincode::encode_to_vec(&self.secrets, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, store_pwd, salt)?;

        Ok(EncryptedRecord {
            key: self.key(),
            value: encrypted,
            metadata: self.metadata(),
        })
    }
}

pub fn migrate_decrypted_record_from_v060(dr: v060::DecryptedRecord) -> DecryptedRecord {
    DecryptedRecord {
        secrets: migrate_secrets_from_v060(dr.creds.clone()),
        metadata: migrate_metadata_from_v060(dr.metadata.clone(), name_from_key(dr.key())),
    }
}
// Encrypted records

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct EncryptedRecord {
    pub key: String,
    pub value: Vec<u8>,
    pub metadata: Metadata,
}

impl EncryptedRecord {
    pub fn add_tag(&mut self, value: String) {
        self.metadata.add_tag(value)
    }

    pub fn add_tags(&mut self, values: Vec<String>) {
        self.metadata.add_tags(values)
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
        let decrypted = decrypt(self.value.clone(), store_pwd, salt)?;
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], util::bincode_cfg()).unwrap();

        Ok(DecryptedRecord {
            secrets: decoded,
            metadata: self.metadata(),
        })
    }
}

pub fn migrate_encrypted_record_from_v060(er: v060::EncryptedRecord) -> EncryptedRecord {
    let key = er.key();
    EncryptedRecord {
        key: key.clone(),
        value: er.value(),
        metadata: migrate_metadata_from_v060(er.metadata(), name_from_key(key)),
    }
}

// Utility functions

pub fn key(category: &str, kind: Kind, name: &str, url: &str) -> String {
    format!("{category}:{kind:?}:{name}:{url}")
}

pub fn name_from_key(key: String) -> String {
    let parts: Vec<&str> = key.split(':').collect();
    parts[0].to_string()
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;
    use rucksack_lib::time;

    #[test]
    fn password_records() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let dpr = testing::data::plaintext_record_v070();
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
        assert_eq!(re_dpr.secrets.password, "4 s3kr1t");
    }

    #[test]
    fn tags() {
        let mut dpr = testing::data::plaintext_record_v070();
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
    fn test_kinds() {
        let kind_list = kinds();
        assert!(!kind_list.is_empty());
        assert!(kind_list.contains(&Kind::Password));
        assert!(kind_list.contains(&Kind::Account));
    }

    #[test]
    fn test_types() {
        let type_list = types();
        assert!(!type_list.is_empty());
        // types() returns kebab-case strings
        assert!(type_list.iter().any(|t| t.contains("password")));
        assert!(type_list.iter().any(|t| t.contains("account")));
    }

    #[test]
    fn test_default_secrets() {
        let secrets = default_secrets();
        assert_eq!(secrets.user, "");
        assert_eq!(secrets.password, "");
    }

    #[test]
    fn test_secrets_from_user_pass() {
        let secrets = secrets_from_user_pass("testuser", "testpass");
        assert_eq!(secrets.user, "testuser");
        assert_eq!(secrets.password, "testpass");
    }

    #[test]
    fn test_new_tag() {
        let tag = new_tag("test_tag".to_string());
        assert_eq!(tag.value, "test_tag");
        assert_ne!(tag.created, time::epoch_zero());
    }

    #[test]
    fn test_new_tags() {
        let tags = new_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].value, "tag1");
    }

    #[test]
    fn test_new_tags_empty() {
        let tags = new_tags(vec![]);
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_default_metadata() {
        let metadata = default_metadata();
        assert_eq!(metadata.name, "");
        assert_eq!(metadata.category, DEFAULT_CATEGORY);
        assert_ne!(metadata.created, time::epoch_zero());
    }

    #[test]
    fn test_key_function() {
        let key = key("test_cat", Kind::Password, "user", "example.com");
        assert!(key.contains("test_cat"));
        assert!(key.contains("Password"));
        assert!(key.contains("user"));
        assert!(key.contains("example.com"));
    }

    #[test]
    fn test_key_function_empty() {
        let key = key("", Kind::Password, "", "");
        assert!(key.contains("Password"));
    }

    #[test]
    fn test_name_from_key() {
        // name_from_key is for v060-style keys: "user:url"
        let key = "username:example.com".to_string();
        let name = name_from_key(key);
        assert_eq!(name, "username");
    }

    #[test]
    fn test_name_from_key_empty() {
        let key = ":".to_string();
        let name = name_from_key(key);
        assert_eq!(name, "");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v070();

        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        assert!(!encrypted.value.is_empty());

        let decrypted = encrypted.decrypt(&pwd, &salt).unwrap();
        assert_eq!(decrypted.secrets.user, record.secrets.user);
        assert_eq!(decrypted.secrets.password, record.secrets.password);
    }

    #[test]
    fn test_decode_hashmap_v070() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let hm: HashMap = dashmap::DashMap::new();

        let record = testing::data::plaintext_record_v070();
        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        hm.insert("test_key".to_string(), encrypted);

        // Serialize
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in hm.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        // Decode
        let version = shared::version(VERSION).unwrap();
        let decoded = decode_hashmap(&bytes, version).unwrap();
        assert_eq!(decoded.len(), 1);
    }

    #[test]
    fn test_decode_hashmap_empty() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = shared::version(VERSION).unwrap();
        let decoded = decode_hashmap(&bytes, version).unwrap();
        assert_eq!(decoded.len(), 0);
    }

    #[test]
    fn test_decrypted_record_methods() {
        let mut record = testing::data::plaintext_record_v070();

        // Test getters
        assert!(!record.user().is_empty());
        assert!(!record.password().is_empty());
        assert!(!record.key().is_empty());
        // Name comes from v060 migration, where user becomes name
        assert_eq!(record.name(), "alice@site.com");

        // Test name_or_user
        let name_or_user = record.name_or_user();
        assert_eq!(name_or_user, "alice@site.com");

        // Test direct field modification (no setters in v070)
        record.secrets.user = "newuser".to_string();
        assert_eq!(record.user(), "newuser");

        record.secrets.password = "newpass".to_string();
        assert_eq!(record.password(), "newpass");

        record.metadata.url = "newurl.com".to_string();
        record.metadata.name = "TestName".to_string();
        let key = record.key();
        assert!(key.contains("TestName"));
        assert!(key.contains("newurl.com"));
    }

    #[test]
    fn test_encrypted_record_methods() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v070();
        let encrypted = record.encrypt(&pwd, &salt).unwrap();

        assert!(!encrypted.key().is_empty());
        assert!(!encrypted.value().is_empty());
        let metadata = encrypted.metadata();
        assert_eq!(metadata.category, record.metadata.category);
    }

    #[test]
    fn test_tag_operations() {
        let mut record = testing::data::plaintext_record_v070();

        record.add_tag("newtag".to_string());
        assert!(record.metadata.tags.len() > 0);

        record.add_tags(vec!["tag2".to_string(), "tag3".to_string()]);
        let tag_values = record.metadata.tag_values();
        assert!(tag_values.len() >= 3);
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, "0.7.0");
        let version = shared::version(VERSION).unwrap();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 7);
    }

    #[test]
    fn test_kind_name() {
        assert_eq!(Kind::Password.name(), "Password");
        assert_eq!(Kind::Account.name(), "Account");
        assert_eq!(Kind::AsymmetricCrypto.name(), "AsymmetricCrypto");
    }

    #[test]
    fn test_migrate_kind_from_v060() {
        assert_eq!(migrate_kind_from_v060(v060::Kind::Password), Kind::Password);
        assert_eq!(migrate_kind_from_v060(v060::Kind::Account), Kind::default());
        assert_eq!(
            migrate_kind_from_v060(v060::Kind::Credential),
            Kind::default()
        );
    }

    #[test]
    fn test_status_as_str() {
        assert_eq!(Status::Active.as_str(), "active");
        assert_eq!(Status::Inactive.as_str(), "inactive");
        assert_eq!(Status::Deleted.as_str(), "deleted");
        assert_eq!(Status::Any.as_str(), "any");
    }

    #[test]
    fn test_migrate_hashmap_from_v060() {
        let hm_v060: v060::HashMap = dashmap::DashMap::new();
        let pwd = testing::data::store_pwd();
        let salt = time::now();

        let record_v060 = testing::data::plaintext_record_v060();
        let encrypted_v060 = record_v060.encrypt(&pwd, &salt).unwrap();
        hm_v060.insert("test_key".to_string(), encrypted_v060);

        let hm_v070 = migrate_hashmap_from_v060(hm_v060);
        assert_eq!(hm_v070.len(), 1);
        assert!(hm_v070.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_from_v060() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let hm_v060: v060::HashMap = dashmap::DashMap::new();

        let record = testing::data::plaintext_record_v060();
        let encrypted = record.encrypt(&pwd, &salt).unwrap();
        hm_v060.insert("v060_key".to_string(), encrypted);

        let mut data: Vec<(String, v060::EncryptedRecord)> = Vec::new();
        for i in hm_v060.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = shared::version("0.6.0").unwrap();
        let decoded_hm = decode_hashmap(&bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("v060_key"));
    }

    #[test]
    fn test_decode_hashmap_error() {
        let invalid_bytes = vec![1, 2, 3, 4, 5];
        let version = shared::version(VERSION).unwrap();
        let result = decode_hashmap(&invalid_bytes, version);
        assert!(result.is_err());
    }

    #[test]
    fn test_migrate_secrets_from_v060() {
        let v060_creds = v060::Creds {
            user: "testuser".to_string(),
            password: "testpass".to_string(),
        };
        let secrets = migrate_secrets_from_v060(v060_creds.clone());
        assert_eq!(secrets.user, v060_creds.user);
        assert_eq!(secrets.password, v060_creds.password);
        assert_eq!(secrets.account_id, "");
    }

    #[test]
    fn test_migrate_metadata_from_v060() {
        let now = time::now();
        let md_v060 = v060::Metadata {
            kind: v060::Kind::Password,
            url: "example.com".to_string(),
            created: now.clone(),
            imported: time::epoch_zero(),
            updated: now.clone(),
            password_changed: time::epoch_zero(),
            last_used: time::epoch_zero(),
            access_count: 5,
        };
        let md = migrate_metadata_from_v060(md_v060.clone(), "TestName".to_string());
        assert_eq!(md.name, "TestName");
        assert_eq!(md.url, md_v060.url);
        assert_eq!(md.access_count, md_v060.access_count);
        assert_eq!(md.kind, Kind::Password);
    }

    #[test]
    fn test_migrate_decrypted_record_from_v060() {
        let record_v060 = testing::data::plaintext_record_v060();
        let record_v070 = migrate_decrypted_record_from_v060(record_v060.clone());
        assert_eq!(record_v070.secrets.user, record_v060.creds.user);
        assert_eq!(record_v070.secrets.password, record_v060.creds.password);
    }

    #[test]
    fn test_migrate_encrypted_record_from_v060() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record_v060 = testing::data::plaintext_record_v060();
        let encrypted_v060 = record_v060.encrypt(&pwd, &salt).unwrap();

        let encrypted_v070 = migrate_encrypted_record_from_v060(encrypted_v060.clone());
        assert_eq!(encrypted_v070.key(), encrypted_v060.key());
        assert_eq!(encrypted_v070.value(), encrypted_v060.value());
    }

    #[test]
    fn test_secrets_display_with_account_id() {
        let secrets = Secrets {
            account_id: "acc123".to_string(),
            user: "user@example.com".to_string(),
            password: "pass".to_string(),
            ..Default::default()
        };
        let display = format!("{}", secrets);
        assert!(display.contains("account_id"));
        assert!(display.contains("user@example.com"));
        assert!(display.contains("*****"));
    }

    #[test]
    fn test_secrets_display_with_key() {
        let secrets = Secrets {
            key: "api_key".to_string(),
            secret: "api_secret".to_string(),
            ..Default::default()
        };
        let display = format!("{}", secrets);
        assert!(display.contains("key"));
        assert!(display.contains("api_key"));
        assert!(display.contains("*****"));
    }

    #[test]
    fn test_secrets_display_with_certs() {
        let secrets = Secrets {
            public_cert: vec![1, 2, 3],
            private_cert: vec![4, 5, 6],
            ..Default::default()
        };
        let display = format!("{}", secrets);
        assert!(display.contains("cert"));
        assert!(display.contains("*****"));
    }

    #[test]
    fn test_secrets_display_with_keys() {
        let secrets = Secrets {
            public_key: vec![1, 2, 3],
            private_key: vec![4, 5, 6],
            ..Default::default()
        };
        let display = format!("{}", secrets);
        assert!(display.contains("key"));
        assert!(display.contains("*****"));
    }

    #[test]
    fn test_secrets_zeroize() {
        use secrecy::Zeroize;
        let mut secrets = Secrets {
            user: "user".to_string(),
            password: "password".to_string(),
            private_key: vec![1, 2, 3],
            private_cert: vec![4, 5, 6],
            key: "key".to_string(),
            secret: "secret".to_string(),
            ..Default::default()
        };
        secrets.zeroize();
        // After zeroize, sensitive fields should be cleared
        assert_eq!(secrets.password, "");
        assert_eq!(secrets.key, "");
        assert_eq!(secrets.secret, "");
    }
}
