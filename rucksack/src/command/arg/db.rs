use clap::Arg;

pub fn path() -> Arg {
    Arg::new("db")
        .help("Path to the encrypted database to use")
        .short('d')
        .long("db")
        .global(true)
}

pub fn pwd() -> Arg {
    Arg::new("db-pass")
        .help("Password used to encrypt the database")
        .long("db-pass")
        .global(true)
}

pub fn salt() -> Arg {
    Arg::new("salt")
        .help("The salt to use for encrypting the database")
        .default_value(default_salt())
        .short('s')
        .long("salt")
        .global(true)
}

fn default_salt() -> String {
    match std::env::var("USER") {
        Ok(user) => user,
        Err(_) => "rucksack".to_string(),
    }
}

pub fn not_needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("false")
        .global(true)
}

pub fn needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("true")
        .global(true)
}

pub fn serialised_format() -> Arg {
    Arg::new("format")
        .help("the de/serialisation format to use for import/export")
        .long("format")
        .value_parser(["", "chrome", "debug", "firefox"])
        .global(true)
}
