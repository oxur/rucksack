use std::cmp::Ordering;

use anyhow::{anyhow, Result};

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

/// Size of the AES-GCM nonce in bytes
const NONCE_SIZE: usize = 12;

/// Size of 128-bit AES key in bytes
const KEY_SIZE_128: usize = 16;

/// Size of 256-bit AES key in bytes
const KEY_SIZE_256: usize = 32;

#[non_exhaustive]
pub enum KeySize {
    Bit128,
    Bit256,
}

/// Encrypts data using AES-256-GCM.
///
/// This function uses AES-256-GCM (Galois/Counter Mode) for authenticated encryption,
/// providing both confidentiality and integrity. The password is used to derive a
/// 256-bit encryption key, and the salt is used as a nonce.
///
/// # Arguments
///
/// * `data` - The plaintext data to encrypt
/// * `pwd` - Password used to derive the encryption key (will be padded/truncated to 32 bytes)
/// * `salt` - Salt used as the nonce for encryption (will be padded/truncated to 12 bytes)
///
/// # Returns
///
/// Returns the encrypted data on success, or an error if encryption fails.
///
/// # Errors
///
/// Returns an error if:
/// - The encryption operation fails (e.g., data too large)
/// - The cipher initialization fails
///
/// # Security Considerations
///
/// - Uses AES-256-GCM for authenticated encryption
/// - Password is deterministically padded/truncated to 32 bytes
/// - Salt is deterministically padded/truncated to 12 bytes
/// - Same password and salt will always produce the same result for the same data
#[must_use = "encryption result must be checked - data may not be encrypted"]
pub fn encrypt(data: Vec<u8>, pwd: &str, salt: &str) -> Result<Vec<u8>> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(salt);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    cipher
        .encrypt(nonce, &data[..])
        .map_err(|e| anyhow!("encryption failed: {}", e))
}

/// Decrypts data that was encrypted using AES-256-GCM.
///
/// This function reverses the encryption performed by [`encrypt`], using the same
/// password and salt to derive the decryption key and nonce. The data is authenticated
/// during decryption, ensuring it hasn't been tampered with.
///
/// # Arguments
///
/// * `encrypted` - The encrypted data to decrypt
/// * `pwd` - Password used to derive the decryption key (must match the encryption password)
/// * `salt` - Salt used as the nonce (must match the encryption salt)
///
/// # Returns
///
/// Returns the decrypted plaintext data on success, or an error if decryption fails.
///
/// # Errors
///
/// Returns an error if:
/// - The password is incorrect
/// - The salt doesn't match the one used for encryption
/// - The encrypted data has been corrupted or tampered with
/// - The decryption operation fails for any other reason
///
/// # Examples
///
/// ```no_run
/// use rucksack_db::crypto;
///
/// # fn main() -> anyhow::Result<()> {
/// let data = b"secret message".to_vec();
/// let encrypted = crypto::encrypt(data.clone(), "password", "salt")?;
/// let decrypted = crypto::decrypt(encrypted, "password", "salt")?;
/// assert_eq!(data, decrypted);
/// # Ok(())
/// # }
/// ```
#[must_use = "decryption result must be checked - data may not be decrypted"]
pub fn decrypt(encrypted: Vec<u8>, pwd: &str, salt: &str) -> Result<Vec<u8>> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(salt);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    cipher.decrypt(nonce, &encrypted[..]).map_err(|_| {
        anyhow!("decryption failed - password may be incorrect, salt invalid, or data corrupted")
    })
}

fn sized_key(source: &str, key_size: KeySize) -> Vec<u8> {
    let size: usize = match key_size {
        KeySize::Bit128 => KEY_SIZE_128,
        KeySize::Bit256 => KEY_SIZE_256,
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

fn sized_nonce(source: &str) -> Vec<u8> {
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
        let pwd = "test_password";
        let salt = "test_salt";

        let encrypted = encrypt(data.clone(), pwd, salt).unwrap();
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
        let pwd = "password";
        let salt = "salt";

        let encrypted = encrypt(data.clone(), pwd, salt).unwrap();
        let decrypted = decrypt(encrypted, pwd, salt).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_large_data() {
        let data = vec![42u8; 10000];
        let pwd = "strong_password";
        let salt = "unique_salt";

        let encrypted = encrypt(data.clone(), pwd, salt).unwrap();
        let decrypted = decrypt(encrypted, pwd, salt).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_decrypt_with_wrong_password() {
        let data = b"Secret data".to_vec();
        let pwd = "correct_password";
        let salt = "salt";

        let encrypted = encrypt(data, pwd, salt).unwrap();

        let wrong_pwd = "wrong_password";
        let result = decrypt(encrypted, wrong_pwd, salt);
        assert!(
            result.is_err(),
            "Decryption with wrong password should fail"
        );
    }

    #[test]
    fn test_decrypt_with_wrong_salt() {
        let data = b"Secret data".to_vec();
        let pwd = "password";
        let salt = "correct_salt";

        let encrypted = encrypt(data, pwd, salt).unwrap();

        let wrong_salt = "wrong_salt";
        let result = decrypt(encrypted, pwd, wrong_salt);
        assert!(result.is_err(), "Decryption with wrong salt should fail");
    }

    #[test]
    fn test_decrypt_corrupted_data() {
        let data = b"Some data".to_vec();
        let pwd = "password";
        let salt = "salt";

        let mut encrypted = encrypt(data, pwd, salt).unwrap();

        // Corrupt the encrypted data
        if !encrypted.is_empty() {
            encrypted[0] ^= 0xFF;
        }

        let result = decrypt(encrypted, pwd, salt);
        assert!(result.is_err(), "Decryption of corrupted data should fail");
    }

    #[test]
    fn test_sized_key_bit256_short() {
        let source = "short";
        let key = sized_key(source, KeySize::Bit256);
        assert_eq!(key.len(), 32, "Bit256 key should be 32 bytes");
        assert_eq!(&key[0..5], b"short");
        // Verify padding with zeros
        assert_eq!(&key[5..], &[0u8; 27]);
    }

    #[test]
    fn test_sized_key_bit256_exact() {
        let source = "a".repeat(32);
        let key = sized_key(&source, KeySize::Bit256);
        assert_eq!(key.len(), 32);
        assert_eq!(key, source.as_bytes());
    }

    #[test]
    fn test_sized_key_bit256_long() {
        let source = "a".repeat(50);
        let key = sized_key(&source, KeySize::Bit256);
        assert_eq!(key.len(), 32, "Bit256 key should be truncated to 32 bytes");
    }

    #[test]
    fn test_sized_key_bit128_short() {
        let source = "test";
        let key = sized_key(source, KeySize::Bit128);
        assert_eq!(key.len(), 16, "Bit128 key should be 16 bytes");
        assert_eq!(&key[0..4], b"test");
        assert_eq!(&key[4..], &[0u8; 12]);
    }

    #[test]
    fn test_sized_key_bit128_exact() {
        let source = "b".repeat(16);
        let key = sized_key(&source, KeySize::Bit128);
        assert_eq!(key.len(), 16);
        assert_eq!(key, source.as_bytes());
    }

    #[test]
    fn test_sized_key_bit128_long() {
        let source = "c".repeat(30);
        let key = sized_key(&source, KeySize::Bit128);
        assert_eq!(key.len(), 16, "Bit128 key should be truncated to 16 bytes");
    }

    #[test]
    fn test_sized_nonce_short() {
        let source = "abc";
        let nonce = sized_nonce(source);
        assert_eq!(nonce.len(), NONCE_SIZE);
        assert_eq!(&nonce[0..3], b"abc");
        assert_eq!(&nonce[3..], &[0u8; 9]);
    }

    #[test]
    fn test_sized_nonce_exact() {
        let source = "x".repeat(NONCE_SIZE);
        let nonce = sized_nonce(&source);
        assert_eq!(nonce.len(), NONCE_SIZE);
        assert_eq!(nonce, source.as_bytes());
    }

    #[test]
    fn test_sized_nonce_long() {
        let source = "y".repeat(20);
        let nonce = sized_nonce(&source);
        assert_eq!(
            nonce.len(),
            NONCE_SIZE,
            "Nonce should be truncated to 12 bytes"
        );
    }

    #[test]
    fn test_encrypt_deterministic_with_same_inputs() {
        let data = b"Test data".to_vec();
        let pwd = "password";
        let salt = "salt";

        let encrypted1 = encrypt(data.clone(), pwd, salt).unwrap();
        let encrypted2 = encrypt(data, pwd, salt).unwrap();

        assert_eq!(
            encrypted1, encrypted2,
            "Encryption should be deterministic with same inputs"
        );
    }

    #[test]
    fn test_encrypt_different_with_different_salt() {
        let data = b"Test data".to_vec();
        let pwd = "password";

        let encrypted1 = encrypt(data.clone(), pwd, "salt1").unwrap();
        let encrypted2 = encrypt(data, pwd, "salt2").unwrap();

        assert_ne!(
            encrypted1, encrypted2,
            "Encryption should differ with different salts"
        );
    }

    #[test]
    fn test_encrypt_different_with_different_password() {
        let data = b"Test data".to_vec();
        let salt = "salt";

        let encrypted1 = encrypt(data.clone(), "password1", salt).unwrap();
        let encrypted2 = encrypt(data, "password2", salt).unwrap();

        assert_ne!(
            encrypted1, encrypted2,
            "Encryption should differ with different passwords"
        );
    }
}
