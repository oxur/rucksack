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
    let expanded = expanded_path(path);
    log::debug!("Reading file {:?} ...", expanded);
    match fs::read(expanded) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(anyhow!(e)),
    }
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
            log::error!("{} {} ({:})", msg, path, e);
            Err(anyhow!("{} {} ({:})", msg, path, e))
        }
    }
}

pub fn write_file(data: Vec<u8>, path: String) -> Result<()> {
    let ap = create_parents(path.clone())?;
    // Then write the file
    log::debug!("Writing file {:?} ...", ap);
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(ap)
    {
        Ok(x) => Ok(x),
        Err(e) => {
            let msg = "Could not set up file options for";
            log::error!("{} {} ({:})", msg, path, e);
            Err(anyhow!("{} {} ({:})", msg, path, e))
        }
    }?;
    file.write_all(&data[..])?;
    match file.sync_all() {
        Ok(x) => Ok(x),
        Err(e) => {
            let msg = "Could not write file";
            log::error!("{} {} ({:})", msg, path, e);
            Err(anyhow!("{} {} ({:})", msg, path, e))
        }
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

pub fn bincode_cfg() -> bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::WriteFixedArrayLength,
    bincode::config::NoLimit,
> {
    bincode::config::legacy()
}

pub fn version() -> Versioning {
    versions::Versioning::new(env!("CARGO_PKG_VERSION")).unwrap()
}
