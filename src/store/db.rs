use std::fs;
use std::io::Write;

use anyhow::Result;
use bincode::config;
use dashmap::DashMap;

use super::crypto::{decrypt, encrypt};
use super::record::{DecryptedRecord, EncryptedRecord};

#[derive(Clone, Default)]
pub struct DB {
    path: String,
    store_pwd: String,
    salt: String,
    bincode_cfg: bincode::config::Configuration,
    hash_map: DashMap<String, EncryptedRecord>,
}

pub fn init(path: String, store_pwd: String, updated: String) -> Result<()> {
    let db = open(path, store_pwd, updated).unwrap();
    db.close()
}

pub fn open(path: String, store_pwd: String, salt: String) -> Result<DB> {
    let mut hash_map: DashMap<String, EncryptedRecord> = DashMap::new();
    let bincode_cfg = config::standard();
    if std::path::Path::new(&path).exists() {
        let mut _len = 0;
        let encrypted = fs::read(path.clone()).unwrap();
        let decrypted = decrypt(encrypted, store_pwd.clone(), salt.clone())?;
        (hash_map, _len) = bincode::serde::decode_from_slice(decrypted.as_ref(), bincode_cfg)?;
        let backup_name = format!(
            "{}-{}",
            path,
            chrono::offset::Local::now().format("%Y%m%d-%H%M%S")
        );
        std::fs::rename(&path, backup_name)?;
    }
    Ok(DB {
        path,
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

    pub fn close(&self) -> Result<()> {
        let path_name = &self.path();
        let path = std::path::Path::new(path_name);
        let encoded = bincode::serde::encode_to_vec(self.hash_map(), self.bincode_cfg).unwrap();
        let encrypted = encrypt(encoded, self.store_pwd(), self.salt());
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.path())
            .unwrap();
        file.write_all(&encrypted[..])?;
        file.sync_all()?;
        Ok(())
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        self.hash_map
            .insert(record.key(), record.encrypt(self.store_pwd()))
    }

    pub fn get(&self, key: String) -> Option<DecryptedRecord> {
        self.hash_map
            .get(&key)
            .map(|encrypted| encrypted.decrypt(self.store_pwd()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::db;
    use crate::store::testing_data;
    use tempfile::NamedTempFile;

    #[test]
    fn db_basics() {
        let pwd = testing_data::store_pwd();
        let date_time = testing_data::now();
        let path = NamedTempFile::new()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_string();
        let tmp_db = db::open(path.clone(), pwd.clone(), date_time.clone()).unwrap();
        let dpr = testing_data::plaintext_record();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.creds.user, "alice@site.com");
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
        tmp_db.close().unwrap();
        let tmp_db = db::open(path, pwd, date_time).unwrap();
        let read_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(read_dpr.creds.user, "alice@site.com");
        assert_eq!(read_dpr.creds.password, "4 s3kr1t");
    }
}
