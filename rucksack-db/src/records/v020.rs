use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use secrecy::Zeroize;
use serde::{Deserialize, Serialize};

use rucksack_lib::util;

use crate::crypto::{decrypt, encrypt};

pub const VERSION: &str = "0.2.0";

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn decode_hashmap(bytes: Vec<u8>, version: versions::SemVer) -> Result<HashMap> {
    log::debug!("Decoding hashmap from stored bytes (format version {version:})...");
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!("Created hashmap.");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!("Created vec for sorted data.");
    match bincode::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
        Ok((result, _len)) => {
            sorted_vec = result;
            for (key, val) in sorted_vec {
                if hm.insert(key.clone(), val).is_some() {}
            }
            Ok(hm)
        }
        Err(e) => {
            let msg = format!("couldn't deserialise bincoded hashmap bytes: {e:?}");
            log::error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

// Structs and Enums

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Kind {
    #[default]
    Password,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Metadata {
    pub kind: Kind,
    pub url: String,
    pub created: String,
    pub updated: String,
    pub password_changed: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Creds {
    pub user: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct EncryptedRecord {
    pub key: String,
    pub value: Vec<u8>,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub key: String,
    pub value: Creds,
    pub metadata: Metadata,
}

// Traits and methods

impl Zeroize for Creds {
    fn zeroize(&mut self) {
        self.password.zeroize();
    }
}

impl std::fmt::Display for Creds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Creds{{user: {}, password: *****}}", self.user)
    }
}

impl std::fmt::Debug for Creds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Creds{{user: {}, password: *****}}", self.user)
    }
}

impl DecryptedRecord {
    pub fn encrypt(&self, store_pwd: String) -> EncryptedRecord {
        let encoded = bincode::encode_to_vec(&self.value, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, store_pwd, self.metadata.updated.clone());

        EncryptedRecord {
            key: self.key.clone(),
            value: encrypted,
            metadata: self.metadata.clone(),
        }
    }
}

impl EncryptedRecord {
    pub fn decrypt(&self, store_pwd: String) -> DecryptedRecord {
        let decrypted =
            decrypt(self.value.clone(), store_pwd, self.metadata.updated.clone()).unwrap();
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], util::bincode_cfg()).unwrap();

        DecryptedRecord {
            key: self.key.clone(),
            value: decoded,
            metadata: self.metadata.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn password_records() {
        let store_pwd = "abc123".to_string();
        let now = chrono::offset::Local::now().to_rfc3339();
        let dpr = super::DecryptedRecord {
            key: "a site".to_string(),
            value: super::Creds {
                user: "alice@site.com".to_string(),
                password: "4 s3kr1t".to_string(),
            },
            metadata: super::Metadata {
                kind: super::Kind::Password,
                url: "https://site.com/".to_string(),
                created: now.clone(),
                updated: now.clone(),
                password_changed: now,
            },
        };
        assert_eq!(
            format!("{}", dpr.value),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.value),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(store_pwd.clone());
        assert_eq!(54, epr.value.len());
        let re_dpr = epr.decrypt(store_pwd);
        assert_eq!(re_dpr.value.password, "4 s3kr1t");
    }
}
