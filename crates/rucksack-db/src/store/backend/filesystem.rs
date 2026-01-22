use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

use super::backup;

#[derive(Clone, Default)]
pub struct FileSystemBackend {}

impl FileSystemBackend {
    pub fn new() -> FileSystemBackend {
        FileSystemBackend {}
    }
}

impl StoreManager for FileSystemBackend {
    fn backup(&self, src_file: &Path, dest_dir: &Path, version: &str) -> Result<PathBuf> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(&self, path: &Path, pwd: String, salt: String) -> Result<EncryptedDB> {
        EncryptedDB::from_file(path, pwd, salt)
    }
}
