use std::fs;
use std::io::Write;

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
