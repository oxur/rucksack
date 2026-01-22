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
    pub fn new(bytes: Vec<u8>, version: String) -> Result<VersionedDB> {
        // Validate version format immediately
        versions::SemVer::new(&version)
            .ok_or_else(|| anyhow!("invalid version format '{}'", version))?;

        Ok(VersionedDB { bytes, version })
    }

    pub fn deserialise(bytes: Vec<u8>) -> Result<VersionedDB> {
        log::debug!(operation = "deserialise"; "Creating versioned DB from previously serialised versioned DB");
        match bincode::decode_from_slice::<VersionedDB, _>(bytes.as_ref(), util::bincode_cfg()) {
            Ok((result, _len)) => {
                log::trace!(version = result.version.as_str(), operation = "deserialise"; "Deserialised versioned DB bytes");
                Ok(result)
            }
            Err(e) => {
                let msg = format!("couldn't deserialise versioned database file: {e:?}");
                log::error!(error = e.to_string().as_str(), operation = "deserialise"; "{}", msg);
                Err(anyhow!(msg))
            }
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<VersionedDB> {
        log::debug!(operation = "init"; "Initialising versioned DB with encoded hashmap");
        VersionedDB::new(bytes, records::version().to_string())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn hash(&self) -> u32 {
        crc32fast::hash(self.bytes.as_ref())
    }

    pub fn serialise(&self) -> Result<Vec<u8>> {
        log::debug!(operation = "serialise"; "Serialising versioned DB");
        match bincode::encode_to_vec(self, util::bincode_cfg()) {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                let msg = format!("couldn't serialise versioned database ({e})");
                log::error!(error = e.to_string().as_str(), operation = "serialise"; "{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn version(&self) -> versions::SemVer {
        // SAFETY: Version string is validated in constructor
        versions::SemVer::new(self.version.as_str()).expect("version validated in constructor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_basic() {
        let vsn_db = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        assert_eq!(vsn_db.bytes(), vec![1, 2, 3]);
        assert_eq!(vsn_db.version(), versions::SemVer::new("1.0.0").unwrap());
    }

    #[test]
    fn test_new_empty_bytes() {
        let vsn_db = VersionedDB::new(vec![], "0.1.0".to_string()).unwrap();
        assert_eq!(vsn_db.bytes(), vec![]);
        assert_eq!(vsn_db.version(), versions::SemVer::new("0.1.0").unwrap());
    }

    #[test]
    fn test_new_large_bytes() {
        let large_vec = vec![42u8; 10000];
        let vsn_db = VersionedDB::new(large_vec.clone(), "2.0.0".to_string()).unwrap();
        assert_eq!(vsn_db.bytes(), large_vec);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![5, 10, 15];
        let vsn_db = VersionedDB::from_bytes(bytes.clone()).unwrap();
        assert_eq!(vsn_db.bytes(), bytes);
        // Should use current records version - just verify it parses
        let _version = vsn_db.version();
    }

    #[test]
    fn test_bytes_getter() {
        let original = vec![1, 2, 3, 4, 5];
        let vsn_db = VersionedDB::new(original.clone(), "1.0.0".to_string()).unwrap();
        let retrieved = vsn_db.bytes();
        assert_eq!(retrieved, original);
    }

    #[test]
    fn test_hash_consistent() {
        let vsn_db = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let hash1 = vsn_db.hash();
        let hash2 = vsn_db.hash();
        assert_eq!(hash1, hash2, "Hash should be consistent");
    }

    #[test]
    fn test_hash_different_for_different_bytes() {
        let vsn_db1 = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let vsn_db2 = VersionedDB::new(vec![4, 5, 6], "1.0.0".to_string()).unwrap();
        assert_ne!(vsn_db1.hash(), vsn_db2.hash());
    }

    #[test]
    fn test_hash_empty_bytes() {
        let vsn_db = VersionedDB::new(vec![], "1.0.0".to_string()).unwrap();
        let hash = vsn_db.hash();
        assert_eq!(hash, 0); // CRC32 of empty data is 0
    }

    #[test]
    fn test_hash_known_value() {
        let vsn_db = VersionedDB::new(vec![2, 4, 16], "1.2.3".to_string()).unwrap();
        assert_eq!(vsn_db.hash(), 2233391132);
    }

    #[test]
    fn test_version_parsing() {
        let vsn_db = VersionedDB::new(vec![1], "3.14.159".to_string()).unwrap();
        let version = vsn_db.version();
        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 14);
        assert_eq!(version.patch, 159);
    }

    #[test]
    fn test_version_with_prerelease() {
        let vsn_db = VersionedDB::new(vec![1], "2.0.0-alpha.1".to_string()).unwrap();
        let version = vsn_db.version();
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn test_version_comparisons() {
        let vsn_db = VersionedDB::new(vec![1, 2], "1.5.0".to_string()).unwrap();
        assert!(vsn_db.version() > versions::SemVer::new("1.4.9").unwrap());
        assert!(vsn_db.version() < versions::SemVer::new("1.5.1").unwrap());
        assert_eq!(vsn_db.version(), versions::SemVer::new("1.5.0").unwrap());
    }

    #[test]
    fn test_serialise_basic() {
        let vsn_db = VersionedDB::new(vec![10, 20], "1.0.0".to_string()).unwrap();
        let result = vsn_db.serialise();
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_serialise_empty() {
        let vsn_db = VersionedDB::new(vec![], "0.0.1".to_string()).unwrap();
        let result = vsn_db.serialise();
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialise_basic() {
        let original = VersionedDB::new(vec![1, 2, 3], "1.2.3".to_string()).unwrap();
        let serialised = original.serialise().unwrap();
        let deserialised = VersionedDB::deserialise(serialised).unwrap();
        assert_eq!(deserialised.bytes(), original.bytes());
        assert_eq!(deserialised.version(), original.version());
    }

    #[test]
    fn test_deserialise_invalid_bytes() {
        let invalid_bytes = vec![255, 255, 255, 1, 2, 3];
        let result = VersionedDB::deserialise(invalid_bytes);
        assert!(result.is_err(), "Should fail to deserialise invalid bytes");
    }

    #[test]
    fn test_deserialise_empty_bytes() {
        let result = VersionedDB::deserialise(vec![]);
        assert!(result.is_err(), "Should fail to deserialise empty bytes");
    }

    #[test]
    fn test_deserialise_truncated_bytes() {
        let original = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let serialised = original.serialise().unwrap();
        // Truncate to cause a deserialization error
        let truncated = &serialised[..serialised.len() / 2];
        let result = VersionedDB::deserialise(truncated.to_vec());
        assert!(result.is_err(), "Truncated data should fail to deserialise");
        if let Err(e) = result {
            assert!(e.to_string().contains("deserialise"));
        }
    }

    #[test]
    fn test_serialise_deserialise_roundtrip() {
        let original = VersionedDB::new(vec![5, 10, 15, 20], "2.1.0".to_string()).unwrap();
        let serialised = original.serialise().unwrap();
        let deserialised = VersionedDB::deserialise(serialised).unwrap();

        assert_eq!(original.bytes(), deserialised.bytes());
        assert_eq!(original.version(), deserialised.version());
        assert_eq!(original.hash(), deserialised.hash());
    }

    #[test]
    fn test_serialise_deserialise_large_data() {
        let large_data = vec![123u8; 5000];
        let original = VersionedDB::new(large_data, "3.0.0".to_string()).unwrap();
        let serialised = original.serialise().unwrap();
        let deserialised = VersionedDB::deserialise(serialised).unwrap();

        assert_eq!(original.bytes(), deserialised.bytes());
        assert_eq!(original.hash(), deserialised.hash());
    }

    #[test]
    fn test_clone() {
        let original = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let cloned = original.clone();
        assert_eq!(original.bytes(), cloned.bytes());
        assert_eq!(original.version(), cloned.version());
        assert_eq!(original.hash(), cloned.hash());
    }

    #[test]
    fn test_equality() {
        let db1 = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let db2 = VersionedDB::new(vec![1, 2, 3], "1.0.0".to_string()).unwrap();
        let db3 = VersionedDB::new(vec![1, 2, 3], "1.0.1".to_string()).unwrap();
        let db4 = VersionedDB::new(vec![1, 2, 4], "1.0.0".to_string()).unwrap();

        assert_eq!(db1, db2);
        assert_ne!(db1, db3); // Different version
        assert_ne!(db1, db4); // Different bytes
    }

    #[test]
    fn db_bytes() {
        let vsn_db = VersionedDB::new(vec![2, 4, 16], "1.2.3".to_string()).unwrap();
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
