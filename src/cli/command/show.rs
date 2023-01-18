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

pub fn data_dir(matches: &ArgMatches, app: &App) -> Result<()> {
    log::info!("matches: {:?}; app: {:?}", matches, app);
    log::warn!("not yet implemented");
    Ok(())
}

pub fn db_file(matches: &ArgMatches, app: &App) -> Result<()> {
    log::info!("matches: {:?}; app: {:?}", matches, app);
    log::warn!("not yet implemented");
    Ok(())
}
