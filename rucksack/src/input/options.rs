use std::path::PathBuf;

use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};

use rucksack_db as store;
use rucksack_db::records;
use rucksack_db::records::{new_tags, Status, Tag};

use super::prompt;

pub fn account_id(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("account-id")
        .unwrap()
        .trim()
        .to_string()
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

pub fn backup_name(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("name") {
        Some(n) => n.trim().to_string(),
        None => "".to_string(),
    }
}

pub fn backup_dir(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("backup-dir") {
        Some(d) => d.trim().to_string(),
        None => "".to_string(),
    }
}

pub fn backup_path(matches: &ArgMatches) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(backup_dir(matches));
    path
}

pub fn category(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("category")
        .unwrap()
        .trim()
        .to_string()
}

pub fn config_file(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("config-file").cloned() {
        Some(file) => file,
        None => "".to_string(),
    }
}

pub fn db(matches: &ArgMatches) -> Option<String> {
    matches.get_one::<String>("db").cloned()
}

pub fn db_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("db-pass") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter DB password: ").unwrap(),
    }
}

pub fn key(matches: &ArgMatches) -> String {
    store::key(
        &category(matches),
        record_kind(matches),
        &user(matches),
        &url(matches),
    )
}

pub fn latest(matches: &ArgMatches) -> bool {
    *matches.get_one::<bool>("latest").unwrap_or(&false)
}

pub fn log_level(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("log-level").cloned() {
        Some(level) => level,
        None => "".to_string(),
    }
}

pub fn name(matches: &ArgMatches) -> String {
    match matches.get_one::<String>("name") {
        Some(n) => n.to_string(),
        None => user(matches),
    }
}

pub fn private(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("private")
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn public(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("public")
        .unwrap()
        .as_bytes()
        .to_vec()
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

pub fn record_pwd(matches: &ArgMatches) -> Secret<String> {
    match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter record password: ").unwrap(),
    }
}

pub fn record_pwd_revealed(matches: &ArgMatches) -> String {
    reveal_it(record_pwd(matches))
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

pub fn reveal(matches: &ArgMatches) -> bool {
    *matches.get_one::<bool>("reveal").unwrap_or(&false)
}

fn reveal_it(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}

pub fn root(matches: &ArgMatches) -> Vec<u8> {
    matches
        .get_one::<String>("root")
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn service_key(matches: &ArgMatches) -> String {
    matches.get_one::<String>("key").unwrap().trim().to_string()
}

pub fn service_secret(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("secret")
        .unwrap()
        .trim()
        .to_string()
}

pub fn tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let values: Vec<String> = matches.get_many("tags")?.cloned().collect();
    Some(new_tags(values))
}

pub fn url(matches: &ArgMatches) -> String {
    matches.get_one::<String>("url").unwrap().trim().to_string()
}

pub fn url_old(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("old-url")
        .unwrap()
        .trim()
        .to_string()
}

pub fn url_new(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("new-url")
        .unwrap()
        .trim()
        .to_string()
}

pub fn user(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("user")
        .unwrap()
        .trim()
        .to_string()
}

pub fn user_new(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("new-user")
        .unwrap()
        .trim()
        .to_string()
}

pub fn user_old(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("old-user")
        .unwrap()
        .trim()
        .to_string()
}
