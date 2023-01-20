use anyhow::{anyhow, Result};
use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct VersionedDB {
    bytes: Vec<u8>,
    version: String,
}

pub fn from_encoded(bytes: Vec<u8>) -> Result<VersionedDB> {
    let versioned: VersionedDB;
    match bincode::serde::decode_from_slice(bytes.as_ref(), config::standard()) {
        Ok((result, _len)) => {
            versioned = result;
            log::debug!("deserialised versioned DB bytes: {:?}", versioned);
            Ok(versioned)
        }
        Err(e) => {
            let msg = format!("couldn't deserialise versioned database file: {:?}", e);
            log::error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

pub fn from_bytes(bytes: Vec<u8>) -> VersionedDB {
    new(bytes, env!("CARGO_PKG_VERSION").to_string())
}

pub fn new(bytes: Vec<u8>, version: String) -> VersionedDB {
    VersionedDB { bytes, version }
}

impl VersionedDB {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn serialise(&self) -> Result<Vec<u8>> {
        match bincode::serde::encode_to_vec(self, config::standard()) {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                let msg = format!("couldn't serialise versioned database ({})", e);
                log::error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn hash(&self) -> u32 {
        crc32fast::hash(self.bytes.as_ref())
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }
}

#[cfg(test)]
mod tests {
    use bincode::config;

    use crate::store::db::versioned::VersionedDB;

    #[test]
    fn db_bytes() {
        let tmp_db = VersionedDB {
            version: "1.2.3".to_string(),
            bytes: vec![1, 2, 3],
        };
        let encoded = bincode::serde::encode_to_vec(tmp_db, config::standard()).unwrap();
        assert_eq!(encoded, vec![5, 49, 46, 50, 46, 51, 3, 1, 2, 3]);
    }
}
