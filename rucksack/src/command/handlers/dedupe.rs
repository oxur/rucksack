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
use std::fmt;

// use anyhow::{anyhow, Result};
use anyhow::Result;
use clap::ArgMatches;

// use rucksack_db as store;
use rucksack_db::db::DB;
use rucksack_db::records::DecryptedRecord;

use crate::app::App;
use crate::output::{result, Column};

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

pub fn post_process(in_groups: result::GroupByString) -> result::GroupByString {
    let mut out_groups = result::GroupByString::new();
    for (key, group) in in_groups.into_iter() {
        let entry = out_groups.entry(key).or_default();
        for r in group.iter() {
            let mut updated = r.clone();
            updated.add(Column::DupeInfo, DupeInfo::Duplicate.name());
            entry.push(updated);
        }
    }
    out_groups
}

pub fn dupe_info(_record_a: DecryptedRecord, _record_b: DecryptedRecord) -> DupeInfo {
    DupeInfo::Primary
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DupeInfo {
    Primary,
    InHistory,
    Duplicate,
}

impl DupeInfo {
    pub fn name(&self) -> String {
        match self {
            DupeInfo::InHistory => "in history".to_string(),
            _ => format!("{self}").to_lowercase(),
        }
    }
}

impl fmt::Display for DupeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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
