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

use rucksack_lib::util;

use crate::records;
use crate::records::{DecryptedRecord, EncryptedRecord, Metadata};

use super::{backup, encrypted, versioned};

#[derive(Clone, Default)]
pub struct DB {
    pub path: String,
    pub backups_path: String,
    store_hash: u32,
    store_pwd: String,
    salt: String,
    hash_map: records::HashMap,
    enabled: bool,
    version: versions::SemVer,
}

impl fmt::Debug for DB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DB")
            .field("path", &self.path)
            .field("hash_map", &self.hash_map)
            .finish()
    }
}

pub fn init(path: String, backups_path: String, store_pwd: String, updated: String) -> Result<()> {
    let db = open(path, backups_path, store_pwd, updated)?;
    db.close()
}

pub fn new() -> DB {
    DB {
        ..Default::default()
    }
}

pub fn open(filename: String, backups_path: String, store_pwd: String, salt: String) -> Result<DB> {
    log::debug!("Opening database ...");
    let mut hash_map: records::HashMap = DashMap::new();
    let mut store_hash = 0;
    let mut version = crate::version();
    let vsn_db: versioned::VersionedDB;
    let file_path = util::create_parents(filename.clone())?;
    if file_path.exists() {
        log::debug!("Creating encrypted DB ...");
        let enc_db = encrypted::from_file(filename, store_pwd.clone(), salt.clone())?;
        log::debug!("Creating versioned DB ...");
        match versioned::deserialise(enc_db.decrypted()) {
            Ok(db) => {
                vsn_db = db;
            }
            Err(_) => {
                log::info!("Given database appears to be non-versioned; be sure to upgrade to the latest micro release of our old version before continuing ...");
                log::trace!("Bytes: {:?}", enc_db.decrypted());
                vsn_db = versioned::from_bytes(enc_db.decrypted());
            }
        }
        log::debug!("Getting database hash ...");
        store_hash = vsn_db.hash();
        version = vsn_db.version();
        // Decode the versioned DB's bytes to a hashmap
        hash_map = records::decode_hashmap(vsn_db.bytes(), version.clone())?;
    };
    let path = file_path.display().to_string();
    log::debug!("Setting database path: {}", path);
    Ok(DB {
        path,
        backups_path,
        store_hash,
        store_pwd,
        salt,
        hash_map,
        enabled: true,
        version,
    })
}

impl DB {
    pub fn backup_path(&self) -> String {
        self.backups_path.clone()
    }

    pub fn close(&self) -> Result<()> {
        log::debug!("Closing DB file ...");
        let path = util::create_parents(self.path())?;
        if path.exists() {
            log::debug!("Database file exists; backing up ...",);
            let backup_file =
                backup::copy(self.path(), self.backup_path(), self.version().to_string())?;
            log::debug!("Backed up file to {backup_file}");
        }

        // Reverse the workflow of `open` ... encode the hashmap
        let srl = match self.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise self";
                log::error!("{} {:?} ({:})", msg, self.path(), e);
                Err(anyhow!("{} {:?} ({:})", msg, self.path(), e))
            }
        }?;
        // Create versioned data
        let vsn_db = versioned::from_bytes(srl);
        let encoded = match vsn_db.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise version db";
                log::error!("{} {:?} ({:})", msg, self.path(), e);
                Err(anyhow!("{} {:?} ({:})", msg, self.path(), e))
            }
        }?;
        // Get the hash for the versioned data
        let store_hash = vsn_db.hash();
        if store_hash == self.store_hash {
            log::debug!("No change in store hash; not persisting ...");
            return Ok(());
        }
        // Encrypt the versioned data
        let enc_db =
            encrypted::from_decrypted(encoded, self.path(), self.store_pwd(), self.salt())?;

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
        log::debug!("Serialising data ...");
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in self.iter() {
            data.push((i.key().clone(), i.value().clone()))
        }
        log::trace!("Converted hashmap to vec.");
        data.sort_by_key(|k| k.0.clone());
        log::trace!("Sorted vec.");
        match bincode::encode_to_vec(data, util::bincode_cfg()) {
            Ok(encoded) => {
                log::trace!("Encoded vector.");
                Ok(encoded)
            }
            Err(e) => {
                let msg = format!("couldn't encode DB hashmap ({e:?})");
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
        log::trace!("Getting metadata of record with key {key} ...");
        match self.get(key.clone()) {
            Some(r) => Some(r.metadata()),
            None => {
                log::debug!("key {key} not found");
                None
            }
        }
    }

    pub fn hash_map(&self) -> records::HashMap {
        self.hash_map.clone()
    }

    pub fn insert(&self, record: DecryptedRecord) -> Option<EncryptedRecord> {
        let key = record.key();
        log::debug!("Inserting record with key {} ...", key);
        if let Some(r) = self.get(record.key()) {
            log::trace!("Record exists; skipping insert");
            return Some(r.encrypt(self.store_pwd(), self.salt()));
        };
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

    // Note that the key has to be passed here, even though the
    // updated record has a key() method; this is because an update
    // might involved a field used to create the key (and since that
    // new key hasn't been saved yet, there's no record for it --
    // just one for the old key).
    pub fn update(&self, key: String, updated: DecryptedRecord) {
        log::debug!("Updating record with key {key} ...");
        match self.delete(key) {
            Some(true) => {
                self.insert(updated);
            }
            Some(false) => log::error!("Could not update record:"),
            None => unreachable!(),
        }
    }

    pub fn update_metadata(&self, key: String, metadata: Metadata) {
        log::debug!("Updating metadata on record with key {key} ...");
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
        log::debug!("Deleting record with key {key} ...");
        match self.hash_map.remove(&key) {
            Some(_) => Some(true),
            None => Some(false),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn version(&self) -> versions::SemVer {
        self.version.clone()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use rucksack_lib::{time, util};

    use crate::testing;

    #[test]
    fn db_basics() {
        let pwd = testing::data::store_pwd();
        let salt = time::now();
        let temp_path = NamedTempFile::new().unwrap();
        assert!(temp_path.path().exists());
        let path_name = temp_path.path().display().to_string();
        println!("Got path_name: {path_name}");
        let backup_path = temp_path.path().parent().unwrap().join("backups");
        let res = util::create_dirs(backup_path.clone());
        assert!(res.is_ok());
        assert!(backup_path.exists());
        let backup_path_name = backup_path.display().to_string();
        println!("Got backup_path: {backup_path_name}");
        let tmp_db = super::open(
            path_name.clone(),
            backup_path_name.clone(),
            pwd.clone(),
            salt.clone(),
        )
        .unwrap();
        assert!(tmp_db.version() > versions::SemVer::new("0.8.0").unwrap());
        let dpr = testing::data::plaintext_record_v090();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.secrets.user, "alice@site.com");
        assert_eq!(re_dpr.secrets.password, "6 s3kr1t");
        assert!(tmp_db.close().is_ok());
        let tmp_db = super::open(path_name, backup_path_name, pwd, salt).unwrap();
        let read_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(read_dpr.secrets.user, "alice@site.com");
        assert_eq!(read_dpr.secrets.password, "6 s3kr1t");
        assert_eq!(read_dpr.history.len(), 2);
        assert_eq!(read_dpr.history[0].secrets.password, "4 s3kr1t");
        assert_eq!(read_dpr.history[1].secrets.password, "5 s3kr1t");
    }
}
