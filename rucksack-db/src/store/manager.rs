use anyhow::Result;

use crate::db::encrypted::EncryptedDB;

pub trait StoreManager {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String>;
    fn read(&self, path: String, pwd: String, salt: String) -> Result<EncryptedDB>;
}

pub fn new() -> Box<dyn StoreManager> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "filesystem")] {
            Box::new(super::backend::FileSystemBackend::new())
        } else if #[cfg(feature = "persy")] {
            Box::new(super::backend::PersyBackend::new())
        } else if #[cfg(feature = "redb")] {
            Box::new(super::backend::ReDBBackend::new())
        } else {
            todo!()
        }
    }
}
