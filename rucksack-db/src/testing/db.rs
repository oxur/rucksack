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
