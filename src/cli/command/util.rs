use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use super::prompt;

use crate::store::db;

pub fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    let db_file = matches.get_one::<String>("db").unwrap().to_string();
    let pwd = match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter db password: ").unwrap(),
    };
    let now = chrono::offset::Local::now().to_rfc3339();
    db::open(db_file, pwd.expose_secret().to_string(), now)
}
