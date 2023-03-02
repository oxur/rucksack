use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Retention {
    pub purge_on_shutdown: bool,
    pub archive_deletes: bool,
    pub delete_inactive: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Config {
    pub logging: twyg::LoggerOpts,
    pub retention: Retention,
    pub rucksack: Rucksack,
}

pub fn defaults() -> Config {
    Config {
        logging: twyg::LoggerOpts {
            coloured: true,
            file: None,
            level: "error".to_string(),
            report_caller: true,
        },
        retention: Retention {
            ..Default::default()
        },
        rucksack: Rucksack {
            ..Default::default()
        },
    }
}
