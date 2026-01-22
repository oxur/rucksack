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
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;

use rucksack_db::csv::writer;
use rucksack_db::csv::{chrome, firefox};
use rucksack_db::records::DEFAULT_CATEGORY;
use rucksack_db::{records, DecryptedRecord, Status};
use rucksack_lib::file;

use crate::app::App;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!(operation = "export"; "Running 'export' subcommand");
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
    let rs = app
        .db
        .collect_decrypted()
        .context("failed to decrypt records for export")?;
    for r in rs {
        if r.metadata().state == Status::Deleted {
            continue;
        }
        println!("{r:?}")
    }
    Ok(())
}

fn to_chrome_csv(matches: &ArgMatches, app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes().context("failed to create CSV writer")?;
    let mut count = 0;
    for dr in app
        .db
        .collect_decrypted()
        .context("failed to decrypt records for Chrome export")?
    {
        log::debug!(key = dr.key().as_str(), operation = "export"; "Processing record");
        if !valid_export(matches, dr.clone()) {
            continue;
        }
        wtr.serialize(chrome::from_decrypted(dr))
            .context("failed to serialize record to Chrome CSV format")?;
        count += 1;
        print!(".");
    }
    wtr.flush().context("failed to flush CSV writer")?;
    let data = wtr
        .into_inner()
        .map_err(|e| anyhow!("failed to finalize CSV data: {}", e))?;
    print_report(count, app.db.hash_map().len());
    file::write(&data, csv_path.clone())
        .with_context(|| format!("failed to write Chrome CSV export to '{}'", csv_path))
}

fn to_firefox_csv(matches: &ArgMatches, app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes().context("failed to create CSV writer")?;
    let mut count = 0;
    for dr in app
        .db
        .collect_decrypted()
        .context("failed to decrypt records for Firefox export")?
    {
        log::debug!(key = dr.key().as_str(), operation = "export"; "Processing record");
        if !valid_export(matches, dr.clone()) {
            continue;
        }
        wtr.serialize(firefox::from_decrypted(dr))
            .context("failed to serialize record to Firefox CSV format")?;
        count += 1;
        print!(".");
    }
    wtr.flush().context("failed to flush CSV writer")?;
    let data = wtr
        .into_inner()
        .map_err(|e| anyhow!("failed to finalize CSV data: {}", e))?;
    print_report(count, app.db.hash_map().len());
    file::write(&data, csv_path.clone())
        .with_context(|| format!("failed to write Firefox CSV export to '{}'", csv_path))
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
