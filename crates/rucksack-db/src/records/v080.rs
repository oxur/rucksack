use anyhow::{anyhow, Result};

use rucksack_lib::{time, util};

use super::shared;
pub use super::v070;
pub use super::v070::{
    name_from_key, types, DecryptedRecord, EncryptedRecord, HashMap, Kind, Metadata, Secrets,
    Status, Tag, ANY_CATEGORY, DEFAULT_CATEGORY,
};

pub const VERSION: &str = "0.8.0";

// Hashmap - the primary store data structure

pub fn migrate_hashmap_from_v070(hm_v070: v070::HashMap) -> HashMap {
    let hm: HashMap = dashmap::DashMap::new();
    for i in hm_v070.iter() {
        let r = i.value();
        let _ = hm.insert(
            i.key().to_string(),
            migrate_encrypted_record_from_v070(r.clone()),
        );
    }
    hm
}

pub fn decode_hashmap(bytes: Vec<u8>, mut version: versions::SemVer) -> Result<HashMap> {
    log::debug!(version = version.to_string().as_str(), operation = "decode"; "Decoding hashmap from stored bytes");
    version = shared::trim_version(version);
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!(operation = "decode"; "Created hashmap");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!(operation = "decode"; "Created vec for sorted data");
    let current_version = shared::version(VERSION).map_err(|e| anyhow!("{}", e))?;
    if version < current_version {
        // version.
        log::info!(version = "0.7.0", operation = "migrate"; "Attempting to decode hashmap from previous version");
        let hm = v070::decode_hashmap(bytes, version)?;
        return Ok(migrate_hashmap_from_v070(hm));
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
            log::info!(error = e.to_string().as_str(), operation = "decode"; "couldn't deserialise bincoded hashmap bytes");
            Err(anyhow!(e))
        }
    }
}

// Secret data

pub fn default_secrets() -> Secrets {
    Secrets {
        ..Default::default()
    }
}

pub fn secrets_from_user_pass(user: &str, pwd: &str) -> Secrets {
    Secrets {
        user: user.to_string(),
        password: pwd.to_string(),
        ..Default::default()
    }
}

pub fn migrate_secrets_from_v070(s070: v070::Secrets) -> Secrets {
    Secrets {
        account_id: s070.account_id,
        user: s070.user,
        password: s070.password,
        public_key: s070.public_key,
        private_key: s070.private_key,
        public_cert: s070.public_cert,
        private_cert: s070.private_cert,
        root_cert: s070.root_cert,
        key: s070.key,
        secret: s070.secret,
    }
}

// Metadata

pub fn new_tag(value: String) -> Tag {
    Tag {
        value,
        created: time::now(),
        updated: time::epoch_zero(),

        ..Default::default()
    }
}

pub fn new_tags(values: Vec<String>) -> Vec<Tag> {
    values.into_iter().map(new_tag).collect()
}

pub fn default_metadata() -> Metadata {
    let now = time::now();
    let time_zero = time::epoch_zero();
    let mut md = Metadata {
        ..Default::default()
    };
    md.state = Status::default();
    md.kind = Kind::default();
    md.category = DEFAULT_CATEGORY.to_string();
    md.created = now.clone();
    md.updated = now;
    md.imported = time_zero.clone();
    md.password_changed = time_zero.clone();
    md.last_used = time_zero.clone();
    md.synced = time_zero;
    md
}

pub fn migrate_metadata_from_v070(md070: v070::Metadata, name: String) -> Metadata {
    Metadata {
        kind: md070.kind,
        category: md070.category,
        name,
        url: md070.url,
        created: md070.created,
        imported: md070.imported,
        updated: md070.updated,
        password_changed: md070.password_changed,
        last_used: md070.last_used,
        synced: md070.synced,
        access_count: md070.access_count,
        state: md070.state,
        tags: md070.tags,
    }
}

// Decrypted records

pub fn migrate_decrypted_record_from_v070(dr: v070::DecryptedRecord) -> DecryptedRecord {
    DecryptedRecord {
        secrets: migrate_secrets_from_v070(dr.secrets.clone()),
        metadata: migrate_metadata_from_v070(dr.metadata.clone(), name_from_key(dr.key())),
    }
}

// Encrypted records

pub fn migrate_encrypted_record_from_v070(er: v070::EncryptedRecord) -> EncryptedRecord {
    let key = er.key();
    EncryptedRecord {
        key: key.clone(),
        value: er.value(),
        metadata: migrate_metadata_from_v070(er.metadata(), name_from_key(key)),
    }
}

// Utility functions

pub fn key(category: &str, kind: Kind, name: &str, url: &str) -> String {
    format!("{category}:{kind:?}:{name}:{url}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;
    use rucksack_lib::time;

    #[test]
    fn password_records() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let dpr = testing::data::plaintext_record_v080();
        assert_eq!(
            format!("{}", dpr.secrets),
            "Creds{user: alice@site.com, password: *****}"
        );
        assert_eq!(
            format!("{:?}", dpr.secrets),
            "Creds{user: alice@site.com, password: *****}"
        );
        let epr = dpr.encrypt(pwd.clone(), salt.clone()).unwrap();
        assert_eq!(118, epr.value.len());
        let re_dpr = epr.decrypt(pwd, salt).unwrap();
        assert_eq!(re_dpr.secrets.password, "4 s3kr1t");
    }

    #[test]
    fn tags() {
        let mut dpr = testing::data::plaintext_record_v080();
        assert_eq!(dpr.metadata().tags, vec![]);
        let tag1 = "good stuff".to_string();
        dpr.add_tag(tag1.clone());
        assert_eq!(dpr.metadata().tags.len(), 1);
        assert_eq!(dpr.metadata().tags[0].value, tag1);
        let tag2 = "only the best".to_string();
        let tag3 = "bonus".to_string();
        dpr.add_tags(vec![tag2.clone(), tag3.clone()]);
        assert_eq!(dpr.metadata().tags.len(), 3);
        assert_eq!(dpr.metadata().tag_values(), vec![tag3, tag1, tag2]);
    }

    #[test]
    fn test_default_secrets() {
        let secrets = default_secrets();
        assert_eq!(secrets.user, "");
        assert_eq!(secrets.password, "");
        assert_eq!(secrets.account_id, "");
    }

    #[test]
    fn test_secrets_from_user_pass() {
        let secrets = secrets_from_user_pass("testuser", "testpass");
        assert_eq!(secrets.user, "testuser");
        assert_eq!(secrets.password, "testpass");
    }

    #[test]
    fn test_new_tag() {
        let tag = new_tag("test_tag".to_string());
        assert_eq!(tag.value, "test_tag");
        assert_ne!(tag.created, time::epoch_zero());
        assert_eq!(tag.updated, time::epoch_zero());
    }

    #[test]
    fn test_new_tags() {
        let values = vec!["tag1".to_string(), "tag2".to_string()];
        let tags = new_tags(values);
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].value, "tag1");
        assert_eq!(tags[1].value, "tag2");
    }

    #[test]
    fn test_new_tags_empty() {
        let tags = new_tags(vec![]);
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_default_metadata() {
        let metadata = default_metadata();
        assert_eq!(metadata.name, "");
        assert_eq!(metadata.category, DEFAULT_CATEGORY);
        assert_eq!(metadata.state, Status::default());
        assert_eq!(metadata.kind, Kind::default());
        assert_ne!(metadata.created, time::epoch_zero());
    }

    #[test]
    fn test_key_function() {
        let key = key("test_cat", Kind::Password, "testuser", "test.com");
        assert!(key.contains("test_cat"));
        assert!(key.contains("Password"));
        assert!(key.contains("testuser"));
        assert!(key.contains("test.com"));
    }

    #[test]
    fn test_key_function_empty_fields() {
        let key = key("", Kind::Password, "", "");
        assert!(key.contains("Password"));
    }

    #[test]
    fn test_migrate_secrets_from_v070() {
        let v070_secrets = v070::Secrets {
            account_id: "acc123".to_string(),
            user: "user@test.com".to_string(),
            password: "pass".to_string(),
            public_key: "pubkey".as_bytes().to_vec(),
            private_key: "privkey".as_bytes().to_vec(),
            public_cert: "pubcert".as_bytes().to_vec(),
            private_cert: "privcert".as_bytes().to_vec(),
            root_cert: "rootcert".as_bytes().to_vec(),
            key: "key".to_string(),
            secret: "secret".to_string(),
        };

        let migrated = migrate_secrets_from_v070(v070_secrets.clone());
        assert_eq!(migrated.account_id, v070_secrets.account_id);
        assert_eq!(migrated.user, v070_secrets.user);
        assert_eq!(migrated.password, v070_secrets.password);
    }

    #[test]
    fn test_migrate_metadata_from_v070() {
        let v070_metadata = v070::Metadata {
            kind: Kind::Password,
            category: "test".to_string(),
            name: "".to_string(),
            url: "example.com".to_string(),
            created: time::now(),
            imported: time::epoch_zero(),
            updated: time::now(),
            password_changed: time::epoch_zero(),
            last_used: time::epoch_zero(),
            synced: time::epoch_zero(),
            access_count: 0,
            state: Status::Active,
            tags: vec![],
        };

        let migrated = migrate_metadata_from_v070(v070_metadata.clone(), "NewName".to_string());
        assert_eq!(migrated.name, "NewName");
        assert_eq!(migrated.category, v070_metadata.category);
        assert_eq!(migrated.url, v070_metadata.url);
        assert_eq!(migrated.kind, v070_metadata.kind);
    }

    #[test]
    fn test_decode_hashmap_v080() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let hm: HashMap = dashmap::DashMap::new();

        let record = testing::data::plaintext_record_v080();
        let encrypted = record.encrypt(pwd, salt).unwrap();
        hm.insert("test_key".to_string(), encrypted);

        // Serialize hashmap
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in hm.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        // Decode it
        let version = shared::version(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_empty() {
        let data: Vec<(String, EncryptedRecord)> = Vec::new();
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = shared::version(VERSION).unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 0);
    }

    #[test]
    fn test_migrate_decrypted_record_from_v070() {
        let v070_record = testing::data::plaintext_record_v070();
        let migrated = migrate_decrypted_record_from_v070(v070_record.clone());

        assert_eq!(migrated.secrets.user, v070_record.secrets.user);
        assert_eq!(migrated.secrets.password, v070_record.secrets.password);
    }

    #[test]
    fn test_migrate_encrypted_record_from_v070() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let v070_record = testing::data::plaintext_record_v070();
        let v070_encrypted = v070_record.encrypt(pwd, salt).unwrap();

        let migrated = migrate_encrypted_record_from_v070(v070_encrypted.clone());
        assert_eq!(migrated.key(), v070_encrypted.key());
        assert_eq!(migrated.value(), v070_encrypted.value());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let record = testing::data::plaintext_record_v080();

        let encrypted = record.encrypt(pwd.clone(), salt.clone()).unwrap();
        assert_ne!(encrypted.value, vec![]);

        let decrypted = encrypted.decrypt(pwd, salt).unwrap();
        assert_eq!(decrypted.secrets.user, record.secrets.user);
        assert_eq!(decrypted.secrets.password, record.secrets.password);
    }

    #[test]
    fn test_metadata_tag_operations() {
        let mut record = testing::data::plaintext_record_v080();
        assert_eq!(record.metadata().tags.len(), 0);

        record.add_tag("tag1".to_string());
        assert_eq!(record.metadata().tags.len(), 1);

        record.add_tags(vec!["tag2".to_string(), "tag3".to_string()]);
        assert_eq!(record.metadata().tags.len(), 3);
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, "0.8.0");
        let version = shared::version(VERSION).unwrap();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 8);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn test_migrate_hashmap_from_v070() {
        let hm_v070: v070::HashMap = dashmap::DashMap::new();
        let pwd = testing::data::store_pwd();
        let salt = time::now();

        let record_v070 = testing::data::plaintext_record_v070();
        let encrypted_v070 = record_v070.encrypt(pwd, salt).unwrap();
        hm_v070.insert("test_key".to_string(), encrypted_v070);

        let hm_v080 = migrate_hashmap_from_v070(hm_v070);
        assert_eq!(hm_v080.len(), 1);
        assert!(hm_v080.contains_key("test_key"));
    }

    #[test]
    fn test_decode_hashmap_from_v070() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let hm_v070: v070::HashMap = dashmap::DashMap::new();

        let record = testing::data::plaintext_record_v070();
        let encrypted = record.encrypt(pwd, salt).unwrap();
        hm_v070.insert("v070_key".to_string(), encrypted);

        let mut data: Vec<(String, v070::EncryptedRecord)> = Vec::new();
        for i in hm_v070.iter() {
            data.push((i.key().clone(), i.value().clone()));
        }
        data.sort_by_key(|k| k.0.clone());
        let bytes = bincode::encode_to_vec(data, util::bincode_cfg()).unwrap();

        let version = shared::version("0.7.0").unwrap();
        let decoded_hm = decode_hashmap(bytes, version).unwrap();
        assert_eq!(decoded_hm.len(), 1);
        assert!(decoded_hm.contains_key("v070_key"));
    }

    #[test]
    fn test_decode_hashmap_error() {
        let invalid_bytes = vec![1, 2, 3, 4, 5];
        let version = shared::version(VERSION).unwrap();
        let result = decode_hashmap(invalid_bytes, version);
        assert!(result.is_err());
    }
}
