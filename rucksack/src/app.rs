use std::path;

use anyhow::Result;
use clap::ArgMatches;
use secrecy::ExposeSecret;

use rucksack_db as store;
use rucksack_lib::file;

use crate::command;
use crate::input::{constant, options, Config};

#[derive(Debug)]
pub struct App {
    pub cfg: Config,
    pub db: store::db::DB,
}

impl App {
    pub fn new(cfg: Config, matches: &ArgMatches) -> Result<App> {
        log::debug!("Setting up rucksack application ...");
        let db = setup_db(matches)?;
        Ok(App { cfg, db })
    }

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

    pub fn config_file(&self) -> String {
        if self.cfg.rucksack.cfg_file != *"" {
            return self.cfg.rucksack.cfg_file.clone();
        }
        file::config_file(&self.name())
    }

    pub fn config_path(&self) -> path::PathBuf {
        if self.cfg.rucksack.cfg_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.cfg_dir.clone());
            return path;
        }
        file::config_dir(&self.name())
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

    pub fn run(&self, matches: &ArgMatches) -> Result<()> {
        log::info!("Executing rucksack command ...");
        if !self.backup_path().exists() {
            log::debug!("Checking for backup dir {:?} ...", self.backup_dir());
            file::create_dirs(self.backup_path())?;
            log::info!("Created backup dir.");
        }
        command::dispatch(self, matches)?;
        log::debug!("Command execution complete.");
        Ok(())
    }

    pub fn shutdown(&self, _matches: &ArgMatches) -> Result<()> {
        log::info!("Performing shutdown operations ...");
        if self.cfg.retention.purge_on_shutdown {
            todo!();
        }
        if self.cfg.retention.delete_inactive {
            // TODO: iterate through all inactive records and flag them as deleted
            todo!();
        }
        Ok(())
    }
}

pub fn setup_db(matches: &ArgMatches) -> Result<store::db::DB> {
    log::debug!("Setting up database ...");
    let db_file = match options::db(matches) {
        Some(file_path) => {
            log::debug!("Got database file from flag: {}", file_path);
            file_path
        }
        None => {
            let file_name = file::db_file(constant::NAME);
            log::debug!("No database flag provided; using default ({file_name:})");
            file_name
        }
    };
    let mut backup_dir = options::backup_dir(matches);
    if backup_dir.is_empty() {
        let dir_path = file::backup_dir(constant::NAME);
        backup_dir = dir_path.display().to_string();
        log::debug!("No backup dir flag provided; using default");
    };
    log::debug!("Got backup dir {backup_dir:}");
    match options::db_needed(matches) {
        Some(false) => {
            log::debug!("Database not needed for this command; skipping load ...");
            return Ok(store::db::new(db_file, backup_dir));
        }
        Some(true) => (),
        None => (),
    };
    log::debug!("Database is needed; preparing for read ...");
    let pwd = options::db_pwd(matches);
    store::db::open(
        db_file,
        backup_dir,
        pwd.expose_secret().to_string(),
        options::salt(matches),
    )
}
