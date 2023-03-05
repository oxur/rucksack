use std::io;

use anyhow::{Context, Result};

use rucksack::command as cli;
use rucksack::input;
use rucksack::input::{config, options};
use rucksack_lib::util;

fn main() -> Result<()> {
    let mut rucksack = cli::setup();
    let matches = rucksack.clone().get_matches();
    let cfg = config::load(
        config::Opts::new()
            .file_name(options::config_file(&matches))
            .log_level(options::log_level(&matches))
            .name(input::constant::NAME.to_string()),
    )?;

    // Shell completion generation is completely independent, so perform it before
    // any config or subcommand operations.
    if let Some(is_version) = matches.get_one::<bool>("version") {
        if *is_version {
            return util::display(rucksack::version().to_string().as_str());
        }
    } else if let Some(shell) = matches.get_one::<clap_complete::Shell>("completions") {
        clap_complete::generate(*shell, &mut rucksack, cfg.rucksack.name, &mut io::stdout());
        return Ok(());
    }

    if matches.subcommand().is_none() {
        return rucksack
            .clone()
            .print_long_help()
            .with_context(|| "failed to print help".to_string());
    }

    let (_, subcmd_matches) = matches.subcommand().unwrap();
    // TODO: let's decide what the SoT should be for data in the running app,
    // and then consolidate all data there (from CLI flags, env vars, config,
    // etc.). See ticket for more details:
    // * https://github.com/oxur/rucksack/issues/92
    // cfg.rucksack.db_file = db.path();
    // cfg.rucksack.data_dir = file::dir_parent(db.path());
    let app = rucksack::app::new(cfg, subcmd_matches)?;
    app.run(&matches)?;
    app.shutdown(&matches)
}
