use anyhow::{anyhow, Result};
use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};

use super::v050;
pub use super::v050::{Creds, DecryptedRecord, EncryptedRecord, Metadata};

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

pub fn decode_hashmap(bytes: Vec<u8>) -> Result<HashMap> {
    let hashmap: HashMap;
    match bincode::serde::decode_from_slice(bytes.as_ref(), config::standard()) {
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
