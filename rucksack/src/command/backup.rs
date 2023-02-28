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

use super::output::column::Column;
use super::output::option::Opts;
use super::output::result;
use super::output::table;

pub fn delete(matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_name = matches.get_one::<String>("name").unwrap();
    log::debug!("Preparing to delete backup DB file '{}'", backup_name);
    let file_path = app.backup_dir().join(backup_name);
    if !file_path.exists() {
        log::error!("Cannot find file {}", file_path.display());
        return Err(anyhow!("backup file '{}' does not exist", backup_name));
    }
    file::delete(file_path)
}

pub fn list(_matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_path_name = app.backup_dir().display().to_string();
    log::debug!("Preparing to list backup DB files in {:}", backup_path_name);
    let opts = Opts {
        backup_files: true,
        ..Default::default()
    };
    let mut backups = file::files(backup_path_name)?;
    backups.sort();
    backups.reverse();
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

pub fn restore(_matches: &ArgMatches, _app: &App) -> Result<()> {
    todo!()
    // Ok(())
}

pub fn run(_matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Backing up database ...");
    let backup_file: String;
    let r = backup::copy(
        app.db_file(),
        app.backup_dir().display().to_string(),
        app.db_version().to_string(),
    );
    match r {
        Ok(b) => {
            backup_file = b;
        }
        Err(e) => {
            return Err(anyhow!(e));
        }
    };
    log::debug!("Backed up database to {backup_file}");
    Ok(())
}
