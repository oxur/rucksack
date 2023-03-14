use clap::Arg;

#[doc(hidden)]
pub fn config() -> Arg {
    Arg::new("config-file")
        .help("The path to the config file to use or create")
        .long("config-file")
        .env("RUXAK_CONFIG_FILE")
        .global(true)
}

#[doc(hidden)]
pub fn log_level() -> Arg {
    Arg::new("log-level")
        .help("Override the configured log-level setting")
        .long("log-level")
        // .default_value("")
        .env("RUXAK_LOG_LEVEL")
        .value_parser(["error", "warn", "info", "debug", "trace", ""])
        .global(true)
}
