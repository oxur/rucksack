use clap::Arg;

pub fn dd_type() -> Arg {
    Arg::new("dedupe-type")
        .help("The type of deduplication to perform")
        .long("dedupe-type")
        .env("RUXAK_DEDUPE_TYPE")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dd_type() {
        let arg = dd_type();
        assert_eq!(arg.get_id(), "dedupe-type");
    }
}
