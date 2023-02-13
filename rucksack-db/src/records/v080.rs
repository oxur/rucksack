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
    log::debug!(
        "Decoding hashmap from stored bytes (format version {:})...",
        version
    );
    version = shared::trim_version(version);
    let hm: HashMap = dashmap::DashMap::new();
    log::trace!("Created hashmap.");
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    log::trace!("Created vec for sorted data.");
    if version < shared::version(VERSION) {
        // version.
        log::info!("Attempting to decode hashmap from previous version (0.7.0)");
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
            log::info!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
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
        let epr = dpr.encrypt(pwd.clone(), salt.clone());
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
}
