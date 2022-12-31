use std::fs;
use std::io::Error;

use bincode::config;
use dashmap::DashMap;

use super::crypto::{decrypt, encrypt};
use super::record::EncryptedRecord;

pub fn init(path: String, store_pwd: String, updated: String) -> DashMap<String, EncryptedRecord> {
    let map: DashMap<String, EncryptedRecord> = DashMap::new();
    write(map, path.clone(), store_pwd.clone(), updated.clone()).unwrap();
    read(path, store_pwd, updated)
}

pub fn write(
    map: DashMap<String, EncryptedRecord>,
    path: String,
    store_pwd: String,
    updated: String,
) -> Result<(), Error> {
    let encoded = bincode::serde::encode_to_vec(map, config::standard()).unwrap();
    let encrypted = encrypt(encoded, store_pwd, updated);
    fs::write(path, encrypted)
}

pub fn read(path: String, store_pwd: String, updated: String) -> DashMap<String, EncryptedRecord> {
    let encrypted = fs::read(path).unwrap();
    let decrypted = decrypt(encrypted, store_pwd, updated);
    let (decoded, _len) =
        bincode::serde::decode_from_slice(decrypted.as_ref(), config::standard()).unwrap();
    decoded
}

pub fn open(path: String, store_pwd: String, updated: String) -> DashMap<String, EncryptedRecord> {
    read(path, store_pwd, updated)
}
