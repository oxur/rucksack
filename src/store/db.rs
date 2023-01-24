use std::fs;

use anyhow::{anyhow, Error, Result};
use dashmap::DashMap;

use crate::{time, util};

use super::crypto::{decrypt, encrypt};
use super::record::{DecryptedRecord, EncryptedRecord};

#[derive(Clone, Default)]
pub struct DB {
    path: String,
    store_hash: u32,
    store_pwd: String,
    salt: String,
    bincode_cfg: util::BincodeConfig,
    hash_map: DashMap<String, EncryptedRecord>,
}

pub fn init(path: String, store_pwd: String, updated: String) -> Result<()> {
    let db = open(path, store_pwd, updated).unwrap();
    db.close()
}

pub fn open(path: String, store_pwd: String, salt: String) -> Result<DB> {
    let mut hash_map: DashMap<String, EncryptedRecord> = DashMap::new();
    let mut store_hash = 0;
    let bincode_cfg = util::bincode_cfg();
    if std::path::Path::new(&path).exists() {
        let mut _len = 0;
        let encrypted = util::read_file(path.clone())?;
        let decrypted = decrypt(encrypted, store_pwd.clone(), salt.clone())?;
        store_hash = crc32fast::hash(decrypted.as_ref());
        hash_map = decode_hashmap(decrypted)?;
    }
    Ok(DB {
        path,
        store_hash,
        store_pwd,
        salt,
        bincode_cfg,
        hash_map,
    })
}

impl DB {
    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn store_pwd(&self) -> String {
        self.store_pwd.clone()
    }

    pub fn salt(&self) -> String {
        self.salt.clone()
    }

    pub fn hash_map(&self) -> DashMap<String, EncryptedRecord> {
        self.hash_map.clone()
    }

    pub fn encode_hashmap(&self) -> Result<Vec<u8>> {
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in self.iter() {
            data.push((i.key().clone(), i.value().clone()))
        }
        data.sort_by_key(|k| k.0.clone());
        match bincode::serde::encode_to_vec(data, self.bincode_cfg) {
            Ok(encoded) => Ok(encoded),
            Err(e) => Err(anyhow!("Couldn't encode hashmap ({:})", e)),
        }
    }

    pub fn close(&self) -> Result<()> {
        let path_name = &self.path();
        let path = std::path::Path::new(path_name);
        let encoded = self.encode_hashmap()?;
        let store_hash = crc32fast::hash(encoded.as_ref());
        if store_hash == self.store_hash {
            return Ok(());
        }
        fs::create_dir_all(path.parent().unwrap())?;
        if std::path::Path::new(&self.path).exists() {
            let backup_name = format!("{}-{}", self.path, time::simple_timestamp());
            std::fs::copy(path, backup_name)?;
        }
        let encrypted = encrypt(encoded, self.store_pwd(), self.salt());
        util::write_file(encrypted, self.path())
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        self.hash_map
            .insert(record.key(), record.encrypt(self.store_pwd(), self.salt()))
    }

    pub fn get(&self, key: String) -> Option<DecryptedRecord> {
        self.hash_map
            .get(&key)
            .map(|encrypted| encrypted.decrypt(self.store_pwd(), self.salt()).unwrap())
    }

    pub fn iter(&self) -> dashmap::iter::Iter<String, EncryptedRecord> {
        self.hash_map.iter()
    }

    pub fn collect_decrypted(&self) -> Result<Vec<DecryptedRecord>, Error> {
        let mut decrypted: Vec<DecryptedRecord> = Vec::new();
        for i in self.iter() {
            let record = i.value().decrypt(self.store_pwd(), self.salt())?;
            decrypted.push(record);
        }
        Ok(decrypted)
    }
}

// Support functions

// Note that this operation is the inverse to DB.encode_hashmap.
fn decode_hashmap(decrypted: Vec<u8>) -> Result<DashMap<String, EncryptedRecord>> {
    let hash_map: DashMap<String, EncryptedRecord> = DashMap::new();
    let (sorted_vec, _): (Vec<(String, EncryptedRecord)>, usize) =
        bincode::serde::decode_from_slice(decrypted.as_ref(), util::bincode_cfg())?;
    for (key, val) in sorted_vec {
        if hash_map.insert(key.clone(), val).is_some() {}
    }
    Ok(hash_map)
}

#[cfg(test)]
mod tests {
    use crate::store::db;
    use crate::store::testing_data;
    use crate::time;
    use tempfile::NamedTempFile;

    #[test]
    fn db_basics() {
        let pwd = testing_data::store_pwd();
        let salt = time::now();
        let path = NamedTempFile::new()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_string();
        let tmp_db = db::open(path.clone(), pwd.clone(), salt.clone()).unwrap();
        let dpr = testing_data::plaintext_record();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.creds.user, "alice@site.com");
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
        tmp_db.close().unwrap();
        let tmp_db = db::open(path, pwd, salt).unwrap();
        let read_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(read_dpr.creds.user, "alice@site.com");
        assert_eq!(read_dpr.creds.password, "4 s3kr1t");
    }
}
