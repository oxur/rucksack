use std::str;

use anyhow::Result;
use confyg::Confygery;
use serde::{Deserialize, Serialize};

use rucksack_lib::file;

const DEFAULT_TOML: &str = r#"[rucksack]

[logging]
coloured = true
level = "error"
report_caller = true

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

pub fn init(opts: &Opts) -> Result<()> {
    let file_path = file::create_parents(opts.file_name.clone())?;
    if file_path.exists() && !opts.force {
        log::debug!("File already exists; skipping init ...");
        return Ok(());
    }
    file::write(DEFAULT_TOML.as_bytes().to_vec(), opts.file_name.clone())
}

pub fn load(opts: &Opts) -> Result<Config> {
    match init(opts) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    }
    let mut cfg: Config;
    if opts.in_memory {
        cfg = Confygery::new()
            .add_str(&opts.config)
            .add_struct(&defaults())
            .build::<Config>()?;
    } else {
        cfg = Confygery::new()
            .add_file(&opts.file_name)
            .add_struct(&defaults())
            .build::<Config>()?;
    }
    if !opts.log_level.is_empty() {
        cfg.logging.level = opts.log_level.clone();
    }
    cfg.rucksack.cfg_file = opts.file_name.clone();
    cfg.rucksack.name = opts.name.clone();
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    // use crate::testing;

    #[test]
    fn in_memory_test() {}
}
