use std::cmp::Ordering;

use anyhow::{anyhow, Result};

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

const NONCE_SIZE: usize = 12;

pub enum KeySize {
    Bit128,
    Bit256,
}

pub fn encrypt(data: Vec<u8>, pwd: String, salt: String) -> Result<Vec<u8>> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(salt);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    cipher
        .encrypt(nonce, &data[..])
        .map_err(|e| anyhow!("encryption failed: {}", e))
}

pub fn decrypt(encrypted: Vec<u8>, pwd: String, salt: String) -> Result<Vec<u8>> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(salt);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    match cipher.decrypt(nonce, &encrypted[..]) {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!(e)),
    }
}

fn sized_key(source: String, key_size: KeySize) -> Vec<u8> {
    let size: usize = match key_size {
        KeySize::Bit128 => 16,
        KeySize::Bit256 => 32,
    };
    let mut bytes = source.as_bytes().to_vec();
    match bytes.len().cmp(&size) {
        Ordering::Equal => bytes,
        _ => {
            bytes.resize(size, 0x00);
            bytes
        }
    }
}

fn sized_nonce(source: String) -> Vec<u8> {
    let mut bytes = source.as_bytes().to_vec();
    bytes.resize(NONCE_SIZE, 0x00);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let data = b"Hello, World!".to_vec();
        let pwd = "test_password".to_string();
        let salt = "test_salt".to_string();

        let encrypted = encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        assert_ne!(
            data, encrypted,
            "Encrypted data should differ from original"
        );

        let decrypted = decrypt(encrypted, pwd, salt).unwrap();
        assert_eq!(data, decrypted, "Decrypted data should match original");
    }

    #[test]
    fn test_encrypt_decrypt_empty_data() {
        let data = Vec::new();
        let pwd = "password".to_string();
        let salt = "salt".to_string();

        let encrypted = encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        let decrypted = decrypt(encrypted, pwd, salt).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_large_data() {
        let data = vec![42u8; 10000];
        let pwd = "strong_password".to_string();
        let salt = "unique_salt".to_string();

        let encrypted = encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        let decrypted = decrypt(encrypted, pwd, salt).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_decrypt_with_wrong_password() {
        let data = b"Secret data".to_vec();
        let pwd = "correct_password".to_string();
        let salt = "salt".to_string();

        let encrypted = encrypt(data, pwd, salt.clone()).unwrap();

        let wrong_pwd = "wrong_password".to_string();
        let result = decrypt(encrypted, wrong_pwd, salt);
        assert!(
            result.is_err(),
            "Decryption with wrong password should fail"
        );
    }

    #[test]
    fn test_decrypt_with_wrong_salt() {
        let data = b"Secret data".to_vec();
        let pwd = "password".to_string();
        let salt = "correct_salt".to_string();

        let encrypted = encrypt(data, pwd.clone(), salt).unwrap();

        let wrong_salt = "wrong_salt".to_string();
        let result = decrypt(encrypted, pwd, wrong_salt);
        assert!(result.is_err(), "Decryption with wrong salt should fail");
    }

    #[test]
    fn test_decrypt_corrupted_data() {
        let data = b"Some data".to_vec();
        let pwd = "password".to_string();
        let salt = "salt".to_string();

        let mut encrypted = encrypt(data, pwd.clone(), salt.clone()).unwrap();

        // Corrupt the encrypted data
        if !encrypted.is_empty() {
            encrypted[0] ^= 0xFF;
        }

        let result = decrypt(encrypted, pwd, salt);
        assert!(result.is_err(), "Decryption of corrupted data should fail");
    }

    #[test]
    fn test_sized_key_bit256_short() {
        let source = "short".to_string();
        let key = sized_key(source, KeySize::Bit256);
        assert_eq!(key.len(), 32, "Bit256 key should be 32 bytes");
        assert_eq!(&key[0..5], b"short");
        // Verify padding with zeros
        assert_eq!(&key[5..], &[0u8; 27]);
    }

    #[test]
    fn test_sized_key_bit256_exact() {
        let source = "a".repeat(32);
        let key = sized_key(source.clone(), KeySize::Bit256);
        assert_eq!(key.len(), 32);
        assert_eq!(key, source.as_bytes());
    }

    #[test]
    fn test_sized_key_bit256_long() {
        let source = "a".repeat(50);
        let key = sized_key(source, KeySize::Bit256);
        assert_eq!(key.len(), 32, "Bit256 key should be truncated to 32 bytes");
    }

    #[test]
    fn test_sized_key_bit128_short() {
        let source = "test".to_string();
        let key = sized_key(source, KeySize::Bit128);
        assert_eq!(key.len(), 16, "Bit128 key should be 16 bytes");
        assert_eq!(&key[0..4], b"test");
        assert_eq!(&key[4..], &[0u8; 12]);
    }

    #[test]
    fn test_sized_key_bit128_exact() {
        let source = "b".repeat(16);
        let key = sized_key(source.clone(), KeySize::Bit128);
        assert_eq!(key.len(), 16);
        assert_eq!(key, source.as_bytes());
    }

    #[test]
    fn test_sized_key_bit128_long() {
        let source = "c".repeat(30);
        let key = sized_key(source, KeySize::Bit128);
        assert_eq!(key.len(), 16, "Bit128 key should be truncated to 16 bytes");
    }

    #[test]
    fn test_sized_nonce_short() {
        let source = "abc".to_string();
        let nonce = sized_nonce(source);
        assert_eq!(nonce.len(), NONCE_SIZE);
        assert_eq!(&nonce[0..3], b"abc");
        assert_eq!(&nonce[3..], &[0u8; 9]);
    }

    #[test]
    fn test_sized_nonce_exact() {
        let source = "x".repeat(NONCE_SIZE);
        let nonce = sized_nonce(source.clone());
        assert_eq!(nonce.len(), NONCE_SIZE);
        assert_eq!(nonce, source.as_bytes());
    }

    #[test]
    fn test_sized_nonce_long() {
        let source = "y".repeat(20);
        let nonce = sized_nonce(source);
        assert_eq!(
            nonce.len(),
            NONCE_SIZE,
            "Nonce should be truncated to 12 bytes"
        );
    }

    #[test]
    fn test_encrypt_deterministic_with_same_inputs() {
        let data = b"Test data".to_vec();
        let pwd = "password".to_string();
        let salt = "salt".to_string();

        let encrypted1 = encrypt(data.clone(), pwd.clone(), salt.clone()).unwrap();
        let encrypted2 = encrypt(data, pwd, salt).unwrap();

        assert_eq!(
            encrypted1, encrypted2,
            "Encryption should be deterministic with same inputs"
        );
    }

    #[test]
    fn test_encrypt_different_with_different_salt() {
        let data = b"Test data".to_vec();
        let pwd = "password".to_string();

        let encrypted1 = encrypt(data.clone(), pwd.clone(), "salt1".to_string()).unwrap();
        let encrypted2 = encrypt(data, pwd, "salt2".to_string()).unwrap();

        assert_ne!(
            encrypted1, encrypted2,
            "Encryption should differ with different salts"
        );
    }

    #[test]
    fn test_encrypt_different_with_different_password() {
        let data = b"Test data".to_vec();
        let salt = "salt".to_string();

        let encrypted1 = encrypt(data.clone(), "password1".to_string(), salt.clone()).unwrap();
        let encrypted2 = encrypt(data, "password2".to_string(), salt).unwrap();

        assert_ne!(
            encrypted1, encrypted2,
            "Encryption should differ with different passwords"
        );
    }
}
