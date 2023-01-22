use anyhow::Result;
use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::store::crypto::{decrypt, encrypt};

use super::v020;
use super::v020::{Creds, Kind};

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

    pub fn encrypt(&self, prime_pwd: String) -> EncryptedRecord {
        let encoded = bincode::encode_to_vec(&self.creds, config::standard()).unwrap();
        let encrypted = encrypt(encoded, prime_pwd, self.metadata().updated);

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

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn decrypt(&self, prime_pwd: String) -> Result<DecryptedRecord> {
        let decrypted = decrypt(self.value.clone(), prime_pwd, self.metadata().updated)?;
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], config::standard()).unwrap();

        Ok(DecryptedRecord {
            creds: decoded,
            metadata: self.metadata(),
        })
    }
}
