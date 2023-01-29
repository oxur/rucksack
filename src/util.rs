use std::io::Write;
use std::{env, fs, io, path};

use anyhow::{anyhow, Result};
use path_clean::PathClean;
use rand::Rng;
use versions::Versioning;

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

pub fn default_config_dir() -> path::PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push(env!("CARGO_PKG_NAME"));
    path
}

pub fn default_config_file() -> String {
    let mut path = default_config_dir();
    path.push("config");
    path.set_extension("toml");
    path.to_str().unwrap().to_string()
}

pub type BincodeConfig = bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::WriteFixedArrayLength,
    bincode::config::NoLimit,
>;

pub fn bincode_cfg() -> BincodeConfig {
    bincode::config::legacy()
}

pub fn dir_parent(dir: String) -> String {
    let mut parent: Vec<&str> = dir.split(std::path::MAIN_SEPARATOR).collect();
    parent.pop();
    parent.join(std::path::MAIN_SEPARATOR.to_string().as_str())
}

pub fn version() -> Versioning {
    versions::Versioning::new(env!("CARGO_PKG_VERSION")).unwrap()
}

pub fn expanded_path(path: String) -> String {
    let expanded = shellexpand::tilde(path.as_str());
    expanded.to_string()
}

pub fn abs_path(path: String) -> io::Result<path::PathBuf> {
    let expanded = expanded_path(path);
    let path = std::path::Path::new(expanded.as_str());
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };
    absolute_path.clean();
    Ok(absolute_path)
}

pub fn create_parents(path: String) -> Result<path::PathBuf> {
    // Make sure the path is created
    let ap = abs_path(path.clone())?;
    match fs::create_dir_all(ap.parent().unwrap()) {
        Ok(_) => Ok(ap),
        Err(e) => {
            let msg = "Could not create missing parent dirs for";
            Err(anyhow!("{} {} ({:})", msg, path, e))
        }
    }
}
