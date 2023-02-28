use std::fs;

use anyhow::{anyhow, Result};

use rucksack_lib::{file, time};

pub fn copy(src_file: String, dest_dir: String, version: String) -> Result<String> {
    let file_path = file::abs_path(src_file.clone())?;
    let mut bu_path = file::abs_path(dest_dir)?;
    bu_path.push(backup_name(
        file_path.file_name().unwrap().to_str().unwrap().to_string(),
        version,
    ));
    match fs::copy(src_file.clone(), bu_path.clone()) {
        Ok(_) => Ok(bu_path.display().to_string()),
        Err(e) => {
            let msg = "Could not copy file";
            log::error!("{msg} {src_file:?} ({e:})");
            Err(anyhow!("{msg} {src_file:?} ({e:})"))
        }
    }
}

pub fn backup_name(src_file: String, version: String) -> String {
    format!("{src_file}-{}-v{version}", time::simple_timestamp())
}
