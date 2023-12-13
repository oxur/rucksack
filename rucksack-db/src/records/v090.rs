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
        log::info!("Attempting to decode hashmap from previous version (0.8.0)");
        let hm = v080::decode_hashmap(bytes, version)?;
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
            log::info!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
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

    pub fn encrypt(&self, store_pwd: String, salt: String) -> EncryptedRecord {
        let encoded_secrets = bincode::encode_to_vec(&self.secrets, util::bincode_cfg()).unwrap();
        let encrypted_secrets = encrypt(encoded_secrets, store_pwd.clone(), salt.clone());

        let encoded_history = bincode::encode_to_vec(&self.history, util::bincode_cfg()).unwrap();
        let encrypted_history = encrypt(encoded_history, store_pwd, salt);

        EncryptedRecord {
            key: self.key(),
            value: encrypted_secrets,
            metadata: self.metadata(),
            history: encrypted_history,
        }
    }

    pub fn history(&self) -> Vec<History> {
        self.history.clone()
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

    pub fn decrypt(&self, store_pwd: String, salt: String) -> Result<DecryptedRecord> {
        let decrypted_secrets = decrypt(self.value.clone(), store_pwd.clone(), salt.clone())?;
        let (decoded_secrets, _len) =
            bincode::decode_from_slice(&decrypted_secrets[..], util::bincode_cfg()).unwrap();

        let decrypted_history = decrypt(self.history.clone(), store_pwd, salt)?;
        let (decoded_history, _len) =
            bincode::decode_from_slice(&decrypted_history[..], util::bincode_cfg()).unwrap();

        Ok(DecryptedRecord {
            secrets: decoded_secrets,
            metadata: self.metadata(),
            history: decoded_history,
        })
    }
}

pub fn migrate_encrypted_record_from_v080(er: v080::EncryptedRecord) -> EncryptedRecord {
    EncryptedRecord {
        key: er.key(),
        value: er.value(),
        metadata: er.metadata(),
        history: vec![],
    }
}

// Utility functions

pub fn key(category: &str, kind: Kind, name: &str, url: &str) -> String {
    format!("{name}:{url}:{kind:?}:{category}")
}

#[cfg(test)]
mod tests {
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
        let epr = dpr.encrypt(pwd.clone(), salt.clone());
        assert_eq!(118, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
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
}
