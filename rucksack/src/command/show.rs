//! # Displaying Miscellaneous Data / Metadata
//!
//! If you need to see what version of the database file format you're currently using:
//!
//! ```shell
//! rucksack show db-version
//! ```
//!
//! Note that this is not necessarily the version of rucksack you're running,
//! rather it will correspond to the version of rucksack that was used when
//! your secrets database was last updated.
//!
//! Display the default location of the config file:
//!
//! ```shell
//! rucksack show config-file
//! ```
//!
//! ```shell
//! <system config dir>/rucksack/config.toml
//! ```
//!
//! Display the default location of the database file:
//!
//! ```shell
//! rucksack show db-file
//! ```
//!
//! ```shell
//! <system config dir>/rucksack/data/secrets.db
//! ````
//!
//! # All Subcommands
//!
//! See the full list of supported subcommands with:
//! ```shell
//! rucksack show -h
//! ```
//!
use std::collections::HashMap;
use std::str;

use anyhow::Result;
use clap::ArgMatches;

use rucksack_db::records;
use rucksack_lib::util;

use crate::app::App;

pub fn config_file(_matches: &ArgMatches, app: &App) -> Result<()> {
    println!("\n{}\n", app.config_file());
    Ok(())
}

pub fn config(_matches: &ArgMatches, app: &App) -> Result<()> {
    match util::read_file(app.config_file()) {
        Ok(bytes) => {
            println!("\n{}\n", str::from_utf8(bytes.as_ref()).unwrap());
        }
        Err(e) => panic!("{}", e),
    }
    Ok(())
}

pub fn data_dir(_matches: &ArgMatches, app: &App) -> Result<()> {
    println!("\n{}\n", app.data_dir().to_str().unwrap());
    Ok(())
}

pub fn db_file(_matches: &ArgMatches, app: &App) -> Result<()> {
    println!("\n{}\n", app.db_file());
    Ok(())
}

pub fn db_version(_matches: &ArgMatches, app: &App) -> Result<()> {
    println!("\n{}\n", app.db_version());
    Ok(())
}

pub fn categories(_matches: &ArgMatches, app: &App) -> Result<()> {
    let mut results: HashMap<String, bool> = HashMap::new();
    for i in app.db.iter() {
        let dr = i.value().decrypt(app.db.store_pwd(), app.db.salt())?;
        results.insert(dr.metadata().category, true);
    }
    let mut tags: Vec<&String> = results.keys().clone().collect();
    tags.sort();
    println!("\n{tags:?}\n");
    Ok(())
}

pub fn tags(_matches: &ArgMatches, app: &App) -> Result<()> {
    let mut results: HashMap<String, bool> = HashMap::new();
    for i in app.db.iter() {
        let dr = i.value().decrypt(app.db.store_pwd(), app.db.salt())?;
        for t in dr.metadata().tags {
            results.insert(t.display_or_value(), true);
        }
    }
    let mut tags: Vec<&String> = results.keys().clone().collect();
    tags.sort();
    println!("\n{tags:?}\n");
    Ok(())
}

pub fn types(_matches: &ArgMatches, _app: &App) -> Result<()> {
    println!("\n{:?}\n", records::types());
    Ok(())
}
