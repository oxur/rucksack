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

use rucksack_lib::{file, util};

use crate::records;
use crate::records::{DecryptedRecord, EncryptedRecord, Metadata};
use crate::store;
use crate::store::manager::StoreManager;

use super::{encrypted, versioned};

pub struct DB {
    pub file_name: String,
    backup_dir: String,
    enabled: bool,
    hash_map: records::HashMap,
    manager: Box<dyn StoreManager>,
    salt: Option<String>,
    store_hash: u32,
    store_pwd: Option<String>,
    version: versions::SemVer,
}

impl fmt::Debug for DB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DB")
            .field("path", &self.file_name)
            .field("hash_map", &self.hash_map)
            .finish()
    }
}

impl DB {
    pub fn new(
        file_name: String,
        backup_dir: String,
        store_pwd: Option<String>,
        salt: Option<String>,
    ) -> DB {
        DB {
            file_name,
            backup_dir,
            store_pwd,
            salt,
            manager: store::manager::new(),
            enabled: true,
            hash_map: DashMap::new(),
            store_hash: 0,
            version: records::version(),
        }
    }

    // Moved in v0.9.0
    pub fn init(
        file_name: String,
        backup_dir: String,
        store_pwd: Option<String>,
        salt: Option<String>,
    ) -> Result<()> {
        log::debug!("Initialising database ...");
        let mut db = DB::new(file_name, backup_dir, store_pwd, salt);
        db.open()?;
        db.close()
    }

    // Moved in v0.9.0
    pub fn open(&mut self) -> Result<()> {
        log::debug!("Opening database ...");
        let store_pwd = self.store_pwd.clone().unwrap();
        let salt = self.salt.clone().unwrap();
        let file_path = file::create_parents(self.file_name.clone())?;
        if file_path.exists() {
            log::debug!("Creating encrypted DB ...");
            let enc_db = self.manager.read(self.file_name.clone(), store_pwd, salt)?;
            let vsn_db = match versioned::deserialise(enc_db.decrypted()) {
                Ok(db) => db,
                Err(_) => {
                    log::info!("Given database appears to be non-versioned; be sure to upgrade to the latest micro release of our old version before continuing ...");
                    log::trace!("Bytes: {:?}", enc_db.decrypted());
                    versioned::from_bytes(enc_db.decrypted())
                }
            };
            log::debug!("Getting database hash ...");
            self.store_hash = vsn_db.hash();
            self.version = vsn_db.version();
            // Decode the versioned DB's bytes to a hashmap
            self.hash_map = records::decode_hashmap(vsn_db.bytes(), self.version.clone())?;
        };

        self.file_name = file_path.display().to_string();
        self.enabled = true;
        log::debug!("Set database path: {}", self.file_name);
        Ok(())
    }

    pub fn backup_dir(&self) -> String {
        self.backup_dir.clone()
    }

    pub fn close(&self) -> Result<()> {
        log::debug!("Closing DB file ...");
        let path = file::create_parents(self.file_name())?;
        if path.exists() {
            log::debug!("Database file exists; backing up ...");
            let backup_file = self.manager.backup(
                self.file_name(),
                self.backup_dir(),
                self.schema_version().to_string(),
            )?;
            log::debug!("Backed up file to {backup_file}");
        }

        // Reverse the workflow of `open` ... encode the hashmap
        let srl = match self.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise self";
                log::error!("{} {:?} ({:})", msg, self.file_name(), e);
                Err(anyhow!("{} {:?} ({:})", msg, self.file_name(), e))
            }
        }?;
        // Create versioned data
        let vsn_db = versioned::from_bytes(srl);
        let encoded = match vsn_db.serialise() {
            Ok(x) => Ok(x),
            Err(e) => {
                let msg = "Could not serialise version db";
                log::error!("{} {:?} ({:})", msg, self.file_name(), e);
                Err(anyhow!("{} {:?} ({:})", msg, self.file_name(), e))
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
            encrypted::from_decrypted(encoded, self.file_name(), self.store_pwd(), self.salt())?;

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

    // Added in v0.7.0
    pub fn delete(&self, key: String) -> Option<bool> {
        log::debug!("Deleting record with key {key} ...");
        match self.hash_map.remove(&key) {
            Some(_) => Some(true),
            None => Some(false),
        }
    }

    // Added in v0.7.0
    pub fn enabled(&self) -> bool {
        self.enabled
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

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn salt(&self) -> String {
        self.salt.clone().unwrap()
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

    pub fn store_pwd(&self) -> String {
        self.store_pwd.clone().unwrap()
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

    // Added in v0.7.0
    pub fn version(&self) -> versions::SemVer {
        self.version.clone()
    }

    // Added in v0.10.1
    pub fn schema_version(&self) -> versions::SemVer {
        records::version()
    }
}

#[cfg(test)]
mod tests {
    use rucksack_lib::time;

    use crate::testing;

    #[test]
    fn db_basics() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        let mut r = db_handler.setup();
        assert!(r.is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();
        println!("Got db_file: {db_file}");
        println!("Got backups_path: {backups}");

        // Store data and close
        let mut tmp_db =
            super::DB::new(db_file.clone(), backups.clone(), pwd.clone(), salt.clone());
        assert!(tmp_db.open().is_ok());
        assert!(tmp_db.version() > versions::SemVer::new("0.8.0").unwrap());
        let dpr = testing::data::plaintext_record_v090();
        tmp_db.insert(dpr.clone());
        let re_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(re_dpr.secrets.user, "alice@site.com");
        assert_eq!(re_dpr.secrets.password, "6 s3kr1t");
        assert!(tmp_db.close().is_ok());

        // Re-open DB and check stored data
        let mut tmp_db = super::DB::new(db_file, backups, pwd, salt);
        assert!(tmp_db.open().is_ok());
        let read_dpr = tmp_db.get(dpr.key()).unwrap();
        assert_eq!(read_dpr.secrets.user, "alice@site.com");
        assert_eq!(read_dpr.secrets.password, "6 s3kr1t");
        assert_eq!(read_dpr.history.len(), 2);
        assert_eq!(read_dpr.history[0].secrets.password, "4 s3kr1t");
        assert_eq!(read_dpr.history[1].secrets.password, "5 s3kr1t");
        assert!(tmp_db.close().is_ok());
        r = db_handler.teardown();
        assert!(r.is_ok());
    }
}
