use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::csv;
use crate::csv::{chrome, firefox};
use crate::store::db::DB;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    let import_file = matches.get_one::<String>("file").unwrap().to_string();

    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("chrome") => from_chrome_csv(&app.db, import_file)?,
        Some("firefox") => from_firefox_csv(&app.db, import_file)?,
        Some(_) => todo!(),
        None => todo!(),
    };
    Ok(())
}

fn from_chrome_csv(db: &DB, csv_path: String) -> Result<(), anyhow::Error> {
    println!("Importing data from {csv_path}:");
    let mut rdr = csv::reader::from_path(csv_path)?;
    let mut count = 0;
    for result in rdr.deserialize() {
        let chr: chrome::Record = result?;
        db.insert(chr.to_decrypted());
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
}

fn from_firefox_csv(db: &DB, csv_path: String) -> Result<(), anyhow::Error> {
    println!("Importing data from {csv_path}:");
    let mut rdr = csv::reader::from_path(csv_path)?;
    let mut count: usize = 0;
    for result in rdr.deserialize() {
        let ffr: firefox::Record = result?;
        db.insert(ffr.to_decrypted());
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
}

fn print_report(count: usize, total: usize) {
    println!("\nImported {count} records (total records in DB: {total})")
}
