use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::db::encrypted::EncryptedDB;
use crate::store::manager::StoreManager;

use super::backup;

// When both filesystem and redb features are enabled, filesystem takes precedence
// in manager::new(), making this code unused
#[cfg_attr(all(feature = "redb", feature = "filesystem"), allow(dead_code))]
#[derive(Clone, Default)]
pub struct ReDBBackend {}

#[cfg_attr(all(feature = "redb", feature = "filesystem"), allow(dead_code))]
impl ReDBBackend {
    pub fn new() -> ReDBBackend {
        ReDBBackend {}
    }
}

impl StoreManager for ReDBBackend {
    fn backup(&self, src_file: &Path, dest_dir: &Path, version: &str) -> Result<PathBuf> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(&self, _path: &Path, _pwd: &str, _salt: &str) -> Result<EncryptedDB> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_new() {
        let backend = ReDBBackend::new();
        // Just verify it constructs
        let _cloned = backend.clone();
    }

    #[test]
    fn test_default() {
        let backend = ReDBBackend::default();
        let _cloned = backend.clone();
    }

    #[test]
    fn test_backup() {
        let backend = ReDBBackend::new();

        // Create a temp source file
        let mut src_file = NamedTempFile::new().unwrap();
        src_file.write_all(b"test data").unwrap();
        let src_path = src_file.path();

        // Create a temp destination directory
        let dest_dir = tempdir().unwrap();
        let dest_path = dest_dir.path();

        // Test backup
        let result = backend.backup(src_path, dest_path, "1.0.0");
        assert!(result.is_ok());

        let backup_file = result.unwrap();
        assert!(backup_file.exists());
        assert!(backup_file.to_string_lossy().contains("v1.0.0"));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_read_not_implemented() {
        let backend = ReDBBackend::new();
        let _ = backend.read(std::path::Path::new("/some/path"), "password", "salt");
    }

    #[test]
    fn test_clone() {
        let backend1 = ReDBBackend::new();
        let backend2 = backend1.clone();

        // Both should work the same way
        let mut src_file = NamedTempFile::new().unwrap();
        src_file.write_all(b"data").unwrap();

        let dest_dir = tempdir().unwrap();

        let result1 = backend1.backup(src_file.path(), dest_dir.path(), "1.0.0");

        let result2 = backend2.backup(src_file.path(), dest_dir.path(), "1.0.0");

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }
}
