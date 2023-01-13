use anyhow::{anyhow, Result};
use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};

use crate::store;
use crate::store::db;
use crate::store::record;
use crate::store::record::DecryptedRecord;

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    match matches.get_one::<String>("db") {
        Some(db_file) => {
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

pub fn record(app_db: &db::DB, matches: &ArgMatches) -> Option<DecryptedRecord> {
    app_db.get(key(matches))
}

pub fn user(matches: &ArgMatches) -> String {
    matches.get_one::<String>("user").unwrap().to_string()
}

pub fn url(matches: &ArgMatches) -> String {
    matches.get_one::<String>("url").unwrap().to_string()
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
