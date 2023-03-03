use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use rucksack_db::db;
use rucksack_lib::file;

use crate::input::{constant, options, prompt};

pub fn db(matches: &ArgMatches) -> Result<db::DB> {
    let db_file = match options::db(matches) {
        Some(file_path) => {
            log::debug!("Got database file from flag: {}", file_path);
            file_path
        }
        None => {
            let file_name = file::db_file(constant::NAME);
            log::debug!("No database flag provided; using default ({file_name:})");
            file_name
        }
    };
    let mut backup_dir = options::backup_dir(matches);
    if backup_dir.is_empty() {
        let dir_path = file::backup_dir(constant::NAME);
        backup_dir = dir_path.display().to_string();
        log::debug!("No backup dir flag provided; using default");
    };
    log::debug!("Got backup dir {backup_dir:}");
    match matches.get_one::<bool>("db-needed") {
        Some(false) => {
            log::debug!("Database not needed for this command; skipping load ...");
            return Ok(db::new(db_file, backup_dir));
        }
        Some(true) => (),
        None => (),
    }
    log::debug!("Database is needed; preparing for read ...");
    let pwd = match matches.get_one::<String>("db-pass") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter db password: ").unwrap(),
    };
    let salt = matches.get_one::<String>("salt").unwrap().to_string();
    db::open(db_file, backup_dir, pwd.expose_secret().to_string(), salt)
}
