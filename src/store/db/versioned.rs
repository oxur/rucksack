use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct VersionedDB {
    bytes: Vec<u8>,
    version: String,
}

pub fn from_encoded(bytes: Vec<u8>) -> Result<VersionedDB> {
    let versioned: VersionedDB;
    match bincode::serde::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
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
        match bincode::serde::encode_to_vec(self, util::bincode_cfg()) {
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

    pub fn version(&self) -> versions::Versioning {
        versions::Versioning::new(self.version.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::store::db::versioned;

    #[test]
    fn db_bytes() {
        let tmp_db = versioned::new(vec![2, 4, 16], "1.2.3".to_string());
        assert!(tmp_db.version() > versions::Versioning::new("0.3.0").unwrap());
        assert_eq!(
            tmp_db.version(),
            versions::Versioning::new("1.2.3").unwrap()
        );
        assert!(tmp_db.version() < versions::Versioning::new("2.3.0").unwrap());
        let encoded = tmp_db.serialise().unwrap();
        let expected = vec![
            3, 0, 0, 0, 0, 0, 0, 0, 2, 4, 16, 5, 0, 0, 0, 0, 0, 0, 0, 49, 46, 50, 46, 51,
        ];
        assert_eq!(encoded, expected);
    }
}
