use clap::Arg;

use crate::input::constant;

#[doc(hidden)]
pub fn config() -> Arg {
    let config_file = rucksack_lib::file::config_file(constant::NAME);
    Arg::new("config-file")
        .help("The path to the config file to use or create")
        .long("config-file")
        .default_value(config_file)
        .global(true)
}

#[doc(hidden)]
pub fn log_level() -> Arg {
    Arg::new("log-level")
        .help("Override the configured log-level setting")
        .long("log-level")
        .default_value("")
        .value_parser(["error", "warn", "info", "debug", "trace", ""])
        .global(true)
}