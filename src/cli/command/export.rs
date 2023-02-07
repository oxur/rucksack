use anyhow::{anyhow, Result};
use clap::ArgMatches;

use crate::app::App;
use crate::csv::writer;
use crate::csv::{chrome, firefox};
use crate::util::write_file;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'export' subcommand ...");
    let export_type = matches.get_one::<String>("type").map(|s| s.as_str());
    // For non-debug types, we need the file option set; for the debug type, there
    // is not file option, so we need to process that one and return right away.
    if let Some("debug") = export_type {
        to_stdout(app)?;
        return Ok(());
    }
    let export_file = matches.get_one::<String>("file").unwrap().to_string();
    match export_type {
        Some("chrome") => to_chrome_csv(app, export_file),
        Some("firefox") => to_firefox_csv(app, export_file),
        Some(_) => todo!(),
        None => todo!(),
    }
}

fn to_stdout(app: &App) -> Result<()> {
    match app.db.collect_decrypted() {
        Ok(rs) => {
            for r in rs {
                println!("{r:?}")
            }
        }
        Err(e) => {
            log::error!("{e:?}")
        }
    }
    Ok(())
}

fn to_chrome_csv(app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in app.db.collect_decrypted()? {
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

fn to_firefox_csv(app: &App, csv_path: String) -> Result<(), anyhow::Error> {
    let mut wtr = writer::to_bytes()?;
    let mut count = 0;
    for dr in app.db.collect_decrypted()? {
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
    println!();
    println!("Exported {count} records (total records in DB: {total})")
}
