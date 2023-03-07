use anyhow::Result;

use crate::db::encrypted::EncryptedDB;

pub trait StoreManager {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String>;
    fn read(&self, path: String, pwd: String, salt: String) -> Result<EncryptedDB>;
}
