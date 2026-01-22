use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use rucksack_lib::util;

use crate::crypto::{decrypt, encrypt};

use super::v020;
pub use super::v020::{Creds, Kind};

pub const VERSION: &str = "0.3.0";

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn decode_hashmap(bytes: Vec<u8>, version: versions::SemVer) -> Result<HashMap> {
    log::debug!(version = version.to_string().as_str(), operation = "decode"; "Decoding hashmap from stored bytes");
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!(operation = "decode"; "Created hashmap");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!(operation = "decode"; "Created vec for sorted data");
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
            log::error!(error = e.to_string().as_str(), operation = "decode"; "{}", msg);
            Err(anyhow!(msg))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Metadata {
    pub kind: Kind,
    pub url: String,
    pub created: String,
    pub imported: String,
    pub updated: String,
    pub password_changed: String,
    pub last_used: String,
    pub access_count: u64,
}

pub fn migrate_metadata_from_v020(md: v020::Metadata) -> Metadata {
    Metadata {
        kind: md.kind,
        url: md.url,
        created: md.created,
        imported: String::new(),
        updated: md.updated,
        password_changed: md.password_changed,
        last_used: String::new(),
        access_count: 0,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub creds: Creds,
    pub metadata: Metadata,
}

pub fn migrate_decrypted_record_from_v020(dr: v020::DecryptedRecord) -> DecryptedRecord {
    DecryptedRecord {
        creds: dr.value,
        metadata: migrate_metadata_from_v020(dr.metadata),
    }
}

impl DecryptedRecord {
    pub fn key(&self) -> String {
        format!("{}:{}", self.creds.user, self.metadata.url)
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn encrypt(&self, store_pwd: &str) -> Result<EncryptedRecord> {
        let encoded = bincode::encode_to_vec(&self.creds, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, store_pwd, &self.metadata().updated)?;

        Ok(EncryptedRecord {
            key: self.key(),
            value: encrypted,
            metadata: self.metadata(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct EncryptedRecord {
    pub key: String,
    pub value: Vec<u8>,
    pub metadata: Metadata,
}

impl EncryptedRecord {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn decrypt(&self, store_pwd: &str) -> Result<DecryptedRecord> {
        let decrypted = decrypt(self.value.clone(), store_pwd, &self.metadata().updated)?;
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], util::bincode_cfg()).unwrap();

        Ok(DecryptedRecord {
            creds: decoded,
            metadata: self.metadata(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rucksack_lib::time;

    fn test_creds() -> Creds {
        Creds {
            user: "testuser".to_string(),
            password: "testpass".to_string(),
        }
    }

    fn test_metadata() -> Metadata {
        Metadata {
            kind: Kind::Password,
            url: "https://example.com".to_string(),
            created: time::now(),
            imported: time::epoch_zero(),
            updated: time::now(),
            password_changed: time::epoch_zero(),
            last_used: time::epoch_zero(),
            access_count: 0,
        }
    }

    fn test_decrypted_record() -> DecryptedRecord {
        DecryptedRecord {
            creds: test_creds(),
            metadata: test_metadata(),
        }
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, "0.3.0");
    }

    #[test]
    fn test_decrypted_record_key() {
        let record = test_decrypted_record();
        let key = record.key();
        assert!(key.contains("testuser"));
        assert!(key.contains("example.com"));
    }

    #[test]
    fn test_decrypted_record_metadata() {
        let record = test_decrypted_record();
        let metadata = record.metadata();
        assert_eq!(metadata.url, "https://example.com");
        assert_eq!(metadata.kind, Kind::Password);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let record = test_decrypted_record();
        let pwd = "store_password".to_string();

        let encrypted = record.encrypt(&pwd).unwrap();
        assert!(!encrypted.value.is_empty());
        assert_eq!(encrypted.key(), record.key());

        let decrypted = encrypted.decrypt(&pwd).unwrap();
        assert_eq!(decrypted.creds.user, record.creds.user);
        assert_eq!(decrypted.creds.password, record.creds.password);
    }

    #[test]
    fn test_encrypted_record_key() {
        let record = test_decrypted_record();
        let encrypted = record.encrypt("password").unwrap();
        assert_eq!(encrypted.key(), record.key());
    }

    #[test]
    fn test_encrypted_record_metadata() {
        let record = test_decrypted_record();
        let encrypted = record.encrypt("password").unwrap();
        let metadata = encrypted.metadata();
        assert_eq!(metadata.url, record.metadata.url);
    }

    #[test]
    fn test_decode_hashmap() {
        let hm: HashMap = dashmap::DashMap::new();
        let record = test_decrypted_record();
        let encrypted = record.encrypt("password").unwrap();
        hm.insert("test_key".to_string(), encrypted);

        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in hm.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = versions::SemVer::new(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_empty() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = versions::SemVer::new(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 0);
    }

    #[test]
    fn test_decode_hashmap_error() {
        let invalid_bytes = vec![1, 2, 3, 4, 5];
        let version = versions::SemVer::new(VERSION).unwrap();
        let result = decode_hashmap(invalid_bytes, version);
        assert!(result.is_err());
    }

    #[test]
    fn test_migrate_metadata_from_v020() {
        let md_v020 = v020::Metadata {
            kind: Kind::Password,
            url: "https://test.com".to_string(),
            created: "2024-01-01".to_string(),
            updated: "2024-01-02".to_string(),
            password_changed: "2024-01-03".to_string(),
        };

        let md_v030 = migrate_metadata_from_v020(md_v020.clone());
        assert_eq!(md_v030.kind, md_v020.kind);
        assert_eq!(md_v030.url, md_v020.url);
        assert_eq!(md_v030.created, md_v020.created);
        assert_eq!(md_v030.updated, md_v020.updated);
        assert_eq!(md_v030.password_changed, md_v020.password_changed);
        assert_eq!(md_v030.imported, "");
        assert_eq!(md_v030.last_used, "");
        assert_eq!(md_v030.access_count, 0);
    }

    #[test]
    fn test_migrate_decrypted_record_from_v020() {
        let dr_v020 = v020::DecryptedRecord {
            key: "user:https://test.com".to_string(),
            value: Creds {
                user: "user".to_string(),
                password: "pass".to_string(),
            },
            metadata: v020::Metadata {
                kind: Kind::Password,
                url: "https://test.com".to_string(),
                created: time::now(),
                updated: time::now(),
                password_changed: time::epoch_zero(),
            },
        };

        let dr_v030 = migrate_decrypted_record_from_v020(dr_v020.clone());
        assert_eq!(dr_v030.creds.user, dr_v020.value.user);
        assert_eq!(dr_v030.creds.password, dr_v020.value.password);
        assert_eq!(dr_v030.metadata.url, dr_v020.metadata.url);
    }
}
