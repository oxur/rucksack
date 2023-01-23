use anyhow::Result;
use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::store::crypto::{decrypt, encrypt};

use super::v060;
pub use super::v060::{Creds, Kind, DEFAULT_KIND};

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

pub fn decode_hashmap(bytes: Vec<u8>) -> Result<HashMap> {
    let hashmap: HashMap;
    match bincode::serde::decode_from_slice(bytes.as_ref(), config::standard()) {
        Ok((result, _len)) => {
            hashmap = result;
            Ok(hashmap)
        }
        Err(e) => {
            log::info!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
            log::info!("Attempting to decode hashmap from previous version (0.6.0)");
            let hm = v060::decode_hashmap(bytes)?;
            Ok(migrate_hashmap_from_v060(hm))
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct Metadata {
    pub kind: Kind,
    pub url: String,
    pub created: String,
    pub imported: String,
    pub updated: String,
    pub password_changed: String,
    pub last_used: String,
    pub synced: String,
    pub access_count: u64,
}

pub fn migrate_metadata_from_v060(md: v060::Metadata) -> Metadata {
    Metadata {
        kind: v060::migrate_kind_from_v050(md.kind),
        url: md.url,
        created: md.created,
        imported: md.imported,
        updated: md.updated,
        password_changed: md.password_changed,
        last_used: md.last_used,
        synced: String::new(),
        access_count: md.access_count,
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

    pub fn encrypt(&self, prime_pwd: String, salt: String) -> EncryptedRecord {
        let encoded = bincode::encode_to_vec(&self.creds, config::standard()).unwrap();
        let encrypted = encrypt(encoded, prime_pwd, salt);

        EncryptedRecord {
            key: self.key(),
            value: encrypted,
            metadata: self.metadata(),
        }
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

    pub fn decrypt(&self, prime_pwd: String, salt: String) -> Result<DecryptedRecord> {
        let decrypted = decrypt(self.value.clone(), prime_pwd, salt)?;
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], config::standard()).unwrap();

        Ok(DecryptedRecord {
            creds: decoded,
            metadata: self.metadata(),
        })
    }
}

pub fn migrate_encrypted_record_from_v060(er: v060::EncryptedRecord) -> EncryptedRecord {
    EncryptedRecord {
        key: er.key(),
        value: er.value(),
        metadata: migrate_metadata_from_v060(er.metadata()),
    }
}

#[cfg(test)]
mod tests {
    use crate::store::testing_data;
    use crate::time;

    #[test]
    fn password_records() {
        let pwd = testing_data::store_pwd();
        let salt = time::now();
        let dpr = testing_data::plaintext_record();
        assert_eq!(
            format!("{}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(pwd.clone(), salt.clone());
        assert_eq!(40, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
    }
}
