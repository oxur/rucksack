use std::path;

use crate::{config, store, util};

#[derive(Debug)]
pub struct App {
    pub cfg: config::Config,
    pub db: store::db::DB,
}

impl App {
    pub fn config_dir(&self) -> path::PathBuf {
        if self.cfg.rucksack.directory != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.directory.clone());
            return path;
        }
        util::default_config_dir()
    }

    pub fn config_file(&self) -> String {
        if self.cfg.rucksack.file != *"" {
            return self.cfg.rucksack.file.clone();
        }
        util::default_config_file()
    }
}
