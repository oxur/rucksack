//! # Displaying and Working with Backups
//!
// use std::collections::HashMap;
// use std::str;
use anyhow::anyhow;
use anyhow::Result;
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
    let mut backups = file::files(backup_dir)?;
    backups.sort();
    backups.reverse();
    let mut results: Vec<result::ResultRow> = Vec::new();
    for (name, _, perms) in backups {
        if opts.latest_only && results.len() == 1 {
            break;
        }
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

pub fn restore(_matches: &ArgMatches, _app: &App) -> Result<()> {
    // TODO: get the latest backup
    // TODO: before a backup
    // TODO: replace the db file with the first latest backup
    todo!()
    // Ok(())
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
