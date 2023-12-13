use std::path;

use anyhow::Result;
use clap::ArgMatches;

use rucksack_db::db::DB;
use rucksack_lib::file;

use crate::command;
use crate::input::{Config, Inputs};

#[derive(Debug)]
pub struct App {
    pub inputs: Inputs,
    pub db: DB,
}

impl App {
    pub fn new(cfg: Config, cmd: String, matches: &ArgMatches) -> Result<App> {
        log::debug!("Setting up rucksack application ...");
        let inputs = cfg.to_inputs(matches);
        let db = setup_db(&inputs, cmd)?;
        Ok(App { inputs, db })
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

    pub fn config_path(&self) -> path::PathBuf {
        let mut path = path::PathBuf::new();
        path.push(self.inputs.config_file());
        return path.parent().unwrap().to_path_buf();
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
        self.inputs.rucksack.name.clone()
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
        if self.inputs.retention.purge_on_shutdown {
            todo!();
        }
        if self.inputs.retention.delete_inactive {
            // TODO: iterate through all inactive records and flag them as
            // deleted
            todo!();
        }
        Ok(())
    }
}

pub fn setup_db(inputs: &Inputs, cmd: String) -> Result<DB> {
    log::debug!("Setting up database ...");
    log::trace!("Got inputs: {:#?}", inputs);
    if !inputs.db_needed() {
        log::debug!(
            "Database not needed for the '{}' command; skipping load ...",
            cmd
        );
        return Ok(DB::new(inputs.db_file(), inputs.backup_dir(), None, None));
    }
    log::debug!("Database is needed; preparing for read ...");
    let mut db = DB::new(
        inputs.db_file(),
        inputs.backup_dir(),
        Some(inputs.db_passwd()),
        Some(inputs.salt()),
    );
    db.open()?;
    Ok(db)
}
