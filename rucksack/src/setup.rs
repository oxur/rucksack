use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use rucksack_db::db;
use rucksack_lib::file;

use crate::{constant, prompt};

pub fn db(matches: &ArgMatches) -> Result<db::DB> {
    let db = matches.get_one::<String>("db");
    match matches.get_one::<bool>("db-needed") {
        Some(false) => {
            log::debug!("Database not needed for this command; skipping load ...");
            if let Some(db_file) = db {
                let mut db = db::new();
                db.path = db_file.to_string();
                return Ok(db);
            }
            return Ok(db::new());
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
    let db_file: String;
    match db {
        Some(file_path) => {
            log::debug!("Got database file from flag: {}", file_path);
            db_file = file_path.to_owned();
        }
        None => {
            db_file = file::db_file(constant::NAME);
            log::debug!("No database flag provided; using default ({db_file:})");
        }
    }
    let backup_dir: String;
    match matches.get_one::<String>("backup-dir") {
        Some(dir_path) => {
            log::debug!("Got database backups dir from flag: {}", dir_path);
            backup_dir = dir_path.to_owned();
        }
        None => {
            backup_dir = file::backup_dir(constant::NAME).display().to_string();
            log::debug!("No backup dir flag provided; using default ({backup_dir:})");
        }
    }
    db::open(db_file, backup_dir, pwd.expose_secret().to_string(), salt)
}
