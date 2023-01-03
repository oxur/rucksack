use clap::Arg;

pub fn db_arg() -> Arg {
    Arg::new("db")
        .help("path to the encrypted database to use")
        .short('d')
        .long("db")
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
        .default_value(env!("USER"))
        .short('s')
        .long("salt")
}
