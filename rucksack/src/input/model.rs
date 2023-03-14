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
use serde::{Deserialize, Serialize};
use secrecy::{Secret, SecretString};

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
    pub defaults: GenDefaults
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
    pub defaults: RecordDefaults
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct RecordDefaults {
    pub new_category: String,
    pub list_category: String,
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
    pub cfg_dir: String,
    pub cfg_file: String,
    pub name: String,
}
