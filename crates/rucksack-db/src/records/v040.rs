use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use rucksack_lib::util;

use crate::crypto::{decrypt, encrypt};

pub use super::v030::{Creds, Kind, Metadata};

pub const VERSION: &str = "0.4.0";

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

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct DecryptedRecord {
    pub creds: Creds,
    pub metadata: Metadata,
}

impl DecryptedRecord {
    pub fn key(&self) -> String {
        format!("{}:{}", self.creds.user, self.metadata.url)
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn password(&self) -> String {
        self.creds.password.clone()
    }

    pub fn user(&self) -> String {
        self.creds.user.clone()
    }

    pub fn encrypt(&self, store_pwd: String, salt: String) -> Result<EncryptedRecord> {
        let encoded = bincode::encode_to_vec(&self.creds, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, store_pwd, salt)?;

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
            creds: decoded,
            metadata: self.metadata(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rucksack_lib::time;

    use crate::testing;

    #[test]
    fn password_records() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let dpr = testing::data::plaintext_record_v040();
        assert_eq!(
            format!("{}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(pwd.clone(), salt.clone()).unwrap();
        assert_eq!(54, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, "0.4.0");
    }

    #[test]
    fn test_decrypted_record_key() {
        let record = testing::data::plaintext_record_v040();
        let key = record.key();
        assert!(key.contains("alice@site.com"));
        assert!(key.contains("site.com"));
    }

    #[test]
    fn test_decrypted_record_metadata() {
        let record = testing::data::plaintext_record_v040();
        let metadata = record.metadata();
        assert_eq!(metadata.kind, Kind::Password);
    }

    #[test]
    fn test_decrypted_record_user() {
        let record = testing::data::plaintext_record_v040();
        assert_eq!(record.user(), "alice@site.com");
    }

    #[test]
    fn test_decrypted_record_password() {
        let record = testing::data::plaintext_record_v040();
        assert_eq!(record.password(), "4 s3kr1t");
    }

    #[test]
    fn test_encrypted_record_key() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v040();
        let encrypted = record.encrypt(pwd, salt).unwrap();
        let key = encrypted.key();
        assert!(key.contains("alice@site.com"));
    }

    #[test]
    fn test_encrypted_record_metadata() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v040();
        let encrypted = record.encrypt(pwd, salt).unwrap();
        let metadata = encrypted.metadata();
        assert_eq!(metadata.kind, Kind::Password);
    }

    #[test]
    fn test_encrypted_record_value() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v040();
        let encrypted = record.encrypt(pwd, salt).unwrap();
        let value = encrypted.value();
        assert!(!value.is_empty());
    }

    #[test]
    fn test_decode_hashmap() {
        let hm: HashMap = dashmap::DashMap::new();
        let record = testing::data::plaintext_record_v040();
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let encrypted = record.encrypt(pwd, salt).unwrap();
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
}
