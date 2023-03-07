use anyhow::Result;

use rucksack::input::{config, options};
use rucksack::{command, handlers, input};

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
        return handlers::version();
    } else if let Some(shell) = options::completions(&matches) {
        return handlers::completions(shell, rucksack, cfg.rucksack.name);
    }

    // With top-level short-circuit flags sorted, let's try for subcommands:
    if let Some((_, subcmd_matches)) = matches.subcommand() {
        let app = rucksack::app::new(cfg, subcmd_matches)?;
        app.run(&matches)?;
        app.shutdown(&matches)
    } else {
        handlers::long_help(rucksack)
    }
}
