use std::path;

use anyhow::Result;
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use rucksack_db as store;
use rucksack_lib::file;

use crate::handlers::{add, backup, config, export, gen, import, list, rm, set, show};
use crate::input::{constant, options, prompt, Config};

#[derive(Debug)]
pub struct App {
    pub cfg: Config,
    pub db: store::db::DB,
}

pub fn new(cfg: Config, matches: &ArgMatches) -> Result<App> {
    log::debug!("Setting up rucksack application ...");
    let db = setup_db(matches)?;
    Ok(App { cfg, db })
}

impl App {
    pub fn backup_dir(&self) -> String {
        self.db.backup_dir()
    }

    pub fn backup_path(&self) -> path::PathBuf {
        let mut path = path::PathBuf::new();
        path.push(self.backup_dir());
        path
    }

    pub fn config_dir(&self) -> String {
        self.config_path().display().to_string()
    }

    pub fn config_file(&self) -> String {
        if self.cfg.rucksack.cfg_file != *"" {
            return self.cfg.rucksack.cfg_file.clone();
        }
        file::config_file(&self.name())
    }

    pub fn config_path(&self) -> path::PathBuf {
        if self.cfg.rucksack.cfg_dir != *"" {
            let mut path = path::PathBuf::new();
            path.push(self.cfg.rucksack.cfg_dir.clone());
            return path;
        }
        file::config_dir(&self.name())
    }

    pub fn data_dir(&self) -> String {
        self.data_path().display().to_string()
    }

    pub fn data_path(&self) -> path::PathBuf {
        self.db_path().parent().unwrap().to_path_buf()
    }

    pub fn db_file(&self) -> String {
        self.db.file_name.clone()
    }

    pub fn db_path(&self) -> path::PathBuf {
        let mut path = path::PathBuf::new();
        path.push(self.db_file());
        path
    }

    pub fn db_version(&self) -> versions::SemVer {
        self.db.version()
    }

    pub fn name(&self) -> String {
        self.cfg.rucksack.name.clone()
    }

    pub fn run(&self, matches: &ArgMatches) -> Result<()> {
        log::info!("Executing rucksack command ...");
        if !self.backup_path().exists() {
            log::debug!("Checking for backup dir {:?} ...", self.backup_dir());
            file::create_dirs(self.backup_path())?;
            log::info!("Created backup dir.");
        }
        match matches.subcommand() {
            Some(("add", add_matches)) => add::new(add_matches, self)?,
            Some(("backup", backup_matches)) => match backup_matches.subcommand() {
                Some(("delete", delete_matches)) => backup::delete(delete_matches, self)?,
                Some(("restore", restore_matches)) => backup::restore(restore_matches, self)?,
                Some((&_, _)) => todo!(),
                None => backup::run(backup_matches, self)?,
            },
            Some(("backups", backup_matches)) => match backup_matches.subcommand() {
                Some(("list", list_matches)) => backup::list(list_matches, self)?,
                Some((&_, _)) => todo!(),
                None => todo!(),
            },
            Some(("config", config_matches)) => match config_matches.subcommand() {
                Some(("re-init", init_matches)) => config::re_init(init_matches, self)?,
                Some((&_, _)) => todo!(),
                None => todo!(),
            },
            Some(("export", export_matches)) => export::new(export_matches, self)?,
            Some(("gen", gen_matches)) => gen::new(gen_matches)?,
            Some(("import", import_matches)) => import::new(import_matches, self)?,
            Some(("list", list_matches)) => match list_matches.subcommand() {
                Some(("backups", backups_matches)) => list::backups(backups_matches, self)?,
                Some(("deleted", deleted_matches)) => list::deleted(deleted_matches, self)?,
                Some(("keys", key_matches)) => list::keys(key_matches, self)?,
                Some(("passwords", passwords_matches)) => list::passwords(passwords_matches, self)?,
                Some((&_, _)) => todo!(),
                None => list::all(list_matches, self)?,
            },
            Some(("rm", rm_matches)) => rm::one(rm_matches, self)?,
            Some(("set", set_matches)) => match set_matches.subcommand() {
                Some(("password", password_matches)) => set::password(password_matches, self)?,
                Some(("status", status_matches)) => set::status(status_matches, self)?,
                Some(("url", url_matches)) => set::url(url_matches, self)?,
                Some(("user", user_matches)) => set::user(user_matches, self)?,
                Some(("type", type_matches)) => set::record_type(type_matches, self)?,
                Some((&_, _)) => todo!(),
                None => todo!(),
            },
            Some(("show", show_matches)) => match show_matches.subcommand() {
                Some(("backup-dir", bud_matches)) => show::backup_dir(bud_matches, self)?,
                Some(("categories", cat_matches)) => show::categories(cat_matches, self)?,
                Some(("config-file", cfgfile_matches)) => show::config_file(cfgfile_matches, self)?,
                Some(("config", cfg_matches)) => show::config(cfg_matches, self)?,
                Some(("data-dir", datadir_matches)) => show::data_dir(datadir_matches, self)?,
                Some(("db-file", dbfile_matches)) => show::db_file(dbfile_matches, self)?,
                Some(("db-version", dbvsn_matches)) => show::db_version(dbvsn_matches, self)?,
                Some(("tags", tag_matches)) => show::tags(tag_matches, self)?,
                Some(("types", type_matches)) => show::types(type_matches, self)?,
                Some((&_, _)) => todo!(),
                None => todo!(),
            },
            Some((cmd, _)) => {
                log::warn!("unknown command: {}", cmd);
                todo!()
            }
            None => todo!(),
        }
        log::debug!("Command execution complete.");
        Ok(())
    }

    pub fn shutdown(&self, _matches: &ArgMatches) -> Result<()> {
        log::info!("Performing shutdown operations ...");
        if self.cfg.retention.purge_on_shutdown {
            todo!();
        }
        if self.cfg.retention.delete_inactive {
            // TODO: iterate through all inactive records and flag them as deleted
            todo!();
        }
        Ok(())
    }
}

pub fn setup_db(matches: &ArgMatches) -> Result<store::db::DB> {
    log::debug!("Setting up database ...");
    let db_file = match options::db(matches) {
        Some(file_path) => {
            log::debug!("Got database file from flag: {}", file_path);
            file_path
        }
        None => {
            let file_name = file::db_file(constant::NAME);
            log::debug!("No database flag provided; using default ({file_name:})");
            file_name
        }
    };
    let mut backup_dir = options::backup_dir(matches);
    if backup_dir.is_empty() {
        let dir_path = file::backup_dir(constant::NAME);
        backup_dir = dir_path.display().to_string();
        log::debug!("No backup dir flag provided; using default");
    };
    log::debug!("Got backup dir {backup_dir:}");
    match matches.get_one::<bool>("db-needed") {
        Some(false) => {
            log::debug!("Database not needed for this command; skipping load ...");
            return Ok(store::db::new(db_file, backup_dir));
        }
        Some(true) => (),
        None => (),
    }
    log::debug!("Database is needed; preparing for read ...");
    let pwd = match matches.get_one::<String>("db-pass") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter db password: ").unwrap(),
    };
    let salt = matches.get_one::<String>("salt").unwrap().to_string();
    store::db::open(db_file, backup_dir, pwd.expose_secret().to_string(), salt)
}
