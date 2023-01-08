use anyhow::{anyhow, Result};
use clap::ArgMatches;

use crate::app;
use crate::csv::writer;
use crate::csv::{chrome, firefox};
use crate::store;
use crate::util::write_file;

pub fn new(matches: &ArgMatches, app: &app::App) -> Result<()> {
    let export_file = matches.get_one::<String>("file").unwrap().to_string();
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("chrome") => to_chrome_csv(&app.db, export_file)?,
        Some("firefox") => to_firefox_csv(&app.db, export_file)?,
        Some(_) => todo!(),
        None => todo!(),
    };
    Ok(())
}

fn to_chrome_csv(db: &store::db::DB, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in db.collect_decrypted()? {
        wtr.serialize(chrome::from_decrypted(dr))?;
        count += 1;
        print!(".");
    }
    wtr.flush()?;
    match wtr.into_inner() {
        Ok(data) => {
            print_report(count, db.hash_map().len());
            write_file(data, csv_path)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

fn to_firefox_csv(db: &store::db::DB, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in db.collect_decrypted()? {
        wtr.serialize(firefox::from_decrypted(dr))?;
        count += 1;
        print!(".");
    }
    wtr.flush()?;
    match wtr.into_inner() {
        Ok(data) => {
            print_report(count, db.hash_map().len());
            write_file(data, csv_path)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

fn print_report(count: usize, total: usize) {
    println!();
    println!(
        "Exported {} records (total records in DB: {})",
        count, total
    )
}
