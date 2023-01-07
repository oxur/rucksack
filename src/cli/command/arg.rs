use clap::Arg;

pub fn db_arg() -> Arg {
    Arg::new("db")
        .help("path to the encrypted database to use")
        .short('d')
        .long("db")
        .default_value("./data/creds.db")
}

pub fn pwd_arg() -> Arg {
    Arg::new("password")
        .help("password used to encrypt the database")
        .short('p')
        .long("password")
}

pub fn salt_arg() -> Arg {
    Arg::new("salt")
        .help("the salt to use for encrypting the database")
        .default_value(default_salt())
        .short('s')
        .long("salt")
}

fn default_salt() -> String {
    match std::env::var("USER") {
        Ok(user) => user,
        Err(_) => "rucksack".to_string(),
    }
}
