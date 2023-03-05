use anyhow::Result;

use rucksack::input::{config, options};
use rucksack::{command, input};

fn main() -> Result<()> {
    let rucksack = command::setup();
    let matches = rucksack.clone().get_matches();
    let cfg = config::load(
        config::Opts::new()
            .file_name(options::config_file(&matches))
            .log_level(options::log_level(&matches))
            .name(input::constant::NAME.to_string()),
    )?;

    // Top-level short-circuit commands: the following are completely
    // independent, so perform them before any config or subcommand operations.
    if options::version(&matches) {
        return command::version();
    } else if let Some(shell) = options::completions(&matches) {
        return command::completions(shell, rucksack, cfg.rucksack.name);
    }

    // With top-level flags sorted, let's try for subcommands:
    let subcommand = matches.subcommand();
    if subcommand.is_none() {
        return command::long_help(rucksack);
    }

    // If there are subcommands, let's fire up the app and dispatch appropriately:
    let (_, subcmd_matches) = subcommand.unwrap();
    let app = rucksack::app::new(cfg, subcmd_matches)?;
    app.run(&matches)?;
    app.shutdown(&matches)
}
