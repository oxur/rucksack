use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, Secret, SecretString};

use rucksack_lib::file;

use crate::crypto;

pub struct EncryptedDB {
    bytes: Vec<u8>,
    decrypted: Secret<Vec<u8>>,
    path: String,
    pwd: SecretString,
    salt: SecretString,
}

impl EncryptedDB {
    pub fn from_decrypted(
        decrypted: Vec<u8>,
        path: String,
        pwd: String,
        salt: String,
    ) -> Result<EncryptedDB> {
        EncryptedDB::new(None, Some(decrypted), path, pwd, salt)
    }

    pub fn from_encrypted(
        encrypted: Vec<u8>,
        path: String,
        pwd: String,
        salt: String,
    ) -> Result<EncryptedDB> {
        EncryptedDB::new(Some(encrypted), None, path, pwd, salt)
    }

    pub fn from_file(path: String, pwd: String, salt: String) -> Result<EncryptedDB> {
        EncryptedDB::new(None, None, path, pwd, salt)
    }

    pub fn new(
        bytes: Option<Vec<u8>>,
        decrypted: Option<Vec<u8>>,
        path: String,
        pwd: String,
        salt: String,
    ) -> Result<EncryptedDB> {
        let mut edb = EncryptedDB {
            bytes: Vec::new(),
            decrypted: Secret::new(Vec::new()),
            path,
            pwd: SecretString::new(pwd),
            salt: SecretString::new(salt),
        };
        if bytes.is_none() && decrypted.is_none() {
            log::debug!(source = "file", path = edb.path.as_str(), operation = "init"; "No bytes provided; reading from file");
            edb.read()?;
            edb.decrypt()?;
        } else if let Some(b) = bytes {
            log::debug!(source = "bytes", operation = "decrypt"; "Got encrypted bytes; decrypting");
            edb.bytes = b;
            edb.decrypt()?;
        } else if let Some(d) = decrypted {
            log::debug!(source = "bytes", operation = "encrypt"; "Got decrypted bytes; encrypting");
            edb.decrypted = Secret::new(d);
            edb.encrypt()?;
        }
        Ok(edb)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn decrypt(&mut self) -> Result<()> {
        log::debug!(operation = "decrypt"; "Decrypting stored bytes");
        log::trace!(pwd_len = self.pwd().len(), salt_len = self.salt().len(); "Credentials info");
        match crypto::decrypt(self.bytes.clone(), self.pwd(), self.salt()) {
            Ok(bytes) => {
                log::trace!(bytes_len = bytes.len(), operation = "decrypt_success"; "Decrypted bytes");
                self.decrypted = Secret::new(bytes);
                Ok(())
            }
            Err(e) => {
                let msg = format!("Could not decrypt data: {e:?}");
                log::error!(error = e.to_string().as_str(), operation = "decrypt"; "{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    pub fn decrypted(&self) -> Vec<u8> {
        self.decrypted.expose_secret().to_vec()
    }

    pub fn encrypt(&mut self) -> Result<()> {
        log::trace!(bytes_len_before = self.bytes.len(), operation = "encrypt"; "Byte length before encryption");
        self.bytes = crypto::encrypt(self.decrypted(), self.pwd(), self.salt())?;
        log::trace!(bytes_len_after = self.bytes.len(), operation = "encrypt"; "Byte length after encryption");
        Ok(())
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn pwd(&self) -> String {
        self.pwd.expose_secret().to_string()
    }

    pub fn read(&mut self) -> Result<()> {
        log::trace!(bytes_len_before = self.bytes.len(), operation = "read"; "Byte length before read");
        self.bytes = file::read(self.path())?;
        log::trace!(bytes_len_after = self.bytes.len(), operation = "read"; "Byte length after read");
        Ok(())
    }

    pub fn salt(&self) -> String {
        self.salt.expose_secret().to_string()
    }

    pub fn write(&self) -> Result<()> {
        log::debug!(operation = "write", path = self.path().as_str(); "Writing encrypted DB");
        file::write(self.bytes(), self.path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup_test_dir() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test_db.bin");
        (dir, path)
    }

    #[test]
    fn test_from_decrypted_basic() {
        let data = b"Hello, World!".to_vec();
        let pwd = "test_password".to_string();
        let salt = "test_salt".to_string();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data.clone(),
            path.to_str().unwrap().to_string(),
            pwd.clone(),
            salt.clone(),
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
        assert_eq!(edb.pwd(), pwd);
        assert_eq!(edb.salt(), salt);
        assert!(
            !edb.bytes().is_empty(),
            "Encrypted bytes should not be empty"
        );
        assert_ne!(edb.bytes(), data, "Encrypted should differ from decrypted");
    }

    #[test]
    fn test_from_decrypted_empty() {
        let data = Vec::new();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data.clone(),
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
    }

    #[test]
    fn test_from_encrypted_basic() {
        let data = b"Test data".to_vec();
        let pwd = "password".to_string();
        let salt = "salt123".to_string();
        let (_dir, path) = setup_test_dir();

        // First encrypt some data
        let encrypted = crypto::encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();

        // Create from encrypted
        let edb = EncryptedDB::from_encrypted(
            encrypted.clone(),
            path.to_str().unwrap().to_string(),
            pwd,
            salt,
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
        assert_eq!(edb.bytes(), encrypted);
    }

    #[test]
    fn test_from_encrypted_decryption_failure() {
        let invalid_encrypted = vec![1, 2, 3, 4, 5];
        let (_dir, path) = setup_test_dir();

        let result = EncryptedDB::from_encrypted(
            invalid_encrypted,
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        );

        assert!(result.is_err(), "Should fail to decrypt invalid data");
        if let Err(e) = result {
            assert!(e.to_string().contains("Could not decrypt"));
        }
    }

    #[test]
    fn test_from_file_success() {
        let data = b"File data".to_vec();
        let pwd = "file_pwd".to_string();
        let salt = "file_salt".to_string();
        let (_dir, path) = setup_test_dir();

        // Write encrypted data to file
        let encrypted = crypto::encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        fs::write(&path, encrypted).unwrap();

        // Load from file
        let edb = EncryptedDB::from_file(path.to_str().unwrap().to_string(), pwd, salt).unwrap();

        assert_eq!(edb.decrypted(), data);
    }

    #[test]
    fn test_from_file_not_exists() {
        let path = "/nonexistent/path/to/file.bin";

        let result =
            EncryptedDB::from_file(path.to_string(), "pwd".to_string(), "salt".to_string());

        assert!(result.is_err(), "Should fail when file doesn't exist");
    }

    #[test]
    fn test_new_with_decrypted() {
        let data = b"New decrypted".to_vec();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::new(
            None,
            Some(data.clone()),
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
        assert!(!edb.bytes().is_empty());
    }

    #[test]
    fn test_new_with_encrypted() {
        let data = b"Original".to_vec();
        let pwd = "pwd".to_string();
        let salt = "salt".to_string();
        let encrypted = crypto::encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::new(
            Some(encrypted.clone()),
            None,
            path.to_str().unwrap().to_string(),
            pwd,
            salt,
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
        assert_eq!(edb.bytes(), encrypted);
    }

    #[test]
    fn test_new_from_file() {
        let data = b"File content".to_vec();
        let pwd = "pwd".to_string();
        let salt = "salt".to_string();
        let (_dir, path) = setup_test_dir();

        // Prepare file
        let encrypted = crypto::encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        fs::write(&path, encrypted).unwrap();

        // Create from file
        let edb =
            EncryptedDB::new(None, None, path.to_str().unwrap().to_string(), pwd, salt).unwrap();

        assert_eq!(edb.decrypted(), data);
    }

    #[test]
    fn test_bytes_getter() {
        let data = b"Data".to_vec();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data,
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        let bytes1 = edb.bytes();
        let bytes2 = edb.bytes();
        assert_eq!(bytes1, bytes2, "bytes() should return same value");
        assert!(!bytes1.is_empty());
    }

    #[test]
    fn test_decrypted_getter() {
        let data = b"Decrypted data".to_vec();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data.clone(),
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.decrypted(), data);
    }

    #[test]
    fn test_path_getter() {
        let data = b"Data".to_vec();
        let (_dir, path) = setup_test_dir();
        let path_str = path.to_str().unwrap().to_string();

        let edb = EncryptedDB::from_decrypted(
            data,
            path_str.clone(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.path(), path_str);
    }

    #[test]
    fn test_pwd_getter() {
        let data = b"Data".to_vec();
        let pwd = "my_password".to_string();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data,
            path.to_str().unwrap().to_string(),
            pwd.clone(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.pwd(), pwd);
    }

    #[test]
    fn test_salt_getter() {
        let data = b"Data".to_vec();
        let salt = "my_salt".to_string();
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data,
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            salt.clone(),
        )
        .unwrap();

        assert_eq!(edb.salt(), salt);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = b"Roundtrip data".to_vec();
        let pwd = "pwd".to_string();
        let salt = "salt".to_string();
        let (_dir, path) = setup_test_dir();

        let mut edb = EncryptedDB::from_decrypted(
            original.clone(),
            path.to_str().unwrap().to_string(),
            pwd.clone(),
            salt.clone(),
        )
        .unwrap();

        let encrypted = edb.bytes();
        assert_ne!(encrypted, original);

        // Manually decrypt
        edb.decrypt().unwrap();
        assert_eq!(edb.decrypted(), original);

        // Re-encrypt
        edb.encrypt().unwrap();
        assert_ne!(edb.bytes(), original);
    }

    #[test]
    fn test_write_and_read() {
        let data = b"Write test".to_vec();
        let pwd = "pwd".to_string();
        let salt = "salt".to_string();
        let (_dir, path) = setup_test_dir();
        let path_str = path.to_str().unwrap().to_string();

        // Create and write
        let edb =
            EncryptedDB::from_decrypted(data.clone(), path_str.clone(), pwd.clone(), salt.clone())
                .unwrap();

        edb.write().unwrap();

        // Verify file exists
        assert!(path.exists(), "File should exist after write");

        // Read back
        let edb2 = EncryptedDB::from_file(path_str, pwd, salt).unwrap();
        assert_eq!(edb2.decrypted(), data);
    }

    #[test]
    fn test_write_invalid_path() {
        let data = b"Data".to_vec();
        let (_dir, _path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            data,
            "/invalid/path/that/does/not/exist/file.bin".to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        let result = edb.write();
        assert!(result.is_err(), "Should fail to write to invalid path");
    }

    #[test]
    fn test_read_updates_bytes() {
        let data = b"Read test".to_vec();
        let pwd = "pwd".to_string();
        let salt = "salt".to_string();
        let (_dir, path) = setup_test_dir();
        let path_str = path.to_str().unwrap().to_string();

        // Write encrypted data to file
        let encrypted = crypto::encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        fs::write(&path, encrypted.clone()).unwrap();

        // Create empty EncryptedDB and read
        let mut edb = EncryptedDB {
            bytes: Vec::new(),
            decrypted: Secret::new(Vec::new()),
            path: path_str,
            pwd: SecretString::new(pwd),
            salt: SecretString::new(salt),
        };

        assert_eq!(edb.bytes().len(), 0, "Should start empty");
        edb.read().unwrap();
        assert_eq!(edb.bytes(), encrypted, "Should match file contents");
    }

    #[test]
    fn test_decrypt_error_handling() {
        let (_dir, path) = setup_test_dir();

        let mut edb = EncryptedDB {
            bytes: vec![1, 2, 3], // Invalid encrypted data
            decrypted: Secret::new(Vec::new()),
            path: path.to_str().unwrap().to_string(),
            pwd: SecretString::new("pwd".to_string()),
            salt: SecretString::new("salt".to_string()),
        };

        let result = edb.decrypt();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Could not decrypt"));
        }
    }

    #[test]
    fn test_multiple_encrypt_decrypt_cycles() {
        let original = b"Cycle test".to_vec();
        let (_dir, path) = setup_test_dir();

        let mut edb = EncryptedDB::from_decrypted(
            original.clone(),
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        // Cycle 1
        edb.encrypt().unwrap();
        edb.decrypt().unwrap();
        assert_eq!(edb.decrypted(), original);

        // Cycle 2
        edb.encrypt().unwrap();
        edb.decrypt().unwrap();
        assert_eq!(edb.decrypted(), original);
    }

    #[test]
    fn test_large_data() {
        let large_data = vec![42u8; 10000];
        let (_dir, path) = setup_test_dir();

        let edb = EncryptedDB::from_decrypted(
            large_data.clone(),
            path.to_str().unwrap().to_string(),
            "pwd".to_string(),
            "salt".to_string(),
        )
        .unwrap();

        assert_eq!(edb.decrypted(), large_data);
        assert!(
            edb.bytes().len() > large_data.len(),
            "Encrypted should be larger"
        );
    }
}
