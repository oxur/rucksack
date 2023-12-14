use clap::Arg;

pub fn dd_type() -> Arg {
    Arg::new("type")
        .help("The type of deduplication to perform")
        .short('t')
        .long("type")
        .env("RUXAK_DEDUPE_TYPE")
}
