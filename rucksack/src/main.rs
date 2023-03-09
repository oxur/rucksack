use anyhow::{anyhow, Result};

use rucksack::input::{config, options};
use rucksack::service::actor::Commander;
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

    // Top-level short-circuit flags: the following are completely
    // independent, so perform them before any config or subcommand operations.
    if options::version(&matches) {
        return command::handlers::version();
    } else if let Some(shell) = options::completions(&matches) {
        return command::handlers::completions(shell, rucksack, cfg.rucksack.name);
    }

    // With top-level short-circuit flags sorted, let's try for subcommands,
    // checking first to see if we're going to be running rucksack as a
    // daemon.
    match matches.subcommand() {
        // Daemon:
        Some(("start", start_matches)) => {
            let daemon = Commander::start(cfg, start_matches)?;
            match daemon.run() {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!(e)),
            }
        }
        // CLI:
        Some((_, subcmd_matches)) => {
            let app = rucksack::App::new(cfg, subcmd_matches)?;
            app.run(&matches)?;
            app.shutdown(&matches)
        }
        None => command::handlers::long_help(rucksack),
    }
}
