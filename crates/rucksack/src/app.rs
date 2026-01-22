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
        log::debug!(operation = "setup"; "Setting up rucksack application");
        let inputs = cfg.to_inputs(matches);
        let db = setup_db(&inputs, cmd)?;
        Ok(App { inputs, db })
    }

    pub fn backup_dir(&self) -> &path::Path {
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
        // SAFETY: Config file paths always have a parent directory.
        // If this path somehow doesn't have a parent (e.g., root "/"),
        // fall back to current directory.
        path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path::PathBuf::from("."))
    }

    pub fn data_dir(&self) -> String {
        self.data_path().display().to_string()
    }

    pub fn data_path(&self) -> path::PathBuf {
        // SAFETY: Database file paths always have a parent directory.
        // If this path somehow doesn't have a parent (e.g., root "/"),
        // fall back to current directory.
        self.db_path()
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path::PathBuf::from("."))
    }

    pub fn db_file(&self) -> &path::Path {
        self.db.file_name()
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
        log::info!(operation = "execute"; "Executing rucksack command");
        if !self.backup_path().exists() {
            log::debug!(dir = self.backup_dir().to_string_lossy().as_ref(), operation = "check_backup_dir"; "Checking for backup dir");
            file::create_dirs(self.backup_path())?;
            log::info!(operation = "create_backup_dir"; "Created backup dir");
        }
        command::dispatch(self, matches)?;
        log::debug!(operation = "execute"; "Command execution complete");
        Ok(())
    }

    pub fn shutdown(&self, _matches: &ArgMatches) -> Result<()> {
        log::info!(operation = "shutdown"; "Performing shutdown operations");
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
    log::debug!(operation = "setup_db"; "Setting up database");
    log::trace!(db_file = inputs.db_file().to_string_lossy().as_ref(), operation = "setup_db"; "Got inputs");
    if !inputs.db_needed() {
        log::debug!(cmd = &cmd[..], operation = "setup_db"; "Database not needed for command; skipping load");
        return Ok(DB::new(inputs.db_file(), inputs.backup_dir(), None, None));
    }
    log::debug!(operation = "setup_db"; "Database is needed; preparing for read");
    let mut db = DB::new(
        inputs.db_file(),
        inputs.backup_dir(),
        Some(inputs.db_passwd()),
        Some(inputs.salt()),
    );
    db.open()?;
    Ok(db)
}
