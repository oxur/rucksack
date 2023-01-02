use std::fs;
use std::io::Error;

use bincode::config;
use dashmap::DashMap;

use super::crypto::{decrypt, encrypt};
use super::record::{DecryptedRecord, EncryptedRecord};

pub struct DB {
    path: String,
    store_pwd: String,
    salt: String,
    bincode_cfg: bincode::config::Configuration,
    hash_map: DashMap<String, EncryptedRecord>,
}

pub fn init(path: String, store_pwd: String, updated: String) -> Result<(), Error> {
    let db = open(path, store_pwd, updated);
    db.close()
}

pub fn open(path: String, store_pwd: String, salt: String) -> DB {
    let mut hash_map: DashMap<String, EncryptedRecord> = DashMap::new();
    let bincode_cfg = config::standard();
    if std::path::Path::new(&path).exists() {
        let mut _len = 0;
        let encrypted = fs::read(path.clone()).unwrap();
        let decrypted = decrypt(encrypted, store_pwd.clone(), salt.clone());
        (hash_map, _len) =
            bincode::serde::decode_from_slice(decrypted.as_ref(), bincode_cfg).unwrap();
    }
    DB {
        path,
        store_pwd,
        salt,
        bincode_cfg,
        hash_map,
    }
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

    pub fn close(&self) -> Result<(), Error> {
        let encoded = bincode::serde::encode_to_vec(self.hash_map(), self.bincode_cfg).unwrap();
        let encrypted = encrypt(encoded, self.store_pwd(), self.salt());
        fs::write(self.path(), encrypted)
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        self.hash_map
            .insert(record.key(), record.encrypt(self.store_pwd()))
    }

    pub fn get(&self, key: String) -> Option<DecryptedRecord> {
        self.hash_map
            .get(&key)
            .map(|encrypted| encrypted.decrypt(self.store_pwd()))
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
        let path = format!("{:?}", NamedTempFile::new().unwrap());
        let tmp_db = db::open(path, pwd, date_time);

        let dpr = testing_data::plaintext_record();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.creds.user, "alice@site.com");
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
    }
}
