use std::{fs, path};

use anyhow::{anyhow, Result};

use rucksack_lib::{file, time};

pub fn copy(src_file: String, dest_dir: String, version: String) -> Result<String> {
    let file_path = file::abs_path(src_file.clone())?;
    let mut bu_path = file::abs_path(dest_dir)?;
    file::create_dirs(bu_path.clone())?;
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

pub fn latest(backup_dir: String) -> Result<file::Data> {
    match list(backup_dir) {
        Ok(all) => match all.first() {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("no backup files found")),
        },
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn list(backup_dir: String) -> Result<file::Listing> {
    let mut backups = file::files(backup_dir)?;
    backups.sort();
    backups.reverse();
    Ok(backups)
}

pub fn restore(
    backup_path: path::PathBuf,
    old_name: String,
    dest_path: path::PathBuf,
) -> Result<()> {
    let mut old_path = backup_path;
    old_path.push(old_name.clone());
    log::debug!(
        "Restoring backup from {} to {} ...",
        old_path.display(),
        dest_path.display()
    );
    let old_file = old_path.display().to_string();
    // let dest_parent = dest_path.parent().unwrap();
    match fs::copy(old_path, dest_path) {
        Ok(_) => (),
        Err(e) => {
            let msg = "Could not copy file";
            log::error!("{msg} {old_file:?} ({e:})");
            return Err(anyhow!("{msg} {old_file:?} ({e:})"));
        }
    }
    // let mut copied = dest_parent.to_owned();
    // copied.push(old_name);
    // fs::rename(copied, dest_path)?;
    Ok(())
}
