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
