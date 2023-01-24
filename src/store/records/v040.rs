use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub use super::v030::{Creds, Kind, Metadata};
use crate::store::crypto::{decrypt, encrypt};
use crate::util;

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn decode_hashmap(bytes: Vec<u8>) -> Result<HashMap> {
    let hashmap: HashMap;
    match bincode::serde::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
        Ok((result, _len)) => {
            hashmap = result;
            Ok(hashmap)
        }
        Err(e) => {
            let msg = format!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
            log::error!("{}", msg);
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
