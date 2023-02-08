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

pub fn migrate_kind_from_v060(k: v060::Kind) -> Kind {
    match k {
        v060::Kind::Account => Kind::default(),
        v060::Kind::Credential => Kind::default(),
        v060::Kind::Password => Kind::Password,
    }
}

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

pub fn decode_hashmap(bytes: Vec<u8>, mut version: versions::SemVer) -> Result<HashMap> {
    log::debug!(
        "Decoding hashmap from stored bytes (format version {:})...",
        version
    );
    version = shared::trim_version(version);
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!("Created hashmap.");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!("Created vec for sorted data.");
    if version < shared::version(VERSION) {
        // version.
        log::info!("Attempting to decode hashmap from previous version (0.6.0)");
        let hm = v060::decode_hashmap(bytes, version)?;
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
            log::info!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
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
        let mut name = self.metadata.name.clone();
        if name.is_empty() {
            name = self.secrets.user.clone();
        }
        key(
            self.metadata.category.as_str(),
            self.metadata.kind.clone(),
            name.as_str(),
            self.metadata.url.as_str(),
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

    pub fn encrypt(&self, store_pwd: String, salt: String) -> EncryptedRecord {
        let encoded = bincode::encode_to_vec(&self.secrets, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, store_pwd, salt);

        EncryptedRecord {
            key: self.key(),
            value: encrypted,
            metadata: self.metadata(),
        }
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

    pub fn decrypt(&self, store_pwd: String, salt: String) -> Result<DecryptedRecord> {
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
    let parts: Vec<&str> = key.split(':').collect();
    EncryptedRecord {
        key: er.key(),
        value: er.value(),
        metadata: migrate_metadata_from_v060(er.metadata(), parts[0].to_string()),
    }
}

pub fn key(category: &str, kind: Kind, name: &str, url: &str) -> String {
    format!("{category}:{kind:?}:{name}:{url}")
}

// Tests

#[cfg(test)]
mod tests {
    use rucksack_lib::time;

    use crate::testing;

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
        let epr = dpr.encrypt(pwd.clone(), salt.clone());
        assert_eq!(118, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
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
}
