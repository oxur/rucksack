use anyhow::Result;
use clap::ArgMatches;

use rucksack_lib::config::init;

use crate::app::App;

pub fn re_init(_matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Re-initialising the rucksack configuration ...");
    init::recreate(app.cfg.rucksack.cfg_file.clone())
}
