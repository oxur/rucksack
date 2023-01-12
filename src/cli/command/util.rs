use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use super::prompt;

use crate::store::db;

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    match matches.get_one::<String>("db") {
        Some(db_file) => {
            let pwd = match matches.get_one::<String>("db-pass") {
                Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
                None => prompt::secret("Enter db password: ").unwrap(),
            };
            let salt = matches.get_one::<String>("salt").unwrap().to_string();
            db::open(db_file.to_owned(), pwd.expose_secret().to_string(), salt)
        }
        None => Ok(db::new()),
    }
}
