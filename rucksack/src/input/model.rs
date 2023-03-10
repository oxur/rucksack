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
    // TODO: for now, we're going to comment these out and explicitly state
    // that the DB is the source of truth for this. We need to address this
    // long-term, though ... see this ticket for context:
    // * https://github.com/oxur/rucksack/issues/92
    // pub data_dir: String,
    // pub db_file: String,
    // pub backup_dir: String,
}
