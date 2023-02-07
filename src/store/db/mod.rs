// Database encryption <-> decryption flow:
//
// When a rucksack database is serialised, the following happens:
// * Its hashmap (DashMap) is converted to a sorted vec (for stable serialisation)
// * the sorted vec is bincoded to bytes
// * The bytes are stored on a field of the VersionedDB struct
// * The VersionDB struct is bincoded to bytes
// * The bytes are stored on a field of the EncryptedDB struct
// * The bytes are encrypted
// * The encrypted bytes are saved to a file
//
// Then, in the reverse, when a database is read from disk, this is how it's done:
// * The file is read into memory as bytes and stored on a field of the EncryptedDB struct
// * The encrypted bytes are decrypted
// * The decrypted bytes are then bincode-decoded (deserialised) to a VersionedDB struct
// * The bytes of the VersionDB are bincode-decoded to a vector of (string, record) tuples
// * The sorted vector of tuples is converted to a hashmap (DashMap)
// * The hashmap is stored as a field on the DB struct
//
use std::fmt;

use anyhow::{anyhow, Error, Result};
use dashmap::DashMap;

use crate::store::record;
use crate::store::record::{DecryptedRecord, EncryptedRecord, Metadata};
use crate::time;
use crate::util;

pub mod encrypted;
pub mod old;
pub mod versioned;

#[derive(Clone, Default)]
pub struct DB {
    pub path: String,
    store_hash: u32,
    store_pwd: String,
    salt: String,
    hash_map: record::HashMap,
    enabled: bool,
    version: versions::Versioning,
}

impl fmt::Debug for DB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DB")
            .field("path", &self.path)
            .field("hash_map", &self.hash_map)
            .finish()
    }
}

pub fn init(path: String, store_pwd: String, updated: String) -> Result<()> {
    let db = open(path, store_pwd, updated)?;
    db.close()
}

pub fn new() -> DB {
    DB {
        ..Default::default()
    }
}

pub fn open(path: String, store_pwd: String, salt: String) -> Result<DB> {
    let mut hash_map: record::HashMap = DashMap::new();
    let mut store_hash = 0;
    let mut version = util::version();
    let vsn_db: versioned::VersionedDB;
    if std::path::Path::new(&path).exists() {
        let enc_db = encrypted::from_file(path.clone(), store_pwd.clone(), salt.clone())?;
        match versioned::from_encoded(enc_db.decrypted()) {
            Ok(db) => {
                vsn_db = db;
            }
            Err(_) => {
                vsn_db = versioned::from_bytes(enc_db.decrypted());
            }
        }
        store_hash = vsn_db.hash();
        version = vsn_db.version();
        // Decode the versioned DB's bytes to a hashmap
        hash_map = decode_hashmap(vsn_db.bytes())?;
    };
    Ok(DB {
        path,
        store_hash,
        store_pwd,
        salt,
        hash_map,
        enabled: true,
        version,
    })
}

fn decode_hashmap(bytes: Vec<u8>) -> Result<record::HashMap> {
    let hm: record::HashMap = dashmap::DashMap::new();
    let sorted_vec: Vec<(String, EncryptedRecord)>;
    match bincode::decode_from_slice(bytes.as_ref(), util::bincode_cfg()) {
        Ok((result, _len)) => {
            sorted_vec = result;
            for (key, val) in sorted_vec {
                if hm.insert(key.clone(), val).is_some() {}
            }
            Ok(hm)
        }
        Err(_) => Ok(hm),
    }
}

impl DB {
    pub fn close(&self) -> Result<()> {
        let path = util::create_parents(self.path())?;
        if path.exists() {
            let backup_name = format!("{}-{}", self.path, time::simple_timestamp());
            match std::fs::copy(path.clone(), backup_name) {
                Ok(x) => Ok(x),
                Err(e) => {
                    let msg = "Could not copy file";
                    Err(anyhow!("{} {:?} ({:})", msg, path, e))
                }
            }?;
        }

        // Reverse the workflow of `open` ... encode the hashmap
        let srl = match self.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise self";
                Err(anyhow!("{} {:?} ({:})", msg, self.path(), e))
            }
        }?;
        // Create versioned data
        let vsn_db = versioned::from_bytes(srl);
        let encoded = match vsn_db.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise version db";
                Err(anyhow!("{} {:?} ({:})", msg, self.path(), e))
            }
        }?;
        // Get the hash for the versioned data
        let store_hash = vsn_db.hash();
        if store_hash == self.store_hash {
            return Ok(());
        }
        // Encrypt the versioned data
        let mut enc_db =
            encrypted::from_decrypted(encoded, self.path(), self.store_pwd(), self.salt())?;
        enc_db.encrypt();

        // Save the encrypted data
        enc_db.write()
    }

    pub fn collect_decrypted(&self) -> Result<Vec<DecryptedRecord>, Error> {
        let mut decrypted: Vec<DecryptedRecord> = Vec::new();
        for i in self.iter() {
            let record = i.value().decrypt(self.store_pwd(), self.salt())?;
            decrypted.push(record);
        }
        Ok(decrypted)
    }

    fn serialise(&self) -> Result<Vec<u8>> {
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in self.iter() {
            data.push((i.key().clone(), i.value().clone()))
        }
        data.sort_by_key(|k| k.0.clone());
        match bincode::encode_to_vec(data, util::bincode_cfg()) {
            Ok(encoded) => Ok(encoded),
            Err(e) => {
                let msg = format!("couldn't encode DB hashmap ({e:?})");
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn get(&self, key: String) -> Option<DecryptedRecord> {
        self.hash_map
            .get(&key)
            .map(|encrypted| encrypted.decrypt(self.store_pwd(), self.salt()).unwrap())
    }

    pub fn get_metadata(&self, key: String) -> Option<Metadata> {
        self.get(key).map(|r| r.metadata())
    }

    pub fn hash_map(&self) -> record::HashMap {
        self.hash_map.clone()
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        let key = record.key();
        self.hash_map
            .insert(key, record.encrypt(self.store_pwd(), self.salt()))
    }

    pub fn iter(&self) -> dashmap::iter::Iter<String, EncryptedRecord> {
        self.hash_map.iter()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn salt(&self) -> String {
        self.salt.clone()
    }

    pub fn store_pwd(&self) -> String {
        self.store_pwd.clone()
    }

    pub fn update_metadata(&self, key: String, metadata: Metadata) {
        match self.hash_map.try_entry(key) {
            Some(entry) => {
                entry.and_modify(|r| r.metadata = metadata);
            }
            None => {
                let msg = "Couldn't get lock for update";
                panic!("{}", msg)
            }
        }
    }

    // V2 schema additions (rucksack >= v0.7.0)
    pub fn delete(&self, key: String) -> Option<bool> {
        match self.hash_map.remove(&key) {
            Some(_) => Some(true),
            None => Some(false),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn version(&self) -> versions::Versioning {
        self.version.clone()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::store::db;
    use crate::store::testing_data;
    use crate::time;

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
        assert!(tmp_db.version() > versions::Versioning::new("0.3.0").unwrap());
        let dpr = testing_data::plaintext_record();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.creds.user, "alice@site.com");
        assert_eq!(re_dpr.creds.password, "4 s3kr1t");
        assert!(tmp_db.close().is_ok());
        let tmp_db = db::open(path, pwd, salt).unwrap();
        let read_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(read_dpr.creds.user, "alice@site.com");
        assert_eq!(read_dpr.creds.password, "4 s3kr1t");
    }
}
