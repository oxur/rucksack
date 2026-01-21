use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use rucksack_lib::util;

use crate::records;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub struct VersionedDB {
    bytes: Vec<u8>,
    version: String,
}

impl VersionedDB {
    pub fn new(bytes: Vec<u8>, version: String) -> VersionedDB {
        VersionedDB { bytes, version }
    }

    pub fn deserialise(bytes: Vec<u8>) -> Result<VersionedDB> {
        log::debug!("Creating versioned DB from previously serialised versioned DB ...");
        match bincode::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
            Ok((result, _len)) => {
                log::trace!("deserialised versioned DB bytes: {:?}", result);
                Ok(result)
            }
            Err(e) => {
                let msg = format!("couldn't deserialise versioned database file: {e:?}");
                log::error!("{}", msg);
                Err(anyhow!(msg))
            }
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> VersionedDB {
        log::debug!("Initialising versioned DB with encoded hashmap ...");
        VersionedDB::new(bytes, records::version().to_string())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn hash(&self) -> u32 {
        crc32fast::hash(self.bytes.as_ref())
    }

    pub fn serialise(&self) -> Result<Vec<u8>> {
        log::debug!("Serialising versioned DB ...");
        match bincode::encode_to_vec(self, util::bincode_cfg()) {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                let msg = format!("couldn't serialise versioned database ({e})");
                log::error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn version(&self) -> versions::SemVer {
        versions::SemVer::new(self.version.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn db_bytes() {
        let vsn_db = super::VersionedDB::new(vec![2, 4, 16], "1.2.3".to_string());
        assert!(vsn_db.version() > versions::SemVer::new("0.3.0").unwrap());
        assert_eq!(vsn_db.hash(), 2233391132);
        assert_eq!(vsn_db.version(), versions::SemVer::new("1.2.3").unwrap());
        assert!(vsn_db.version() < versions::SemVer::new("2.3.0").unwrap());
        let encoded = vsn_db.serialise().unwrap();
        let expected = vec![
            3, 0, 0, 0, 0, 0, 0, 0, 2, 4, 16, 5, 0, 0, 0, 0, 0, 0, 0, 49, 46, 50, 46, 51,
        ];
        assert_eq!(encoded, expected);
    }
}
