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

use clap::ArgMatches;
use secrecy::{ExposeSecret, Secret, SecretString};
use serde::{Deserialize, Serialize};

use rucksack_db::records;
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
    pub fn backup_dir(&self) -> String {
        let mut dir = options::backup_dir(&self.matches);
        if !dir.is_empty() {
            log::debug!("Got backup dir from flag: {}", dir);
            return dir;
        }
        dir = self.db.backup_dir.clone();
        if !dir.is_empty() {
            log::debug!(
                "No database flag provided; using configured file ({:})",
                dir
            );
            return dir;
        }
        dir = file::backup_dir(constant::NAME).display().to_string();
        log::debug!("No configured database file; using default ({:})", dir);
        dir
    }

    pub fn category(&self, flag: Flag) -> String {
        match options::category(&self.matches) {
            Some(c) => {
                return c.trim().to_owned();
            }
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

    pub fn config_file(&self) -> String {
        let mut cf = options::config_file(&self.matches);
        if !cf.is_empty() {
            return cf;
        }
        cf = self.rucksack.cfg_file.clone();
        if !cf.is_empty() {
            return cf;
        }
        file::config_file(constant::NAME)
    }

    pub fn db_file(&self) -> String {
        match options::db(&self.matches) {
            Some(file_name) => {
                log::debug!("Got database file from flag: {}", file_name);
                file_name
            }
            None => {
                let mut db_file = self.db.path.clone();
                if !db_file.is_empty() {
                    log::debug!(
                        "No database flag provided; using configured file ({:})",
                        db_file
                    );
                    return db_file;
                }
                db_file = file::db_file(constant::NAME);
                log::debug!("No configured database file; using default ({:})", db_file);
                db_file
            }
        }
    }

    pub fn db_needed(&self) -> bool {
        options::db_needed(&self.matches).unwrap_or(false)
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

    pub fn salt(&self) -> String {
        match options::salt(&self.matches) {
            Some(s) => s,
            None => match env::var(constant::SALT_ENV) {
                Ok(user) => user,
                Err(_) => constant::SALT_FALLBACK.to_string(),
            },
        }
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

    pub fn to_twyg(&self) -> twyg::LoggerOpts {
        twyg::LoggerOpts {
            coloured: self.coloured,
            file: self.file.clone(),
            level: self.level.clone(),
            report_caller: self.report_caller,
        }
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
