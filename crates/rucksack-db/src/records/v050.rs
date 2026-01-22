use anyhow::{anyhow, Result};

use rucksack_lib::util;

use super::shared;
pub use super::v040::{Creds, DecryptedRecord, EncryptedRecord, Kind, Metadata};

pub const VERSION: &str = "0.5.0";

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn decode_hashmap(bytes: Vec<u8>, mut version: versions::SemVer) -> Result<HashMap> {
    log::debug!(version = version.to_string().as_str(), operation = "decode"; "Decoding hashmap from stored bytes");
    version = shared::trim_version(version);
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!(operation = "decode"; "Created hashmap");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!(operation = "decode"; "Created vec for sorted data");
    let current_version = shared::version(VERSION).map_err(|e| anyhow!("{}", e))?;
    if version < current_version {
        let msg = format!("automatic migration not supported for versions prior to: {VERSION}");
        log::error!(version = VERSION, operation = "decode"; "{}", msg);
        return Err(anyhow!(msg));
    }
    match bincode::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
        Ok((result, _len)) => {
            sorted_vec = result;
            for (key, val) in sorted_vec {
                if hm.insert(key.clone(), val).is_some() {}
            }
            Ok(hm)
        }
        Err(e) => {
            let msg = format!("couldn't deserialise bincoded hashmap bytes: {e:?}");
            log::error!(error = e.to_string().as_str(), operation = "decode"; "{}", msg);
            Err(anyhow!(msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rucksack_lib::time;

    fn test_metadata() -> Metadata {
        Metadata {
            kind: Kind::Password,
            url: "https://example.com".to_string(),
            created: time::now(),
            imported: time::epoch_zero(),
            updated: time::now(),
            password_changed: time::epoch_zero(),
            last_used: time::epoch_zero(),
            access_count: 0,
        }
    }

    fn test_decrypted_record() -> DecryptedRecord {
        DecryptedRecord {
            creds: Creds {
                user: "testuser".to_string(),
                password: "testpass".to_string(),
            },
            metadata: test_metadata(),
        }
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, "0.5.0");
    }

    #[test]
    fn test_decode_hashmap() {
        let hm: HashMap = dashmap::DashMap::new();
        let record = test_decrypted_record();
        let salt = time::now();
        let encrypted = record.encrypt("password", &salt).unwrap();
        hm.insert("test_key".to_string(), encrypted);

        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in hm.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = versions::SemVer::new(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_empty() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = versions::SemVer::new(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 0);
    }

    #[test]
    fn test_decode_hashmap_error() {
        let invalid_bytes = vec![1, 2, 3, 4, 5];
        let version = versions::SemVer::new(VERSION).unwrap();
        let result = decode_hashmap(invalid_bytes, version);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_hashmap_old_version() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let old_version = versions::SemVer::new("0.4.0").unwrap();
        let result = decode_hashmap(bytes, old_version);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("migration not supported"));
    }
}
