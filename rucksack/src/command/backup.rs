//! # Displaying and Working with Backups
//!
// use std::collections::HashMap;
// use std::str;
use anyhow::Result;
use clap::ArgMatches;

// use rucksack_db::records;
use rucksack_lib::file;

use crate::app::App;

// use super::output::option;
// use super::output::result;
// use super::output::table;

pub fn delete(_matches: &ArgMatches, _app: &App) -> Result<()> {
    todo!()
    // Ok(())
}

pub fn list(_matches: &ArgMatches, app: &App) -> Result<()> {
    let backup_path_name = app.backup_dir().display().to_string();
    log::debug!("Preparing to list backup DB files in {:}", backup_path_name);
    let mut backups = file::files(backup_path_name)?;
    backups.sort();
    backups.reverse();
    for (created, name, perms) in backups {
        println!("{name}, {created}, {perms}");
    }
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
