use std::cmp::Ordering;

use anyhow::Result;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

const NONCE_SIZE: usize = 12;

pub enum KeySize {
    Bit128,
    Bit256,
}

pub fn encrypt(data: Vec<u8>, pwd: String, updated: String) -> Vec<u8> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(updated);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    cipher.encrypt(nonce, &data[..]).unwrap()
}

pub fn decrypt(encrypted: Vec<u8>, pwd: String, updated: String) -> Result<Vec<u8>> {
    let key_bytes = sized_key(pwd, KeySize::Bit256);
    let key = aead::Key::<Aes256Gcm>::from_slice(key_bytes.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = sized_nonce(updated);
    let nonce = Nonce::from_slice(nonce_bytes.as_ref());
    match cipher.decrypt(nonce, &encrypted[..]) {
        Ok(result) => Ok(result),
        Err(e) => Ok(e.to_string().into()),
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
