use clap::Arg;

// Database Flags

pub fn db_arg() -> Arg {
    Arg::new("db")
        .help("path to the encrypted database to use")
        .short('d')
        .long("db")
        .default_value("./data/creds.db")
}

pub fn pwd_arg() -> Arg {
    Arg::new("db-pass")
        .help("password used to encrypt the database")
        .long("db-pass")
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

// Account Flags

pub fn account_type() -> Arg {
    Arg::new("type")
        .help("the type of secret to add")
        .short('t')
        .long("type")
        // These next have not yet been defined/refined:
        .value_parser(["", "account", "credential", "creds", "password"])
}

pub fn account_user() -> Arg {
    Arg::new("user")
        .help("the user, login, or account, identifier")
        .short('u')
        .long("user")
        .required(true)
}

pub fn account_pass() -> Arg {
    Arg::new("password")
        .help("the account / login password")
        .long("password")
        .required(true)
}

pub fn account_url() -> Arg {
    Arg::new("url").help("the login URL").long("url")
}
