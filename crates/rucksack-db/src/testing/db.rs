use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};

use rucksack_lib::file;

#[derive(Clone, Debug, Default)]
pub struct TempDB {
    backups_path: PathBuf,
    pub base: PathBuf,
    data_path: PathBuf,
    file_name: String,
    file_path: PathBuf,
}

pub fn new() -> TempDB {
    TempDB {
        base: tempfile::tempdir().unwrap().path().to_owned(),

        ..Default::default()
    }
}

impl TempDB {
    pub fn backups_path(&mut self) -> Result<PathBuf> {
        if !self.backups_path.as_os_str().is_empty() {
            return Ok(self.backups_path.clone());
        };
        self.backups_path = self.base.clone();
        self.backups_path.push("backups");
        file::create_dirs(self.backups_path.clone())?;
        Ok(self.backups_path.clone())
    }

    pub fn data_path(&mut self) -> Result<PathBuf> {
        if !self.data_path.as_os_str().is_empty() {
            return Ok(self.data_path.clone());
        };
        self.data_path = self.base.clone();
        self.data_path.push("data");
        file::create_dirs(self.data_path.clone())?;
        Ok(self.data_path.clone())
    }

    pub fn file_path(&mut self) -> Result<PathBuf> {
        if !self.file_path.as_os_str().is_empty() {
            return Ok(self.file_path.clone());
        };
        self.file_path = self.data_path()?;
        self.file_path.push("secrets");
        self.file_path.with_extension("db");
        Ok(self.file_path.clone())
    }

    pub fn file_name(&mut self) -> Result<String> {
        if !self.file_name.is_empty() {
            return Ok(self.file_name.clone());
        };
        let f = self.file_path()?;
        self.file_name = f.display().to_string();
        Ok(self.file_name.clone())
    }

    pub fn setup(&mut self) -> Result<()> {
        let _ = self.data_path()?;
        let _ = self.backups_path()?;
        let _ = self.file_path()?;
        Ok(())
    }

    pub fn teardown(&self) -> Result<()> {
        match fs::remove_dir_all(self.base.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let temp_db = new();
        // Just check that base path is set, not that it exists
        // (tempdir is dropped immediately so path doesn't exist)
        assert!(!temp_db.base.as_os_str().is_empty());
    }

    #[test]
    fn test_default() {
        let temp_db = TempDB::default();
        assert!(temp_db.file_name.is_empty());
        assert!(temp_db.data_path.as_os_str().is_empty());
    }

    #[test]
    fn test_backups_path() {
        let mut temp_db = new();
        let backups_path = temp_db.backups_path().unwrap();
        assert!(backups_path.exists());
        assert!(backups_path.ends_with("backups"));

        // Calling again should return cached path
        let backups_path2 = temp_db.backups_path().unwrap();
        assert_eq!(backups_path, backups_path2);
    }

    #[test]
    fn test_data_path() {
        let mut temp_db = new();
        let data_path = temp_db.data_path().unwrap();
        assert!(data_path.exists());
        assert!(data_path.ends_with("data"));

        // Calling again should return cached path
        let data_path2 = temp_db.data_path().unwrap();
        assert_eq!(data_path, data_path2);
    }

    #[test]
    fn test_file_path() {
        let mut temp_db = new();
        let file_path = temp_db.file_path().unwrap();
        assert!(file_path.to_string_lossy().contains("secrets"));

        // Calling again should return cached path
        let file_path2 = temp_db.file_path().unwrap();
        assert_eq!(file_path, file_path2);
    }

    #[test]
    fn test_file_name() {
        let mut temp_db = new();
        let file_name = temp_db.file_name().unwrap();
        assert!(!file_name.is_empty());
        assert!(file_name.contains("secrets"));

        // Calling again should return cached name
        let file_name2 = temp_db.file_name().unwrap();
        assert_eq!(file_name, file_name2);
    }

    #[test]
    fn test_setup() {
        let mut temp_db = new();
        let result = temp_db.setup();
        assert!(result.is_ok());
        assert!(temp_db.data_path.exists());
        assert!(temp_db.backups_path.exists());
    }

    #[test]
    fn test_teardown() {
        let mut temp_db = new();
        let base_path = temp_db.base.clone();
        let _ = temp_db.setup();
        assert!(base_path.exists());

        let result = temp_db.teardown();
        assert!(result.is_ok());
        assert!(!base_path.exists());
    }

    #[test]
    fn test_clone() {
        let temp_db1 = new();
        let temp_db2 = temp_db1.clone();
        assert_eq!(temp_db1.base, temp_db2.base);
    }
}
