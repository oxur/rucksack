use std::path;

use rucksack_db as store;
use rucksack_lib::file;

use crate::input::config;

#[derive(Debug)]
pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

pub fn new(cfg: config::Config, db: store::db::DB) -> App {
    App { cfg, db }
}

impl App {
    pub fn backup_dir(&self) -> String {
        self.db.backup_dir()
    }

    pub fn backup_path(&self) -> path::PathBuf {
        let mut path = path::PathBuf::new();
        path.push(self.backup_dir());
        path
    }

    pub fn config_dir(&self) -> String {
        self.config_path().display().to_string()
    }

    pub fn config_path(&self) -> path::PathBuf {
        if self.cfg.rucksack.cfg_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.cfg_dir.clone());
            return path;
        }
        file::config_dir(&self.name())
    }

    pub fn config_file(&self) -> String {
        if self.cfg.rucksack.cfg_file != *"" {
            return self.cfg.rucksack.cfg_file.clone();
        }
        file::config_file(&self.name())
    }

    pub fn data_dir(&self) -> String {
        self.data_path().display().to_string()
    }

    pub fn data_path(&self) -> path::PathBuf {
        self.db_path().parent().unwrap().to_path_buf()
    }

    pub fn db_file(&self) -> String {
        self.db.file_name.clone()
    }

    pub fn db_path(&self) -> path::PathBuf {
        let mut path = path::PathBuf::new();
        path.push(self.db_file());
        path
    }

    pub fn db_version(&self) -> versions::SemVer {
        self.db.version()
    }

    pub fn name(&self) -> String {
        self.cfg.rucksack.name.clone()
    }
}
