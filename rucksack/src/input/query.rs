use anyhow::{anyhow, Result};
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use rucksack_db::db;
use rucksack_db::records::DecryptedRecord;

use crate::input::options;

pub fn record(app_db: &db::DB, matches: &ArgMatches) -> Result<DecryptedRecord> {
    record_by_key(app_db, options::key(matches))
}

pub fn record_by_key(app_db: &db::DB, key: String) -> Result<DecryptedRecord> {
    log::debug!("Querying record by key '{key}' ...");
    match app_db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            let msg = format!("No secret record for given key '{key}'");
            log::info!("{msg}");
            Err(anyhow!(msg))
        }
    }
}

pub fn remove(app_db: &db::DB, matches: &ArgMatches) -> Result<()> {
    remove_by_key(app_db, options::key(matches))
}

pub fn remove_by_key(app_db: &db::DB, key: String) -> Result<()> {
    log::debug!("Removing record associated with {} ...", key);
    match app_db.delete(key.clone()) {
        Some(true) => Ok(()),
        Some(false) => {
            let msg = format!("Could not delete record with given key '{key}'");
            log::error!("{msg}");
            Err(anyhow!(msg))
        }
        None => unreachable!(),
    }
}

pub fn reveal(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}
