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
        self.key.clone()
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

impl EncryptedRecord {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn decrypt(&self, prime_pwd: String) -> DecryptedRecord {
        let decrypted = decrypt(self.value.clone(), prime_pwd, self.metadata().updated);
        let (decoded, _len) =
            bincode::decode_from_slice(&decrypted[..], config::standard()).unwrap();

        DecryptedRecord {
            key: self.key(),
            creds: decoded,
            metadata: self.metadata(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::testing_data;

    #[test]
    fn password_records() {
        let pwd = testing_data::store_pwd();
        let dpr = testing_data::plaintext_record();
        assert_eq!(
            format!("{}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.creds),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(pwd.clone());
        assert_eq!(40, epr.value.len());
        let re_dpr = epr.decrypt(pwd);
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
    }
}
