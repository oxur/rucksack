//! # Deduplicating Records
//!
//! To combine all separate records that have the same data, converting them
//! instead to history entries for one record:
//!
//! ```shell
//! rucksack dedupe --type exact
//! ```
//!
//! The dedupe type `exact` is the safest and thus the default type, so the above
//! may be executed more succinctly with:
//!
//! ```shell
//! rucksack dedupe
//! ```
//!
//! To combine records that differ only by password, converting them to history
//! entries of one (the oldest) record and to set the most recent (timestamp)
//! as current:
//!
//! ```shell
//! rucksack deduple --type updated
//! ```
// use anyhow::{anyhow, Result};
use anyhow::Result;
use clap::ArgMatches;

// use rucksack_db as store;
use rucksack_db::db::DB;
// use rucksack_db::records;

use crate::app::App;
// use crate::input::{query, Flag};

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("exact") => dedupe_exact(matches, &app.db)?,
        Some("updated") => dedupe_passwords_updated(matches, &app.db)?,
        Some("all") => dedupe_all(matches, &app.db)?,
        Some("") => dedupe_exact(matches, &app.db)?,
        Some(_) => todo!(),
        None => dedupe_exact(matches, &app.db)?,
    };
    app.db.close()?;
    Ok(())
}

fn dedupe_exact(_matches: &ArgMatches, _db: &DB) -> Result<(), anyhow::Error> {
    log::debug!("Performing exact record deduplication ...");
    Ok(())
}

fn dedupe_passwords_updated(_matches: &ArgMatches, _db: &DB) -> Result<(), anyhow::Error> {
    log::debug!("Performing updated password record deduplication ...");
    Ok(())
}

fn dedupe_all(matches: &ArgMatches, db: &DB) -> Result<(), anyhow::Error> {
    dedupe_exact(matches, db)?;
    dedupe_passwords_updated(matches, db)?;
    Ok(())
}
