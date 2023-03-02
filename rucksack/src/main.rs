use std::io;

use anyhow::{Context, Result};
use clap::ArgMatches;

use rucksack::command as cli;
use rucksack::command::{add, backup, config, export, gen, import, list, rm, set, show};
use rucksack::constant;
use rucksack::setup;
use rucksack_lib::{file, util};

fn run(matches: &ArgMatches, app: &rucksack::App) -> Result<()> {
    log::debug!("Dispatching based upon (sub)command ...");
    let backup_path = app.backup_path();
    if !backup_path.exists() {
        log::debug!("Checking for backup dir {:?} ...", app.backup_dir());
        file::create_dirs(backup_path)?;
        log::info!("Created backup dir.");
    }
    match matches.subcommand() {
        Some(("add", add_matches)) => add::new(add_matches, app)?,
        Some(("backup", backup_matches)) => match backup_matches.subcommand() {
            Some(("delete", delete_matches)) => backup::delete(delete_matches, app)?,
            Some(("restore", restore_matches)) => backup::restore(restore_matches, app)?,
            Some((&_, _)) => todo!(),
            None => backup::run(backup_matches, app)?,
        },
        Some(("backups", backup_matches)) => match backup_matches.subcommand() {
            Some(("list", list_matches)) => backup::list(list_matches, app)?,
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("init", init_matches)) => config::init(init_matches, app)?,
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("export", export_matches)) => export::new(export_matches, app)?,
        Some(("gen", gen_matches)) => gen::new(gen_matches)?,
        Some(("import", import_matches)) => import::new(import_matches, app)?,
        Some(("list", list_matches)) => match list_matches.subcommand() {
            Some(("backups", backups_matches)) => list::backups(backups_matches, app)?,
            Some(("deleted", deleted_matches)) => list::deleted(deleted_matches, app)?,
            Some(("keys", key_matches)) => list::keys(key_matches, app)?,
            Some(("passwords", passwords_matches)) => list::passwords(passwords_matches, app)?,
            Some((&_, _)) => todo!(),
            None => list::all(list_matches, app)?,
        },
        Some(("rm", rm_matches)) => rm::one(rm_matches, app)?,
        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("password", password_matches)) => set::password(password_matches, app)?,
            Some(("status", status_matches)) => set::status(status_matches, app)?,
            Some(("url", url_matches)) => set::url(url_matches, app)?,
            Some(("user", user_matches)) => set::user(user_matches, app)?,
            Some(("type", type_matches)) => set::record_type(type_matches, app)?,
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("show", show_matches)) => match show_matches.subcommand() {
            Some(("backup-dir", bud_matches)) => show::backup_dir(bud_matches, app)?,
            Some(("categories", cat_matches)) => show::categories(cat_matches, app)?,
            Some(("config-file", cfgfile_matches)) => show::config_file(cfgfile_matches, app)?,
            Some(("config", cfg_matches)) => show::config(cfg_matches, app)?,
            Some(("data-dir", datadir_matches)) => show::data_dir(datadir_matches, app)?,
            Some(("db-file", dbfile_matches)) => show::db_file(dbfile_matches, app)?,
            Some(("db-version", dbvsn_matches)) => show::db_version(dbvsn_matches, app)?,
            Some(("tags", tag_matches)) => show::tags(tag_matches, app)?,
            Some(("types", type_matches)) => show::types(type_matches, app)?,
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

fn shutdown(_matches: &ArgMatches, app: &rucksack::App) -> Result<()> {
    log::debug!("Performing shutdown operations ...");
    if app.cfg.retention.purge_on_shutdown {
        todo!();
    }
    if app.cfg.retention.delete_inactive {
        // TODO: iterate through all inactive records and flag them as deleted
        todo!();
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut rucksack = cli::setup();
    let matches = rucksack.clone().get_matches();
    let mut config_file = String::new();
    if let Some(cfg_file) = matches.get_one::<String>("config-file") {
        config_file = cfg_file.to_string();
    }
    let mut log_level = String::new();
    if let Some(level) = matches.get_one::<String>("log-level") {
        log_level = level.to_string();
    }
    let cfg = rucksack_lib::config::load(config_file, log_level, constant::NAME.to_string());
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => {
            panic!("Could not setup logger: {error:?}")
        }
    }
    log::debug!("Config setup complete (using {})", cfg.rucksack.cfg_file);
    log::debug!("Logger setup complete");

    // Shell completion generation is completely independent, so perform it before
    // any config or subcommand operations.
    if let Some(is_version) = matches.get_one::<bool>("version") {
        if *is_version {
            return util::display(rucksack::version().to_string().as_str());
        }
    } else if let Some(shell) = matches.get_one::<clap_complete::Shell>("completions") {
        clap_complete::generate(*shell, &mut rucksack, cfg.rucksack.name, &mut io::stdout());
        return Ok(());
    }

    if matches.subcommand().is_none() {
        return rucksack
            .clone()
            .print_long_help()
            .with_context(|| "failed to print help".to_string());
    }

    let (_, subcmd_matches) = matches.subcommand().unwrap();
    log::debug!("Setting up database ...");
    let db = setup::db(subcmd_matches)?;
    // TODO: let's decide what the SoT should be for data in the running app,
    // and then consolidate all data there (from CLI flags, env vars, config,
    // etc.). See ticket for more details:
    // * https://github.com/oxur/rucksack/issues/92
    // cfg.rucksack.db_file = db.path();
    // cfg.rucksack.data_dir = file::dir_parent(db.path());
    log::debug!("Setting up rucksack application ...");
    let app = rucksack::app::new(cfg, db);
    run(&matches, &app)?;
    shutdown(&matches, &app)
}
