use anyhow::{anyhow, Result};
use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};

use crate::store;
use crate::store::db;
use crate::store::record;
use crate::store::record::DecryptedRecord;

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    let db = matches.get_one::<String>("db");
    match matches.get_one::<bool>("db-needed") {
        Some(false) => {
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
    match db {
        Some(db_file) => {
            log::debug!("Got database file from flag: {}", db_file);
            let pwd = match matches.get_one::<String>("db-pass") {
                Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
                None => secret("Enter db password: ").unwrap(),
            };
            let salt = matches.get_one::<String>("salt").unwrap().to_string();
            db::open(db_file.to_owned(), pwd.expose_secret().to_string(), salt)
        }
        None => Ok(db::new()),
    }
}

pub fn record(app_db: &db::DB, matches: &ArgMatches) -> Result<DecryptedRecord> {
    record_by_key(app_db, key(matches))
}

pub fn record_by_key(app_db: &db::DB, key: String) -> Result<DecryptedRecord> {
    match app_db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            let msg = format!("no secret record for given key '{}'", key);
            log::info!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

pub fn remove(app_db: &db::DB, matches: &ArgMatches) -> Result<()> {
    remove_by_key(app_db, key(matches))
}

pub fn remove_by_key(app_db: &db::DB, key: String) -> Result<()> {
    match app_db.delete(key.clone()) {
        Some(true) => Ok(()),
        Some(false) => {
            let msg = format!("could not delete record with given key '{}'", key);
            log::error!("{}", msg);
            Err(anyhow!(msg))
        }
        None => unreachable!(),
    }
}

pub fn user(matches: &ArgMatches) -> String {
    matches.get_one::<String>("user").unwrap().to_string()
}

pub fn user_old(matches: &ArgMatches) -> String {
    matches.get_one::<String>("old-user").unwrap().to_string()
}

pub fn user_new(matches: &ArgMatches) -> String {
    matches.get_one::<String>("new-user").unwrap().to_string()
}

pub fn url(matches: &ArgMatches) -> String {
    matches.get_one::<String>("url").unwrap().to_string()
}

pub fn url_old(matches: &ArgMatches) -> String {
    matches.get_one::<String>("old-url").unwrap().to_string()
}

pub fn url_new(matches: &ArgMatches) -> String {
    matches.get_one::<String>("new-url").unwrap().to_string()
}

pub fn key(matches: &ArgMatches) -> String {
    store::key(&user(matches), &url(matches))
}

pub fn db_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("db-pass") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => secret("Enter DB password: ").unwrap(),
    }
}

pub fn account_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => secret("Enter account password: ").unwrap(),
    }
}

pub fn account_pwd_revealed(matches: &ArgMatches) -> String {
    reveal(account_pwd(matches))
}

pub fn secret(prompt: &str) -> Result<SecretString> {
    rpassword::prompt_password(prompt)
        .map(SecretString::new)
        .map_err(|e| anyhow!("password prompt failed: {}", e.to_string()))
}

pub fn reveal(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}

pub fn account_kind(matches: &ArgMatches) -> record::Kind {
    let account_type = matches.get_one::<String>("type").map(|s| s.as_str());
    match account_type {
        Some("account") => record::Kind::Account,
        Some("creds") => record::Kind::Credential,
        Some("credential") => record::Kind::Credential,
        Some("password") => record::Kind::Password,
        Some("") => record::DEFAULT_KIND,
        Some(&_) => todo!(),
        None => record::DEFAULT_KIND,
    }
}
