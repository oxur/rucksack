use anyhow::Result;

use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

use super::backup;

#[derive(Clone, Default)]
pub struct PersyBackend {}

impl PersyBackend {
    pub fn new() -> PersyBackend {
        PersyBackend {}
    }
}

impl StoreManager for PersyBackend {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(&self, _path: String, _pwd: String, _salt: String) -> Result<EncryptedDB> {
        todo!()
    }
}
