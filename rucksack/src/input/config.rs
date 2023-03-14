use std::str;

use anyhow::Result;
use confyg::Confygery;
use serde::{Deserialize, Serialize};

use rucksack_lib::file;

use super::{constant, model};

const DEFAULT: &str = r#"[rucksack]

[db]

[generation]

[generation.defaults]
gen_type = "uuid++"
length = 12
suffix_length = 4
word_count = 4
delimiter = "-"
max_score = 100
min_score = 0
sort_by = "url"

[logging]
coloured = true
level = "error"
report_caller = true

[records]

[records.defaults]
new_category = "default"
list_category = "any"
kind = "password"
status = "active"

[retention]
purge_on_shutdown = false
archive_deletes = true
delete_inactive = false

[output]
show_inactive = true
show_deleted = false
"#;

#[derive(Clone, Default)]
pub struct Opts {
    config: String,
    file_name: String,
    force: bool,
    in_memory: bool,
    log_level: String,
    name: String,
}

// These opts are used for bootstrapping the config
impl Opts {
    pub fn new() -> Opts {
        Opts {
            ..Default::default()
        }
    }

    pub fn config(&mut self, data: String) -> &mut Opts {
        self.config = data;
        self
    }

    pub fn file_name(&mut self, name: String) -> &mut Opts {
        self.file_name = name;
        self
    }

    pub fn force(&mut self) -> &mut Opts {
        self.force = true;
        self
    }

    pub fn in_memory(&mut self) -> &mut Opts {
        self.in_memory = true;
        self
    }

    pub fn log_level(&mut self, level: String) -> &mut Opts {
        self.log_level = level;
        self
    }

    pub fn name(&mut self, name: String) -> &mut Opts {
        self.name = name;
        self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct DbConfig {
    pub path: String,
    pub data_dir: String,
    pub backup_dir: String,
}

impl DbConfig {
    pub fn to_db(&self) -> model::Db {
        let mut db = model::Db::default();
        if !self.path.is_empty() {
            db.path = self.path.clone();
        }
        if !self.data_dir.is_empty() {
            db.data_dir = self.data_dir.clone();
        }
        if !self.backup_dir.is_empty() {
            db.backup_dir = self.backup_dir.clone();
        }
        db
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Config {
    pub db: DbConfig,
    pub generation: model::Generation,
    pub logging: model::Logging,
    pub records: model::Records,
    pub retention: model::Retention,
    pub rucksack: model::Rucksack,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db: DbConfig {
                ..Default::default()
            },
            generation: model::Generation {
                ..Default::default()
            },
            logging: model::Logging::new(),
            records: model::Records {
                ..Default::default()
            },
            retention: model::Retention {
                ..Default::default()
            },
            rucksack: model::Rucksack {
                ..Default::default()
            },
        }
    }
}

impl Config {
    pub fn init(opts: &Opts) -> Result<()> {
        let file_path = file::create_parents(opts.file_name.clone())?;
        if file_path.exists() && !opts.force {
            log::debug!("File already exists; skipping init ...");
            return Ok(());
        }
        file::write(DEFAULT.as_bytes().to_vec(), opts.file_name.clone())
    }

    pub fn new(opts: &Opts) -> Result<Config> {
        let defaults = Self::default();
        let cfg: Config = if opts.in_memory {
            Confygery::new()
                .add_str(&opts.config)
                .add_struct(&defaults)
                .build::<Config>()?
        } else {
            Confygery::new()
                .add_file(&opts.file_name)
                .add_struct(&defaults)
                .build::<Config>()?
        };
        Ok(cfg)
    }

    pub fn load(opts: &Opts) -> Result<Config> {
        match Self::init(opts) {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }

        let mut cfg = Self::new(opts)?;
        cfg.logging.level = constant::DEFAULT_LOG_LEVEL.to_string();
        if !opts.log_level.is_empty() {
            cfg.logging.level = opts.log_level.clone();
        }
        twyg::setup_logger(&cfg.logging.to_twyg())?;
        cfg.rucksack.cfg_file = opts.file_name.clone();
        log::debug!("Config setup complete (using {})", cfg.rucksack.cfg_file);
        cfg.rucksack.name = opts.name.clone();
        log::debug!("Logger setup complete");
        Ok(cfg)
    }

    pub fn to_inputs(&self) -> model::Inputs {
        model::Inputs {
            db: self.db.to_db(),
            generation: self.generation.clone(),
            logging: self.logging.clone(),
            records: self.records.clone(),
            retention: self.retention.clone(),
            rucksack: self.rucksack.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::testing;
    #[test]
    fn in_memory_test() {
        let r = super::Config::load(&super::Opts {
            in_memory: true,
            config: super::DEFAULT.to_string(),
            ..Default::default()
        });
        assert!(r.is_ok());
        assert!(!r.unwrap().retention.purge_on_shutdown);
    }

    #[test]
    fn in_memory_purge_test() {
        let r = super::Config::load(&super::Opts {
            in_memory: true,
            config: testing::configs::PURGE_TOML.to_string(),
            ..Default::default()
        });
        assert!(r.is_ok());
        assert!(r.unwrap().retention.purge_on_shutdown);
    }

    #[test]
    fn in_memory_inactive_test() {
        let r = super::Config::load(&super::Opts {
            in_memory: true,
            config: testing::configs::DELETE_INACTIVE_TOML.to_string(),
            ..Default::default()
        });
        assert!(r.is_ok());
        assert!(r.unwrap().retention.delete_inactive);
    }
}
