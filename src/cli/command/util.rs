use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use super::prompt;

use crate::store::db;

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    let db_file = matches.get_one::<String>("db").unwrap().to_string();
    let pwd = match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter db password: ").unwrap(),
    };
    let salt = matches.get_one::<String>("salt").unwrap().to_string();
    db::open(db_file, pwd.expose_secret().to_string(), salt)
}
