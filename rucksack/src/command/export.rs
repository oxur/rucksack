//! # Exporting
//!
//! Logins may be exported to files that can then be used to import into browsers:
//!
//! ```shell
//! rucksack export \
//!   --db-pass abc123 \
//!   --type chrome \
//!   --file /tmp/exported-logins.csv
//! ```
//!

use anyhow::{anyhow, Result};
use clap::ArgMatches;

use rucksack_db::csv::writer;
use rucksack_db::csv::{chrome, firefox};
use rucksack_db::records::DEFAULT_CATEGORY;
use rucksack_db::{records, DecryptedRecord, Status};
use rucksack_lib::util::write_file;

use crate::app::App;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'export' subcommand ...");
    let serialised_format = matches.get_one::<String>("format").map(|s| s.as_str());
    // For non-debug types, we need the file option set; for the debug type, there
    // is not file option, so we need to process that one and return right away.
    if let Some("debug") = serialised_format {
        to_stdout(app)?;
        return Ok(());
    }
    let export_file = matches.get_one::<String>("output").unwrap().to_string();
    match serialised_format {
        Some("chrome") => to_chrome_csv(matches, app, export_file),
        Some("firefox") => to_firefox_csv(matches, app, export_file),
        Some("") => to_firefox_csv(matches, app, export_file),
        Some(_) => todo!(),
        None => to_firefox_csv(matches, app, export_file),
    }
}

fn to_stdout(app: &App) -> Result<()> {
    match app.db.collect_decrypted() {
        Ok(rs) => {
            for r in rs {
                if r.metadata().state == Status::Deleted {
                    continue;
                }
                println!("{r:?}")
            }
        }
        Err(e) => {
            log::error!("{e:?}")
        }
    }
    Ok(())
}

fn to_chrome_csv(matches: &ArgMatches, app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in app.db.collect_decrypted()? {
        log::debug!("Record: {}", dr.key());
        if !valid_export(matches, dr.clone()) {
            continue;
        }
        wtr.serialize(chrome::from_decrypted(dr))?;
        count += 1;
        print!(".");
    }
    wtr.flush()?;
    match wtr.into_inner() {
        Ok(data) => {
            print_report(count, app.db.hash_map().len());
            write_file(data, csv_path)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

fn to_firefox_csv(matches: &ArgMatches, app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in app.db.collect_decrypted()? {
        log::debug!("Record: {}", dr.key());
        if !valid_export(matches, dr.clone()) {
            continue;
        }
        wtr.serialize(firefox::from_decrypted(dr))?;
        count += 1;
        print!(".");
    }
    wtr.flush()?;
    match wtr.into_inner() {
        Ok(data) => {
            print_report(count, app.db.hash_map().len());
            write_file(data, csv_path)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

fn print_report(count: usize, total: usize) {
    println!("\nExported {count} records (total records in DB: {total})")
}

fn valid_export(_matches: &ArgMatches, r: DecryptedRecord) -> bool {
    // Right now, only Kind::Password records of the "default" category are
    // supported for export
    let md = r.metadata();
    if md.kind == records::Kind::Password
        && md.category == DEFAULT_CATEGORY
        && md.state != Status::Deleted
    {
        return true;
    }
    false
}
