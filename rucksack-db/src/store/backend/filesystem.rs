use anyhow::Result;

use crate::db::backup;
use crate::db::encrypted;
use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

pub struct FileSystemBackend {}

impl StoreManager for FileSystemBackend {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(&self, path: String, pwd: String, salt: String) -> Result<EncryptedDB> {
        encrypted::from_file(path, pwd, salt)
    }
}
