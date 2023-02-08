use std::path;

use rucksack_db as store;

use rucksack_lib::{config, util};

#[derive(Debug)]
pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

pub fn new(cfg: config::Config, db: store::db::DB) -> App {
    App { cfg, db }
}

impl App {
    pub fn config_dir(&self) -> path::PathBuf {
        if self.cfg.rucksack.cfg_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.cfg_dir.clone());
            return path;
        }
        util::config_dir()
    }

    pub fn config_file(&self) -> String {
        if self.cfg.rucksack.cfg_file != *"" {
            return self.cfg.rucksack.cfg_file.clone();
        }
        util::config_file()
    }

    pub fn data_dir(&self) -> path::PathBuf {
        if self.cfg.rucksack.data_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.data_dir.clone());
            return path;
        }
        util::data_dir()
    }

    pub fn db_file(&self) -> String {
        if self.cfg.rucksack.db_file != *"" {
            return self.cfg.rucksack.db_file.clone();
        }
        util::db_file()
    }

    pub fn db_version(&self) -> versions::SemVer {
        self.db.version()
    }
}
