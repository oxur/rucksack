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
    old_path.push(old_name);
    log::debug!(
        "Restoring backup from {} to {} ...",
        old_path.display(),
        dest_path.display()
    );
    let old_file = old_path.display().to_string();
    match fs::copy(old_path, dest_path) {
        Ok(_) => (),
        Err(e) => {
            let msg = "Could not copy file";
            log::error!("{msg} {old_file:?} ({e:})");
            return Err(anyhow!("{msg} {old_file:?} ({e:})"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_backup_name() {
        let name = backup_name("test.db".to_string(), "1.0.0".to_string());
        assert!(name.starts_with("test.db-"));
        assert!(name.ends_with("-v1.0.0"));
    }

    #[test]
    fn test_backup_name_with_path() {
        let name = backup_name("data.db".to_string(), "2.5.3".to_string());
        assert!(name.contains("data.db"));
        assert!(name.contains("v2.5.3"));
    }

    #[test]
    fn test_copy_success() {
        let mut src_file = NamedTempFile::new().unwrap();
        src_file.write_all(b"test data").unwrap();
        let src_path = src_file.path().to_str().unwrap().to_string();

        let dest_dir = tempdir().unwrap();
        let dest_path = dest_dir.path().to_str().unwrap().to_string();

        let result = copy(src_path, dest_path, "1.0.0".to_string());
        assert!(result.is_ok());
        let backup_file = result.unwrap();
        assert!(std::path::Path::new(&backup_file).exists());
    }

    #[test]
    fn test_copy_nonexistent_file() {
        let dest_dir = tempdir().unwrap();
        let result = copy(
            "/nonexistent/file.db".to_string(),
            dest_dir.path().to_str().unwrap().to_string(),
            "1.0.0".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_list_backups() {
        let backup_dir = tempdir().unwrap();
        let backup_path = backup_dir.path();

        // Create some backup files
        std::fs::write(backup_path.join("db-2024-01-01-v1.0.0"), b"data1").unwrap();
        std::fs::write(backup_path.join("db-2024-01-02-v1.0.1"), b"data2").unwrap();

        let result = list(backup_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert!(backups.len() >= 2);
    }

    #[test]
    fn test_list_empty_directory() {
        let backup_dir = tempdir().unwrap();
        let result = list(backup_dir.path().to_str().unwrap().to_string());
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert_eq!(backups.len(), 0);
    }

    #[test]
    fn test_latest_backup() {
        let backup_dir = tempdir().unwrap();
        let backup_path = backup_dir.path();

        std::fs::write(backup_path.join("db-2024-01-01-v1.0.0"), b"data1").unwrap();
        std::fs::write(backup_path.join("db-2024-01-02-v1.0.1"), b"data2").unwrap();

        let result = latest(backup_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_latest_no_backups() {
        let backup_dir = tempdir().unwrap();
        let result = latest(backup_dir.path().to_str().unwrap().to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_success() {
        let backup_dir = tempdir().unwrap();
        let backup_path = backup_dir.path();
        let backup_file = "test-backup.db";
        std::fs::write(backup_path.join(backup_file), b"backup data").unwrap();

        let dest_dir = tempdir().unwrap();
        let dest_path = dest_dir.path().join("restored.db");

        let result = restore(
            backup_path.to_path_buf(),
            backup_file.to_string(),
            dest_path.clone(),
        );
        assert!(result.is_ok());
        assert!(dest_path.exists());
        let contents = std::fs::read_to_string(dest_path).unwrap();
        assert_eq!(contents, "backup data");
    }

    #[test]
    fn test_restore_nonexistent_backup() {
        let backup_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();
        let dest_path = dest_dir.path().join("restored.db");

        let result = restore(
            backup_dir.path().to_path_buf(),
            "nonexistent.db".to_string(),
            dest_path,
        );
        assert!(result.is_err());
    }
}
