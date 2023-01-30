use anyhow::{anyhow, Result};

use crate::util;

pub use super::v040::{Creds, DecryptedRecord, EncryptedRecord, Kind, Metadata};

pub type HashMap = dashmap::DashMap<String, EncryptedRecord>;

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
            let msg = format!("couldn't deserialise bincoded hashmap bytes: {:?}", e);
            log::error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}
