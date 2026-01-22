//! # Input Data Model
//!
//! This is the data model that unifies all input to the application. Possible
//! sources of input include:
//!
//! * ENV variables
//! * CLI flags / options / args
//! * Values in a config file
//! * CLI parser defaults
//! * Module-level consts/defaults
//! * Library-level consts/defaults
//!
//! The ordering of this list represents the order precedence for these as
//! well, from highest priority to lowest priority.
//!
use std::env;
use std::path;

use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};
use serde::{Deserialize, Serialize};

use rucksack_db::{records, Tag};
use rucksack_lib::file;

use super::{constant, options};

pub enum Flag {
    One,
    Many,
}

#[derive(Clone, Debug, Default)]
pub struct Inputs {
    pub db: Db,
    pub generation: Generation,
    pub logging: Logging,
    pub records: Records,
    pub retention: Retention,
    pub rucksack: Rucksack,
    pub matches: ArgMatches,
}

// The methods of Inputs are focused on one main goal: consolidate ENV vars,
// CLI opts, configuration, and statically defined defaults, presenting a
// single API (source of truth) whereby the rest of the app may come to get
// what it needs.
impl Inputs {
    pub fn account_id(&self) -> String {
        options::account_id(&self.matches)
    }

    pub fn backup_dir(&self) -> String {
        let mut dir = options::backup_dir(&self.matches);
        if !dir.is_empty() {
            log::debug!(dir = dir.as_str(), source = "flag", operation = "get_backup_dir"; "Got backup dir from flag");
            return dir;
        }
        dir = self.db.backup_dir.clone();
        if !dir.is_empty() {
            log::debug!(dir = dir.as_str(), source = "config", operation = "get_backup_dir"; "No database flag provided; using configured file");
            return dir;
        }
        dir = file::backup_dir(constant::NAME).display().to_string();
        log::debug!(dir = dir.as_str(), source = "default", operation = "get_backup_dir"; "No configured database file; using default");
        dir
    }

    pub fn category(&self, flag: Flag) -> String {
        match options::category(&self.matches) {
            Some(c) => c.trim().to_owned(),
            None => match flag {
                Flag::One => {
                    let c = self.records.defaults.new_category.trim().to_owned();
                    if !c.is_empty() {
                        return c;
                    }
                    records::DEFAULT_CATEGORY.to_string()
                }
                Flag::Many => {
                    let c = self.records.defaults.list_category.trim().to_owned();
                    if !c.is_empty() {
                        return c;
                    }
                    records::ANY_CATEGORY.to_string()
                }
            },
        }
    }

    pub fn config_file(&self) -> path::PathBuf {
        let mut cf = options::config_file(&self.matches);
        if !cf.is_empty() {
            return path::PathBuf::from(cf);
        }
        cf = self.rucksack.cfg_file.clone();
        if !cf.is_empty() {
            return path::PathBuf::from(cf);
        }
        file::config_file(constant::NAME)
    }

    pub fn db_file(&self) -> path::PathBuf {
        match options::db(&self.matches) {
            Some(file_name) => {
                log::debug!(file = file_name.as_str(), source = "flag", operation = "get_db_file"; "Got database file from flag");
                path::PathBuf::from(file_name)
            }
            None => {
                let db_file_str = self.db.path.clone();
                if !db_file_str.is_empty() {
                    log::debug!(file = db_file_str.as_str(), source = "config", operation = "get_db_file"; "No database flag provided; using configured file");
                    return path::PathBuf::from(db_file_str);
                }
                let db_file = file::db_file(constant::NAME);
                log::debug!(file = db_file.to_string_lossy().as_ref(), source = "default", operation = "get_db_file"; "No configured database file; using default");
                db_file
            }
        }
    }

    pub fn db_needed(&self) -> bool {
        options::db_needed(&self.matches).unwrap_or(true)
    }

    pub fn db_passwd(&self) -> String {
        options::db_pwd(&self.matches).expose_secret().to_string()
    }

    pub fn key(&self) -> String {
        records::key(
            &self.category(Flag::One),
            options::record_kind(&self.matches),
            &options::user(&self.matches),
            &options::url(&self.matches),
        )
    }

    pub fn name(&self) -> String {
        options::name(&self.matches)
    }

    pub fn private(&self) -> Vec<u8> {
        options::private(&self.matches)
    }

    pub fn public(&self) -> Vec<u8> {
        options::public(&self.matches)
    }

    pub fn record_kind(&self) -> records::Kind {
        options::record_kind(&self.matches)
    }

    pub fn record_passwd(&self) -> String {
        options::record_pwd_revealed(&self.matches)
    }

    pub fn root(&self) -> Vec<u8> {
        options::root(&self.matches)
    }

    pub fn salt(&self) -> String {
        match options::salt(&self.matches) {
            Some(s) => s,
            None => match env::var(constant::SALT_ENV) {
                Ok(user) => user,
                Err(_) => constant::SALT_FALLBACK.to_string(),
            },
        }
    }

    pub fn service_key(&self) -> String {
        options::service_key(&self.matches)
    }

    pub fn service_secret(&self) -> String {
        options::service_secret(&self.matches)
    }

    pub fn tags(&self) -> Option<Vec<Tag>> {
        options::tags(&self.matches)
    }

    pub fn user(&self) -> String {
        options::user(&self.matches)
    }

    pub fn url(&self) -> String {
        options::url(&self.matches)
    }
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Db {
    pub path: String,
    pub data_dir: String,
    pub backup_dir: String,
    pub defaults: DbDefaults,
    pub secrets: DbSecrets,
}

impl Default for Db {
    fn default() -> Self {
        Db {
            path: String::new(),
            data_dir: String::new(),
            backup_dir: String::new(),
            defaults: DbDefaults {
                ..Default::default()
            },
            secrets: DbSecrets {
                ..Default::default()
            },
        }
    }
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct DbSecrets {
    pub password: Secret<String>,
    pub salt: Secret<String>,
}

impl Default for DbSecrets {
    fn default() -> Self {
        DbSecrets {
            password: SecretString::new(String::new()),
            salt: SecretString::new(String::new()),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[allow(unused)]
pub struct DbDefaults {
    pub serialisation_format: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Generation {
    pub defaults: GenDefaults,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct GenDefaults {
    pub gen_type: String,
    pub length: u16,
    pub suffix_length: u8,
    pub word_count: u8,
    pub delimiter: String,
    pub max_score: u16,
    pub min_score: u16,
    pub sort_by: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Logging {
    pub coloured: bool,
    pub file: Option<String>,
    pub level: String,
    pub report_caller: bool,
}

impl Logging {
    pub fn new() -> Logging {
        Logging {
            coloured: true,
            file: None,
            level: "error".to_string(),
            report_caller: true,
        }
    }

    pub fn to_twyg(&self) -> Result<twyg::Opts, String> {
        // Parse level string into LogLevel enum
        let level: twyg::LogLevel = self
            .level
            .parse()
            .map_err(|_| format!("Invalid log level: {}", self.level))?;

        // Convert file Option<String> to Output enum
        let output = match &self.file {
            Some(path) => twyg::Output::file(path),
            None => twyg::Output::Stdout,
        };

        // Build opts using OptsBuilder
        twyg::OptsBuilder::new()
            .coloured(self.coloured)
            .output(output)
            .level(level)
            .report_caller(self.report_caller)
            .build()
            .map_err(|e| format!("Failed to build twyg opts: {:?}", e))
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Records {
    pub defaults: RecordDefaults,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct RecordDefaults {
    pub new_category: String,
    pub list_category: String,
    // TODO: there is currently no unification between these and the related
    // methods in input::options ...
    pub kind: String,
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Retention {
    pub purge_on_shutdown: bool,
    pub archive_deletes: bool,
    pub delete_inactive: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Rucksack {
    pub cfg_file: String,
    pub name: String,
}
