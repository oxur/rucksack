use std::io;

use anyhow::{Context, Result};

use rucksack::command as cli;
use rucksack::input;
use rucksack_lib::util;

fn main() -> Result<()> {
    let mut rucksack = cli::setup();
    let matches = rucksack.clone().get_matches();
    let mut config_file = String::new();
    if let Some(cfg_file) = matches.get_one::<String>("config-file") {
        config_file = cfg_file.to_string();
    }
    let mut log_level = String::new();
    if let Some(level) = matches.get_one::<String>("log-level") {
        log_level = level.to_string();
    }
    let cfg = input::config::load(
        input::config::Opts::new()
            .file_name(config_file)
            .log_level(log_level)
            .name(input::constant::NAME.to_string()),
    )?;
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => {
            panic!("Could not setup logger: {error:?}")
        }
    }
    log::debug!("Config setup complete (using {})", cfg.rucksack.cfg_file);
    log::debug!("Logger setup complete");

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
