use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;

pub fn config_file(matches: &ArgMatches, app: &App) -> Result<()> {
    log::info!("matches: {:?}; app: {:?}", matches, app);
    log::warn!("not yet implemented");
    Ok(())
}

pub fn config(matches: &ArgMatches, app: &App) -> Result<()> {
    log::info!("matches: {:?}; app: {:?}", matches, app);
    log::warn!("not yet implemented");
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
