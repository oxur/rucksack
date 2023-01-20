// Database encryption <-> decryption flow:
//
// When a rucksack database is serialised, the following happens:
// * Its hashmap (DashMap) is bincoded to bytes
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
// * The bytes of the VersionDB are bincode-decoded to a hashmap (DashMap)
// * The hashmap is stored as a field on the DB struct
//
use std::{fmt, fs};

use anyhow::{anyhow, Error, Result};
use bincode::config;
use dashmap::DashMap;

use crate::store::record::{DecryptedRecord, EncryptedRecord, Metadata};
use crate::time;

pub mod encrypted;
pub mod versioned;

#[derive(Clone, Default)]
pub struct DB {
    pub path: String,
    store_hash: u32,
    store_pwd: String,
    salt: String,
    hash_map: DashMap<String, EncryptedRecord>,
    enabled: bool,
    version: String,
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
    let mut hash_map: DashMap<String, EncryptedRecord> = DashMap::new();
    let mut store_hash = 0;
    let mut version = env!("CARGO_PKG_VERSION").to_string();
    if std::path::Path::new(&path).exists() {
        // Decrypt the stored data
        let mut enc_db = encrypted::from_file(path.clone(), store_pwd.clone(), salt.clone());
        enc_db.read()?;
        enc_db.decrypt()?;
        // Decode the decrypted data as a VersionedDB
        let vsn_db = versioned::from_encoded(enc_db.decrypted())?;
        store_hash = vsn_db.hash();
        version = vsn_db.version();
        // Decode the hashmap
        hash_map = hashmap_from_encoded(vsn_db.bytes())?;
    };
    log::debug!("Setting database path: {}", path);
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

fn hashmap_from_encoded(bytes: Vec<u8>) -> Result<DashMap<String, EncryptedRecord>> {
    let hashmap: DashMap<String, EncryptedRecord>;
    match bincode::serde::decode_from_slice(bytes.as_ref(), config::standard()) {
        Ok((result, _len)) => {
            hashmap = result;
            Ok(hashmap)
        }
        Err(e) => {
            let msg = format!("couldn't deserialise bincoded database bytes: {:?}", e);
            log::error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

impl DB {
    pub fn close(&self) -> Result<()> {
        let path_name = &self.path();
        let path = std::path::Path::new(path_name);
        // Reverse the workflow of `open` ... encode the hashmap
        let encoded = self.serialise()?;
        // Create versioned data
        let vsn_db = versioned::from_bytes(encoded);
        let encoded = vsn_db.serialise()?;
        // Get the hash for the versioned data
        let store_hash = vsn_db.hash();
        if store_hash == self.store_hash {
            log::debug!("No change in store hash; not persisting ...");
            return Ok(());
        }
        // Encrypt the versioned data
        let mut enc_db = encrypted::from_bytes(encoded, self.path(), self.store_pwd(), self.salt());
        enc_db.encrypt();
        fs::create_dir_all(path.parent().unwrap())?;
        // TODO: refactor backup code
        if std::path::Path::new(&self.path).exists() {
            let backup_name = format!("{}-{}", self.path, time::simple_timestamp());
            log::debug!(
                "Path to db already exists; backing up to {} ...",
                backup_name
            );
            std::fs::copy(path, backup_name)?;
        }
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
        match bincode::serde::encode_to_vec(self.hash_map.clone(), config::standard()) {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                let msg = format!("couldn't encode DB hashmap ({})", e);
                log::error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn get(&self, key: String) -> Option<DecryptedRecord> {
        log::trace!("Getting record with key {} ...", key);
        self.hash_map
            .get(&key)
            .map(|encrypted| encrypted.decrypt(self.store_pwd(), self.salt()).unwrap())
    }

    pub fn get_metadata(&self, key: String) -> Option<Metadata> {
        log::trace!("Getting metadata of record with key {} ...", key);
        match self.get(key.clone()) {
            Some(r) => Some(r.metadata()),
            None => {
                log::debug!("key {:} not found", key);
                None
            }
        }
    }

    pub fn hash_map(&self) -> DashMap<String, EncryptedRecord> {
        self.hash_map.clone()
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        let key = record.key();
        log::trace!("Inserting record with key {} ...", key);
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
        log::trace!("Updating metadata on record with key {} ...", key);
        match self.hash_map.try_entry(key) {
            Some(entry) => {
                entry.and_modify(|r| r.metadata = metadata);
                log::trace!("updated!")
            }
            None => {
                let msg = "Couldn't get lock for update";
                log::error!("{}", msg);
                panic!("{}", msg)
            }
        }
    }

    // V2 schema additions (rucksack >= v0.7.0)
    pub fn delete(&self, key: String) -> Option<bool> {
        log::trace!("Deleting record with key {} ...", key);
        match self.hash_map.remove(&key) {
            Some(_) => Some(true),
            None => Some(false),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn version(&self) -> String {
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
