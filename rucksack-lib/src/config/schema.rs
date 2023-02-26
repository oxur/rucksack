use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Rucksack {
    pub cfg_dir: String,
    pub cfg_file: String,
    pub data_dir: String,
    pub db_file: String,
    pub name: String,
    pub backup_dir: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Config {
    pub logging: twyg::LoggerOpts,
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
        rucksack: Rucksack {
            ..Default::default()
        },
    }
}
