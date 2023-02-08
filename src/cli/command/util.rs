use anyhow::{anyhow, Result};
use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};

use crate::store;
use crate::store::db;
use crate::store::records;
use crate::store::records::{new_tags, DecryptedRecord, Status, Tag};
use crate::util;

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
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
        None => secret("Enter db password: ").unwrap(),
    };
    let salt = matches.get_one::<String>("salt").unwrap().to_string();
    let db_file: String;
    match db {
        Some(file_path) => {
            log::debug!("Got database file from flag: {}", file_path);
            db_file = file_path.to_owned();
        }
        None => {
            db_file = util::db_file();
            log::debug!("No database flag provided; using default ({db_file:})");
        }
    }
    db::open(db_file, pwd.expose_secret().to_string(), salt)
}

pub fn record(app_db: &db::DB, matches: &ArgMatches) -> Result<DecryptedRecord> {
    record_by_key(app_db, key(matches))
}

pub fn record_by_key(app_db: &db::DB, key: String) -> Result<DecryptedRecord> {
    match app_db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            let msg = format!("no secret record for given key '{key}'");
            log::info!("{msg}");
            Err(anyhow!(msg))
        }
    }
}

pub fn remove(app_db: &db::DB, matches: &ArgMatches) -> Result<()> {
    remove_by_key(app_db, key(matches))
}

pub fn remove_by_key(app_db: &db::DB, key: String) -> Result<()> {
    log::debug!("Removing record associated with {} ...", key);
    match app_db.delete(key.clone()) {
        Some(true) => Ok(()),
        Some(false) => {
            let msg = format!("could not delete record with given key '{key}'");
            log::error!("{msg}");
            Err(anyhow!(msg))
        }
        None => unreachable!(),
    }
}

pub fn category(matches: &ArgMatches) -> String {
    matches.get_one::<String>("category").unwrap().to_string()
}

pub fn tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let values: Vec<String> = matches.get_many("tags")?.cloned().collect();
    Some(new_tags(values))
}

pub fn all_tags(matches: &ArgMatches) -> Option<Vec<String>> {
    matches
        .get_many("all-tags")
        .map(|x| x.cloned().collect::<Vec<String>>())
}

pub fn any_tags(matches: &ArgMatches) -> Option<Vec<String>> {
    matches
        .get_many("any-tags")
        .map(|x| x.cloned().collect::<Vec<String>>())
}

pub fn name(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("name") {
        Some(n) => n.to_string(),
        None => user(matches),
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

pub fn account_id(matches: &ArgMatches) -> String {
    matches.get_one::<String>("account-id").unwrap().to_string()
}

pub fn public(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("public")
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn private(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("private")
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn root(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("root")
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn service_key(matches: &ArgMatches) -> String {
    matches.get_one::<String>("key").unwrap().to_string()
}

pub fn service_secret(matches: &ArgMatches) -> String {
    matches.get_one::<String>("secret").unwrap().to_string()
}

pub fn key(matches: &ArgMatches) -> String {
    store::key(
        &category(matches),
        record_kind(matches),
        &user(matches),
        &url(matches),
    )
}

pub fn db_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("db-pass") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => secret("Enter DB password: ").unwrap(),
    }
}

pub fn record_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => secret("Enter record password: ").unwrap(),
    }
}

pub fn record_pwd_revealed(matches: &ArgMatches) -> String {
    reveal(record_pwd(matches))
}

pub fn record_state(matches: &ArgMatches) -> Status {
    match matches.get_one::<String>("status").map(|s| s.as_str()) {
        Some("active") => Status::Active,
        Some("inactive") => Status::Inactive,
        Some("deleted") => Status::Deleted,
        Some(&_) => todo!(),
        None => Status::Active,
    }
}

pub fn secret(prompt: &str) -> Result<SecretString> {
    rpassword::prompt_password(prompt)
        .map(SecretString::new)
        .map_err(|e| anyhow!("password prompt failed: {}", e.to_string()))
}

pub fn reveal(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}

pub fn record_kind(matches: &ArgMatches) -> records::Kind {
    let record_type = matches.get_one::<String>("type").map(|s| s.as_str());
    match record_type {
        Some("account") => records::Kind::Account, // Anything that has an account ID, e.g., AWS creds
        Some("asymmetric-crypto") => records::Kind::AsymmetricCrypto, // SSH, GPG, etc.
        Some("asymmetric") => records::Kind::AsymmetricCrypto, // Alias for 'asymmetric-crypto'
        Some("certificates") => records::Kind::Certificates, // public, private, root -- SSL, e.g.
        Some("certs") => records::Kind::Certificates, // Alias for 'certificates'
        Some("password") => records::Kind::Password, // standard username/password
        Some("service-creds") => records::Kind::ServiceCredentials, // Alias for service-creds
        Some("service-credentials") => records::Kind::ServiceCredentials, // API key/secret pairs, e.g.
        Some("any") => records::Kind::Any,
        Some("") => records::Kind::default(),
        Some(&_) => todo!(),
        None => records::Kind::default(),
    }
}
