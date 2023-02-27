use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs, io, path};

use anyhow::{anyhow, Result};
use chrono::offset::Local;
use chrono::DateTime;
use path_clean::PathClean;

use crate::time;

const DATA_DIR: &str = "data";
const BACKUP_DIR: &str = "backups";
const DEFAULT_DB_NAME: &str = "secrets";
const DB_EXTENSION: &str = "db";

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

pub fn backup_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push(project);
    path.push(BACKUP_DIR);
    path
}

pub fn config_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push(project);
    path
}

pub fn config_file(project: &str) -> String {
    let mut path = config_dir(project);
    path.push("config");
    path.set_extension("toml");
    path.to_str().unwrap().to_string()
}

pub fn create_parents(path: String) -> Result<path::PathBuf> {
    // Make sure the path is created
    log::debug!("Attempting to create parent directory of {path} ...");
    let ap = abs_path(path)?;
    log::debug!("Attempting to create directory {:}", ap.display());
    let parent: path::PathBuf = path::PathBuf::from(ap.parent().unwrap());
    create_dirs(parent)?;
    Ok(ap)
}

pub fn create_dirs(path: path::PathBuf) -> Result<path::PathBuf> {
    let path_name = path.display();
    match fs::create_dir_all(path.clone()) {
        Ok(_) => Ok(path),
        Err(e) => {
            let msg = "Could not create missing parent dirs for";
            log::error!("{msg} {path_name} ({e:})");
            Err(anyhow!("{} {} ({:})", msg, path_name, e))
        }
    }
}

pub fn data_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push(project);
    path.push(DATA_DIR);
    path
}

pub fn db_file(project: &str) -> String {
    let mut path = data_dir(project);
    path.push(DEFAULT_DB_NAME);
    path.set_extension(DB_EXTENSION);
    path.to_str().unwrap().to_string()
}

pub fn dir_parent(dir: String) -> String {
    let mut parent: Vec<&str> = dir.split(std::path::MAIN_SEPARATOR).collect();
    parent.pop();
    parent.join(std::path::MAIN_SEPARATOR.to_string().as_str())
}

pub fn expanded_path(path: String) -> String {
    let expanded = shellexpand::tilde(path.as_str());
    expanded.to_string()
}

pub fn files(path: String) -> Result<Vec<(String, String, String)>> {
    let mut f = Vec::<(String, String, String)>::new();
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let metadata = dir.metadata()?;
        let created: DateTime<Local> = metadata.created()?.into();
        f.push((
            dir.file_name().to_str().unwrap().to_owned(),
            time::format_datetime(created),
            unix_mode::to_string(metadata.permissions().mode()),
        ));
    }
    Ok(f)
}

pub fn read(path: String) -> Result<Vec<u8>> {
    let expanded = expanded_path(path);
    log::debug!("Reading file {:?} ...", expanded);
    match fs::read(expanded) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn write(data: Vec<u8>, path: String) -> Result<()> {
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
