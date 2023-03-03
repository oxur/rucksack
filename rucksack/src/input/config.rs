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
    pub force: bool,
    pub in_memory: bool,
}

pub fn default_opts() -> Opts {
    Opts {
        ..Default::default()
    }
}

pub fn force_opts() -> Opts {
    Opts {
        force: true,
        ..Default::default()
    }
}

pub fn in_memory_opts() -> Opts {
    Opts {
        in_memory: true,
        ..Default::default()
    }
}

pub fn init(filename: String, opts: Opts) -> Result<()> {
    let file_path = file::create_parents(filename.clone())?;
    if file_path.exists() && !opts.force {
        log::debug!("File already exists; skipping init ...");
        return Ok(());
    }
    file::write(DEFAULT_TOML.as_bytes().to_vec(), filename)
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

pub fn load(config_file: String, log_level: String, name: String) -> Config {
    let defaults = defaults();
    match init(config_file.clone(), default_opts()) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    }
    match Confygery::new()
        .add_file(&config_file)
        .add_struct(&defaults)
        .build::<Config>()
    {
        Ok(mut cfg) => {
            if !log_level.is_empty() {
                cfg.logging.level = log_level;
            }
            cfg.rucksack.cfg_file = config_file;
            cfg.rucksack.name = name;
            cfg
        }
        Err(e) => panic!("{e}"),
    }
}

#[cfg(test)]
mod tests {
    // use crate::testing;

    #[test]
    fn in_memory_test() {}
}
