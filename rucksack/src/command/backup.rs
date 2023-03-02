//! # Displaying and Working with Backups
//!
//! Create a new backup of your current secrets DB:
//!
//! ```shell
//! rucksack backup
//! ```
//!
//! Get a list of current backup files:
//!
//! ```shell
//! rucksack backups list
//! ```
//!
//! Show just the latest backup file:
//!
//! ```shell
//! rucksack backups list --latest
//! ```
//!
//! Delete a specific backup:
//!
//! ```shell
//! rucksack backup delete <name from list command>
//! ```
//!
//! Restore the DB from a previous backup:
//!
//! ```shell
//! rucksack backup restore <name from list command>
//! ```
//!
use anyhow::{anyhow, Result};
use clap::ArgMatches;

use rucksack_db::db::backup;
use rucksack_lib::file;

use crate::app::App;
use crate::option;

use super::output::column::Column;
use super::output::option::Opts;
use super::output::result;
use super::output::table;

pub fn delete(matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_path = app.backup_path();
    let backup_name = option::backup_name(matches);
    log::debug!("Preparing to delete backup DB file '{}'", backup_name);
    if !backup_path.exists() {
        log::error!("Cannot find file {}", backup_path.display());
        return Err(anyhow!("backup file '{}' does not exist", backup_name));
    }
    file::delete(backup_path)
}

pub fn list(matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_dir = app.backup_dir();
    log::debug!("Preparing to list backup DB files in {backup_dir:}");
    let opts = Opts {
        backup_files: true,
        latest_only: option::latest(matches),
        ..Default::default()
    };
    let backups: file::Listing = if opts.latest_only {
        vec![backup::latest(backup_dir)?]
    } else {
        backup::list(backup_dir)?
    };
    let mut results: Vec<result::ResultRow> = Vec::new();
    for (name, _, perms) in backups {
        let mut r = result::ResultRow {
            ..Default::default()
        };
        r.add(Column::Name, name);
        r.add(Column::Permissions, perms);
        results.push(r);
    }
    let mut t = table::new(results.to_owned(), opts);
    t.display();
    println!("\n{} backup files)\n", results.len());
    Ok(())
}

pub fn restore(matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_dir = app.backup_dir();
    let mut backup_name = option::backup_name(matches);
    if backup_name.is_empty() {
        let (_, latest, _) = backup::latest(backup_dir)?;
        backup_name = latest;
    }
    // Do a backup before we go any further
    run(matches, app)?;
    backup::restore(app.backup_path(), backup_name.clone(), app.db_path())?;
    log::info!("Successfully restored {backup_name} to {}", app.db_file());
    Ok(())
}

pub fn run(_matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Backing up database ...");
    let r = backup::copy(
        app.db_file(),
        app.backup_dir(),
        app.db_version().to_string(),
    );
    let backup_file: String = match r {
        Ok(b) => b,
        Err(e) => {
            return Err(anyhow!(e));
        }
    };
    log::debug!("Backed up database to {backup_file}");
    Ok(())
}
