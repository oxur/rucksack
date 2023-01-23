use anyhow::{anyhow, Result};
use bincode::config;

pub use super::v040::{Creds, DecryptedRecord, EncryptedRecord, Kind, Metadata};

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
