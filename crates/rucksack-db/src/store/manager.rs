use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::db::encrypted::EncryptedDB;

pub trait StoreManager {
    fn backup(&self, src_file: &Path, dest_dir: &Path, version: &str) -> Result<PathBuf>;
    fn read(&self, path: &Path, pwd: String, salt: String) -> Result<EncryptedDB>;
}

pub fn new() -> Box<dyn StoreManager> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "filesystem")] {
            Box::new(super::backend::FileSystemBackend::new())
        } else if #[cfg(feature = "redb")] {
            Box::new(super::backend::ReDBBackend::new())
        } else {
            todo!()
        }
    }
}
