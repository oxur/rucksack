use anyhow::Result;
use bincode::{Decode, Encode};
use secrecy::Zeroize;
use serde::{Deserialize, Serialize};

use crate::util;

use super::crypto::{decrypt, encrypt};

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
    pub imported: String,
    pub updated: String,
    pub password_changed: String,
    pub last_used: String,
    pub access_count: u64,
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
    pub creds: Creds,
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
        let encoded = bincode::encode_to_vec(&self.creds, util::bincode_cfg()).unwrap();
        let encrypted = encrypt(encoded, prime_pwd, salt);

        EncryptedRecord {
            key: self.key(),
            value: encrypted,
            metadata: self.metadata(),
        }
    }
}

impl EncryptedRecord {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn decrypt(&self, prime_pwd: String, salt: String) -> Result<DecryptedRecord> {
        let decrypted = decrypt(self.value.clone(), prime_pwd, salt)?;
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
        assert_eq!(54, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
    }
}
