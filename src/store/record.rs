use bincode::{config, Decode, Encode};
use secrecy::Zeroize;
use serde::{Deserialize, Serialize};

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
    pub fn encrypt(&self, prime_pwd: String) -> EncryptedRecord {
        let encoded = bincode::encode_to_vec(&self.value, config::standard()).unwrap();
        let encrypted = encrypt(encoded, prime_pwd, self.metadata.updated.clone());

        EncryptedRecord {
            key: self.key.clone(),
            value: encrypted,
            metadata: self.metadata.clone(),
        }
    }
}

impl EncryptedRecord {
    pub fn decrypt(&self, prime_pwd: String) -> DecryptedRecord {
        let decrypted = decrypt(self.value.clone(), prime_pwd, self.metadata.updated.clone());
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], config::standard()).unwrap();

        DecryptedRecord {
            key: self.key.clone(),
            value: decoded,
            metadata: self.metadata.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::record::{Creds, DecryptedRecord, Kind, Metadata};

    #[test]
    fn password_records() {
        let store_pwd = "abc123".to_string();
        let now = chrono::offset::Local::now().to_rfc3339();
        let dpr = DecryptedRecord {
            key: "a site".to_string(),
            value: Creds {
                user: "alice@site.com".to_string(),
                password: "4 s3kr1t".to_string(),
            },
            metadata: Metadata {
                kind: Kind::Password,
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
        assert_eq!(
            epr.value,
            [
                101, 17, 166, 86, 153, 74, 37, 214, 8, 132, 34, 171, 59, 16, 237, 55, 142, 193, 20,
                37, 129, 209, 236, 51, 60, 51, 201, 60, 241, 65, 83, 170, 12, 100, 228, 187, 254,
                29, 190, 217
            ]
        );
        let re_dpr = epr.decrypt(store_pwd);
        assert_eq!(re_dpr.value.password, "4 s3kr1t");
    }
}
