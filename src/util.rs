use std::io::Write;
use std::{fs, path};

use anyhow::{anyhow, Result};
use rand::Rng;

const SPECIALS: &[u8] = b"!@#%&*?=+:";

pub fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

pub fn random_specials(count: usize) -> Vec<u8> {
    let mut specials: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..count + 1 {
        specials.push(SPECIALS[rng.gen_range(0..SPECIALS.len())])
    }
    specials
}

pub fn read_file(path: String) -> Result<Vec<u8>> {
    let expanded = shellexpand::tilde(path.as_str());
    match fs::read(expanded.as_ref()) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn write_file(data: Vec<u8>, path: String) -> Result<()> {
    let expanded = shellexpand::tilde(path.as_str());
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(expanded.as_ref())
        .unwrap();
    file.write_all(&data[..])?;
    match file.sync_all() {
        Ok(x) => Ok(x),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn config_dir() -> path::PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push(env!("CARGO_PKG_NAME"));
    path
}

pub fn config_file() -> String {
    let mut path = config_dir();
    path.push("config");
    path.set_extension("toml");
    path.to_str().unwrap().to_string()
}

pub fn data_dir() -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push(env!("CARGO_PKG_NAME"));
    path.push("data");
    path
}

pub fn db_file() -> String {
    let mut path = data_dir();
    path.push("secrets");
    path.set_extension("db");
    path.to_str().unwrap().to_string()
}

pub fn dir_parent(dir: String) -> String {
    let mut parent: Vec<&str> = dir.split(std::path::MAIN_SEPARATOR).collect();
    parent.pop();
    parent.join(std::path::MAIN_SEPARATOR.to_string().as_str())
}
