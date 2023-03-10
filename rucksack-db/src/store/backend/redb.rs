use anyhow::Result;

use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

use super::backup;

#[derive(Clone, Default)]
pub struct ReDBBackend {}

impl ReDBBackend {
    pub fn new() -> ReDBBackend {
        ReDBBackend {}
    }
}

impl StoreManager for ReDBBackend {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(&self, _path: String, _pwd: String, _salt: String) -> Result<EncryptedDB> {
        todo!()
    }
}
