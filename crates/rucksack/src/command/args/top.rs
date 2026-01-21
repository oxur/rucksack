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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let arg = config();
        assert_eq!(arg.get_id(), "config-file");
        assert!(arg.is_global_set());
    }

    #[test]
    fn test_log_level() {
        let arg = log_level();
        assert_eq!(arg.get_id(), "log-level");
        assert!(arg.is_global_set());
    }
}
