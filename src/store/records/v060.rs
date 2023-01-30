use anyhow::Result;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::util;

use super::v050;
pub use super::v050::{Creds, DecryptedRecord, EncryptedRecord, Metadata};

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn migrate_hashmap_from_v050(hm_v050: v050::HashMap) -> HashMap {
    let hm: HashMap = dashmap::DashMap::new();
    for i in hm_v050.iter() {
        let r = i.value();
        let _ = hm.insert(
            i.key().to_string(),
            migrate_encrypted_record_from_v050(r.clone()),
        );
    }
    hm
}

pub fn decode_hashmap(bytes: Vec<u8>) -> Result<HashMap> {
    let hm: HashMap = dashmap::DashMap::new();
    let sorted_vec: Vec<(String, EncryptedRecord)>;
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
            log::info!("Attempting to decode hashmap from previous version (0.5.0)");
            let hm = v050::decode_hashmap(bytes)?;
            Ok(migrate_hashmap_from_v050(hm))
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq, Encode, Decode)]
pub enum Kind {
    #[default]
    Account,
    Credential,
    Password,
}

pub const DEFAULT_KIND: Kind = Kind::Password;

pub fn migrate_kind_from_v050(k: v050::Kind) -> Kind {
    match k {
        v050::Kind::Password => Kind::Password,
    }
}

pub fn migrate_encrypted_record_from_v050(er: v050::EncryptedRecord) -> EncryptedRecord {
    EncryptedRecord {
        key: er.key(),
        value: er.value(),
        metadata: migrate_metadata_from_v050(er.metadata()),
    }
}

pub fn migrate_metadata_from_v050(md: v050::Metadata) -> Metadata {
    Metadata {
        kind: md.kind,
        url: md.url,
        created: md.created,
        imported: md.imported,
        updated: md.updated,
        password_changed: md.password_changed,
        last_used: md.last_used,
        access_count: md.access_count,
    }
}
