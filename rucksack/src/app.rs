use std::path;

use rucksack_db as store;
use rucksack_lib::{config, file};

#[derive(Debug)]
pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

pub fn new(cfg: config::Config, db: store::db::DB) -> App {
    App { cfg, db }
}

impl App {
    pub fn backup_dir(&self) -> path::PathBuf {
        if self.cfg.rucksack.backup_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.backup_dir.clone());
            return path;
        }
        file::backup_dir(&self.name())
    }

    pub fn config_dir(&self) -> path::PathBuf {
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

    pub fn data_dir(&self) -> path::PathBuf {
        if self.cfg.rucksack.data_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.data_dir.clone());
            return path;
        }
        file::data_dir(&self.name())
    }

    pub fn db_file(&self) -> String {
        if self.cfg.rucksack.db_file != *"" {
            return self.cfg.rucksack.db_file.clone();
        }
        file::db_file(&self.name())
    }

    pub fn db_version(&self) -> versions::SemVer {
        self.db.version()
    }

    pub fn name(&self) -> String {
        self.cfg.rucksack.name.clone()
    }
}
