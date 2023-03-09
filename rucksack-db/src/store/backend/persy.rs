use anyhow::Result;

use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

#[derive(Clone, Default)]
pub struct PersyBackend {}

impl PersyBackend {
    pub fn new() -> PersyBackend {
        PersyBackend {}
    }
}

impl StoreManager for PersyBackend {
    fn backup(&self, _src_file: String, _dest_dir: String, _version: String) -> Result<String> {
        todo!()
    }

    fn read(&self, _path: String, _pwd: String, _salt: String) -> Result<EncryptedDB> {
        todo!()
    }
}
