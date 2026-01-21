use clap::Arg;

pub fn path() -> Arg {
    Arg::new("db")
        .help("Path to the encrypted database to use")
        .short('d')
        .long("db")
        .env("RUXAK_DB")
        .global(true)
}

pub fn pwd() -> Arg {
    Arg::new("db-pass")
        .help("Password used to encrypt the database")
        .long("db-pass")
        .env("RUXAK_DB_PASS")
        .global(true)
}

pub fn salt() -> Arg {
    Arg::new("salt")
        .help("The salt to use for encrypting the database")
        .short('s')
        .long("salt")
        .env("RUXAK_SALT")
        .global(true)
}

pub fn backup_dir() -> Arg {
    Arg::new("backup-dir")
        .help("Path for database backups")
        .long("backup-dir")
        .env("RUXAK_BACKUP_DIR")
        .global(true)
}

// TODO: let's look at the other bool flags and make sure we're being consistent
pub fn not_needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("false")
        .global(true)
}

// TODO: let's look at the other bool flags and make sure we're being consistent
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
        .env("RUXAK_FORMAT")
        .value_parser(["", "chrome", "debug", "firefox"])
        .global(true)
}
