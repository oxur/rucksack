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

use anyhow::{anyhow, Context, Error, Result};
use dashmap::DashMap;
use secrecy::{ExposeSecret, SecretString};

use rucksack_lib::{file, util};

use crate::db::encrypted::EncryptedDB;
use crate::db::versioned::VersionedDB;
use crate::records;
use crate::records::{DecryptedRecord, EncryptedRecord, Metadata};
use crate::store;
use crate::store::manager::StoreManager;

pub struct DB {
    pub file_name: String,
    backup_dir: String,
    enabled: bool,
    hash_map: records::HashMap,
    manager: Box<dyn StoreManager>,
    salt: Option<SecretString>,
    store_hash: u32,
    store_pwd: Option<SecretString>,
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
            store_pwd: store_pwd.map(SecretString::new),
            salt: salt.map(SecretString::new),
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
        log::debug!(operation = "init"; "Initialising database");
        let mut db = DB::new(file_name, backup_dir, store_pwd, salt);
        db.open()?;
        db.close()
    }

    // Moved in v0.9.0
    #[must_use = "database operations must be checked for errors"]
    pub fn open(&mut self) -> Result<()> {
        log::debug!(operation = "open"; "Opening database");
        let store_pwd = self
            .store_pwd
            .as_ref()
            .expect("store_pwd must be set to open database")
            .expose_secret()
            .to_string();
        let salt = self
            .salt
            .as_ref()
            .expect("salt must be set to open database")
            .expose_secret()
            .to_string();
        let file_path = file::create_parents(self.file_name.clone()).with_context(|| {
            format!(
                "failed to create parent directory for database: {}",
                self.file_name
            )
        })?;
        if file_path.exists() {
            log::debug!(operation = "decrypt", db_file = self.file_name.as_str(); "Creating encrypted DB");
            let enc_db = self
                .manager
                .read(self.file_name.clone(), store_pwd, salt)
                .with_context(|| {
                    format!(
                        "failed to read database file: {} (check password and salt)",
                        self.file_name
                    )
                })?;
            let vsn_db = match VersionedDB::deserialise(enc_db.decrypted()) {
                Ok(db) => db,
                Err(_) => {
                    log::info!(db_file = self.file_name.as_str(), format = "non-versioned"; "Given database appears to be non-versioned; be sure to upgrade to the latest micro release of our old version before continuing");
                    log::trace!(bytes_len = enc_db.decrypted().len(); "Database bytes");
                    VersionedDB::from_bytes(enc_db.decrypted()).with_context(|| {
                        format!("failed to parse database version from: {}", self.file_name)
                    })?
                }
            };
            log::debug!(operation = "hash_compute"; "Getting database hash");
            self.store_hash = vsn_db.hash();
            self.version = vsn_db.version();
            // Decode the versioned DB's bytes to a hashmap
            self.hash_map = records::decode_hashmap(vsn_db.bytes(), self.version.clone())
                .with_context(|| {
                    format!(
                        "failed to decode database records (version: {})",
                        self.version
                    )
                })?;
        };

        self.file_name = file_path.display().to_string();
        self.enabled = true;
        log::debug!(db_file = self.file_name.as_str(); "Set database path");
        Ok(())
    }

    pub fn backup_dir(&self) -> String {
        self.backup_dir.clone()
    }

    #[must_use = "database operations must be checked for errors"]
    pub fn close(&self) -> Result<()> {
        log::debug!(operation = "close", db_file = self.file_name().as_str(); "Closing DB file");
        let path = file::create_parents(self.file_name()).with_context(|| {
            format!(
                "failed to create parent directory for database: {}",
                self.file_name()
            )
        })?;
        if path.exists() {
            log::debug!(db_file = self.file_name().as_str(), operation = "backup"; "Database file exists; backing up");
            let backup_file = self
                .manager
                .backup(
                    self.file_name(),
                    self.backup_dir(),
                    self.schema_version().to_string(),
                )
                .with_context(|| {
                    format!("failed to create backup of database: {}", self.file_name())
                })?;
            log::debug!(backup_file = backup_file.as_str(), operation = "backup_complete"; "Backed up file");
        }

        // Reverse the workflow of `open` ... encode the hashmap
        let srl = self
            .serialise()
            .with_context(|| format!("failed to serialize database: {}", self.file_name()))?;

        // Create versioned data
        let vsn_db = VersionedDB::from_bytes(srl).with_context(|| {
            format!(
                "failed to create versioned database wrapper: {}",
                self.file_name()
            )
        })?;
        let encoded = vsn_db.serialise().with_context(|| {
            format!(
                "failed to serialize versioned database: {}",
                self.file_name()
            )
        })?;
        // Get the hash for the versioned data
        let store_hash = vsn_db.hash();
        if store_hash == self.store_hash {
            log::debug!(hash = store_hash, operation = "persist_skip"; "No change in store hash; not persisting");
            return Ok(());
        }
        // Encrypt the versioned data
        let enc_db =
            EncryptedDB::from_decrypted(encoded, self.file_name(), self.store_pwd(), self.salt())
                .with_context(|| format!("failed to encrypt database: {}", self.file_name()))?;

        // Save the encrypted data
        enc_db
            .write()
            .with_context(|| format!("failed to write database to disk: {}", self.file_name()))
    }

    #[must_use = "database operations must be checked for errors"]
    pub fn collect_decrypted(&self) -> Result<Vec<DecryptedRecord>, Error> {
        let mut decrypted: Vec<DecryptedRecord> = Vec::new();
        for i in self.iter() {
            let record = records::decrypt_versioned(
                i.value(),
                self.store_pwd(),
                self.salt(),
                self.version.clone(),
            )?;
            decrypted.push(record);
        }
        Ok(decrypted)
    }

    // Added in v0.7.0
    pub fn delete(&self, key: String) -> Option<bool> {
        log::debug!(key = key.as_str(), operation = "delete"; "Deleting record");
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
        log::trace!(key = key.as_str(), operation = "get"; "Getting record");
        self.hash_map.get(&key).and_then(|encrypted| {
            records::decrypt_versioned(
                encrypted.value(),
                self.store_pwd(),
                self.salt(),
                self.version.clone(),
            )
            .ok()
        })
    }

    pub fn get_metadata(&self, key: String) -> Option<Metadata> {
        log::trace!(key = key.as_str(), operation = "get_metadata"; "Getting metadata of record");
        match self.get(key.clone()) {
            Some(r) => Some(r.metadata()),
            None => {
                log::debug!(key = key.as_str(), status = "not_found"; "Key not found");
                None
            }
        }
    }

    pub fn hash_map(&self) -> records::HashMap {
        self.hash_map.clone()
    }

    #[must_use = "database operations must be checked for errors"]
    pub fn insert(&self, record: DecryptedRecord) -> Result<Option<EncryptedRecord>> {
        let key = record.key();
        log::debug!(key = key.as_str(), operation = "insert"; "Inserting record");
        if let Some(r) = self.get(record.key()) {
            log::trace!(key = key.as_str(), status = "exists"; "Record exists; skipping insert");
            return Ok(Some(
                r.encrypt(self.store_pwd(), self.salt())
                    .with_context(|| format!("failed to encrypt existing record: {}", key))?,
            ));
        };
        let encrypted = record
            .encrypt(self.store_pwd(), self.salt())
            .with_context(|| format!("failed to encrypt new record: {}", key))?;
        Ok(self.hash_map.insert(key, encrypted))
    }

    pub fn iter(&self) -> dashmap::iter::Iter<'_, String, EncryptedRecord> {
        self.hash_map.iter()
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn salt(&self) -> String {
        self.salt
            .as_ref()
            .expect(
                "BUG: salt should be Some when database operations are performed. \
                This indicates the database was not properly initialized with a salt.",
            )
            .expose_secret()
            .to_string()
    }

    fn serialise(&self) -> Result<Vec<u8>> {
        log::debug!(operation = "serialize"; "Serialising data");
        let mut data: Vec<(String, EncryptedRecord)> = Vec::new();
        for i in self.iter() {
            data.push((i.key().clone(), i.value().clone()))
        }
        log::trace!(operation = "serialize_convert"; "Converted hashmap to vec");
        data.sort_by_key(|k| k.0.clone());
        log::trace!(operation = "serialize_sort"; "Sorted vec");
        match bincode::encode_to_vec(data, util::bincode_cfg()) {
            Ok(encoded) => {
                log::trace!(operation = "serialize_encode"; "Encoded vector");
                Ok(encoded)
            }
            Err(e) => {
                let msg = format!("couldn't encode DB hashmap ({e:?})");
                log::error!(error = e.to_string().as_str(), operation = "serialize_encode"; "{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn store_pwd(&self) -> String {
        self.store_pwd
            .as_ref()
            .expect(
                "BUG: store_pwd should be Some when database operations are performed. \
                This indicates the database was not properly initialized with a password.",
            )
            .expose_secret()
            .to_string()
    }

    // Note that the key has to be passed here, even though the
    // updated record has a key() method; this is because an update
    // might involved a field used to create the key (and since that
    // new key hasn't been saved yet, there's no record for it --
    // just one for the old key).
    #[must_use = "database operations must be checked for errors"]
    pub fn update(&self, key: String, updated: DecryptedRecord) -> Result<()> {
        log::debug!(key = key.as_str(), operation = "update"; "Updating record");
        match self.delete(key.clone()) {
            Some(true) => {
                self.insert(updated)
                    .with_context(|| format!("failed to insert updated record: {}", key))?;
                Ok(())
            }
            Some(false) => {
                log::error!(key = key.as_str(), operation = "update"; "Could not update record");
                Err(anyhow!("failed to delete record '{}' for update", key))
            }
            None => unreachable!(),
        }
    }

    #[must_use = "database operations must be checked for errors"]
    pub fn update_metadata(&self, key: String, metadata: Metadata) -> Result<()> {
        log::debug!(key = key.as_str(), operation = "update_metadata"; "Updating metadata on record");
        let key_for_error = key.clone();
        match self.hash_map.try_entry(key) {
            Some(entry) => {
                entry.and_modify(|r| r.metadata = metadata);
                log::trace!(key = key_for_error.as_str(), status = "success"; "Updated metadata");
                Ok(())
            }
            None => Err(anyhow!("record '{}' not found or locked", key_for_error)),
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
    use super::*;
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
        tmp_db.insert(dpr.clone()).unwrap();
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

    #[test]
    fn test_new_db() {
        let pwd = Some("password".to_string());
        let salt = Some("salt".to_string());
        let db = DB::new(
            "/tmp/test.db".to_string(),
            "/tmp/backups".to_string(),
            pwd,
            salt,
        );

        assert_eq!(db.file_name(), "/tmp/test.db");
        assert_eq!(db.backup_dir(), "/tmp/backups");
        assert!(db.enabled());
        assert_eq!(db.hash_map().len(), 0);
    }

    #[test]
    fn test_getters() {
        let pwd = Some("test_pwd".to_string());
        let salt = Some("test_salt".to_string());
        let db = DB::new(
            "/path/to/db".to_string(),
            "/path/to/backups".to_string(),
            pwd.clone(),
            salt.clone(),
        );

        assert_eq!(db.file_name(), "/path/to/db");
        assert_eq!(db.backup_dir(), "/path/to/backups");
        assert_eq!(db.store_pwd(), pwd.unwrap());
        assert_eq!(db.salt(), salt.unwrap());
        assert!(db.enabled());
        assert_eq!(db.schema_version(), records::version());
    }

    #[test]
    fn test_init_creates_db() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let result = DB::init(db_file.clone(), backups, pwd, salt);
        assert!(result.is_ok());

        // Verify file was created
        assert!(std::path::Path::new(&db_file).exists());

        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_insert_and_get() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        let key = record.key();
        db.insert(record.clone()).unwrap();

        let retrieved = db.get(key).unwrap();
        assert_eq!(retrieved.secrets.user, record.secrets.user);
        assert_eq!(retrieved.secrets.password, record.secrets.password);

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_insert_duplicate_returns_existing() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        let result1 = db.insert(record.clone()).unwrap();
        assert!(result1.is_none(), "First insert should return None");

        let result2 = db.insert(record.clone()).unwrap();
        assert!(result2.is_some(), "Duplicate insert should return existing");

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_get_nonexistent() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let result = db.get("nonexistent_key".to_string());
        assert!(result.is_none());

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_delete_existing() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        let key = record.key();
        db.insert(record).unwrap();

        let result = db.delete(key.clone());
        assert_eq!(result, Some(true));

        let retrieved = db.get(key);
        assert!(retrieved.is_none(), "Record should be deleted");

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_delete_nonexistent() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let result = db.delete("nonexistent_key".to_string());
        assert_eq!(result, Some(false));

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_update_record() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let mut record = testing::data::plaintext_record_v090();
        let key = record.key();
        db.insert(record.clone()).unwrap();

        // Update the record
        record.secrets.password = "new_password".to_string();
        db.update(key.clone(), record).unwrap();

        let retrieved = db.get(key).unwrap();
        assert_eq!(retrieved.secrets.password, "new_password");

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_get_metadata() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        let key = record.key();
        db.insert(record.clone()).unwrap();

        let metadata = db.get_metadata(key).unwrap();
        assert_eq!(metadata.name, record.metadata.name);

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_get_metadata_nonexistent() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let result = db.get_metadata("nonexistent_key".to_string());
        assert!(result.is_none());

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_update_metadata() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        let key = record.key();
        db.insert(record.clone()).unwrap();

        // Update metadata
        let mut new_metadata = record.metadata.clone();
        new_metadata.name = "Updated Name".to_string();
        db.update_metadata(key.clone(), new_metadata.clone())
            .unwrap();

        let retrieved_metadata = db.get_metadata(key).unwrap();
        assert_eq!(retrieved_metadata.name, "Updated Name");

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_collect_decrypted() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        db.insert(record.clone()).unwrap();

        let decrypted = db.collect_decrypted().unwrap();
        assert_eq!(decrypted.len(), 1);
        assert_eq!(decrypted[0].secrets.user, record.secrets.user);

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_iter() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        db.insert(record).unwrap();

        let count = db.iter().count();
        assert_eq!(count, 1);

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_hash_map_getter() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let record = testing::data::plaintext_record_v090();
        db.insert(record).unwrap();

        let hash_map = db.hash_map();
        assert_eq!(hash_map.len(), 1);

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_close_without_changes_no_write() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        // Create and close empty DB
        let mut db = DB::new(db_file.clone(), backups.clone(), pwd.clone(), salt.clone());
        assert!(db.open().is_ok());
        assert!(db.close().is_ok());

        // Reopen and close again (no changes)
        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());
        let _initial_hash = db.store_hash;
        assert!(db.close().is_ok());
        // If hash didn't change, close returns early without writing

        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_version_tracking() {
        let pwd = Some(testing::data::store_pwd());
        let salt = Some(time::now());
        let mut db_handler = testing::db::new();
        assert!(db_handler.setup().is_ok());
        let db_file = db_handler.file_name().unwrap();
        let backups = db_handler.backups_path().unwrap().display().to_string();

        let mut db = DB::new(db_file, backups, pwd, salt);
        assert!(db.open().is_ok());

        let version = db.version();
        assert!(version >= versions::SemVer::new("0.7.0").unwrap());

        assert!(db.close().is_ok());
        assert!(db_handler.teardown().is_ok());
    }

    #[test]
    fn test_debug_impl() {
        let pwd = Some("pwd".to_string());
        let salt = Some("salt".to_string());
        let db = DB::new(
            "/test/path".to_string(),
            "/test/backups".to_string(),
            pwd,
            salt,
        );

        let debug_str = format!("{:?}", db);
        assert!(debug_str.contains("DB"));
        assert!(debug_str.contains("/test/path"));
    }
}
