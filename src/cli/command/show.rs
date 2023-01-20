use std::str;

use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::util;

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
