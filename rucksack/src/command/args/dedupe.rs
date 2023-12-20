use clap::Arg;

pub fn dd_type() -> Arg {
    Arg::new("dedupe-type")
        .help("The type of deduplication to perform")
        .long("dedupe-type")
        .env("RUXAK_DEDUPE_TYPE")
}
