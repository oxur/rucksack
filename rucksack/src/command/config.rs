use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::input::config::{init, Opts};

pub fn re_init(_matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Re-initialising the rucksack configuration ...");
    init(
        Opts::new()
            .file_name(app.cfg.rucksack.cfg_file.clone())
            .force(),
    )
}
