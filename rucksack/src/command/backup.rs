//! # Displaying and Working with Backups
//!
// use std::collections::HashMap;
// use std::str;
use anyhow::Result;
use clap::ArgMatches;

// use rucksack_db::records;
use rucksack_lib::file;

use crate::app::App;

use super::output::column::Column;
use super::output::option::Opts;
use super::output::result;
use super::output::table;

pub fn delete(_matches: &ArgMatches, _app: &App) -> Result<()> {
    todo!()
    // Ok(())
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

pub fn run(_matches: &ArgMatches, _app: &App) -> Result<()> {
    todo!()
    // Ok(())
}
