use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;

pub fn init(_matches: &ArgMatches, _app: &App) -> Result<()> {
    Ok(())
}
