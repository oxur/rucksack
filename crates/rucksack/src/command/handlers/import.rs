//! # Importing
//!
//! Import login data from Firefox Sync:
//!
//! ```shell
//! rucksack import \
//!   --db-pass abc123 \
//!   --type firefox \
//!   --file ~/Downloads/logins.csv
//! ```
//!
//! From Chrome or Brave:
//!
//! ```shell
//! rucksack import \
//!   --db-pass abc123 \
//!   --type chrome \
//!   --file ~/Downloads/logins.csv
//! ```
//!
use anyhow::{Context, Result};
use clap::ArgMatches;

use rucksack_db::csv;
use rucksack_db::csv::{chrome, firefox};
use rucksack_db::db::DB;
use rucksack_db::records::DEFAULT_CATEGORY;
use rucksack_db::{records, DecryptedRecord};

use crate::app::App;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    let import_file = matches.get_one::<String>("file").unwrap().to_string();

    match matches.get_one::<String>("format").map(|s| s.as_str()) {
        Some("chrome") => from_chrome_csv(matches, &app.db, import_file)?,
        Some("firefox") => from_firefox_csv(matches, &app.db, import_file)?,
        Some("") => from_firefox_csv(matches, &app.db, import_file)?,
        Some(_) => todo!(),
        None => from_firefox_csv(matches, &app.db, import_file)?,
    };
    Ok(())
}

fn from_chrome_csv(matches: &ArgMatches, db: &DB, csv_path: String) -> Result<(), anyhow::Error> {
    println!("Importing Chrome data from {csv_path}:");
    let mut rdr = csv::reader::from_path(csv_path.clone())
        .with_context(|| format!("failed to read Chrome CSV file '{}'", csv_path))?;
    let mut count = 0;
    for result in rdr.deserialize() {
        let chr: chrome::Record = result.context("failed to parse Chrome CSV record")?;
        let mut dr = chr.to_decrypted();
        log::debug!(key = dr.key().as_str(), operation = "import"; "Processing record");
        if !valid_import(matches, dr.clone()) {
            continue;
        }
        dr.set_name(dr.name_or_user());
        db.insert(dr)
            .context("failed to insert imported record into database")?;
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
        .context("failed to save database after Chrome import")
}

fn from_firefox_csv(matches: &ArgMatches, db: &DB, csv_path: String) -> Result<(), anyhow::Error> {
    println!("Importing Firefox data from {csv_path}:");
    let mut rdr = csv::reader::from_path(csv_path.clone())
        .with_context(|| format!("failed to read Firefox CSV file '{}'", csv_path))?;
    let mut count: usize = 0;
    for result in rdr.deserialize() {
        let ffr: firefox::Record = result.context("failed to parse Firefox CSV record")?;
        let mut dr = ffr.to_decrypted();
        log::debug!(key = dr.key().as_str(), operation = "import"; "Processing record");
        if !valid_import(matches, dr.clone()) {
            continue;
        }
        dr.set_name(dr.name_or_user());
        db.insert(dr)
            .context("failed to insert imported record into database")?;
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
        .context("failed to save database after Firefox import")
}

fn print_report(count: usize, total: usize) {
    println!("\nImported {count} records (total records in DB: {total})",)
}

fn valid_import(_matches: &ArgMatches, r: DecryptedRecord) -> bool {
    // Right now, only Kind::Password records of the "default" category are
    // supported for import
    let md = r.metadata();
    if md.kind == records::Kind::Password && md.category == DEFAULT_CATEGORY {
        return true;
    }
    false
}
