use std::io;

use anyhow::{Context, Result};
use clap::ArgMatches;

use rucksack::cli;
use rucksack::cli::command::{add, export, gen, import, list, rm, set, show};
use rucksack::{config, util};

const NAME: &str = env!("CARGO_PKG_NAME");

fn run(matches: &ArgMatches, app: &rucksack::App) -> Result<()> {
    log::debug!("Preparing to dispatch based upon (sub)command ...");
    match matches.subcommand() {
        Some(("add", add_matches)) => add::new(add_matches, app)?,
        Some(("export", export_matches)) => export::new(export_matches, app)?,
        Some(("gen", gen_matches)) => gen::new(gen_matches)?,
        Some(("import", import_matches)) => import::new(import_matches, app)?,
        Some(("list", list_matches)) => match list_matches.subcommand() {
            Some(("deleted", deleted_matches)) => list::deleted(deleted_matches, app)?,
            Some((&_, _)) => todo!(),
            None => list::all(list_matches, app)?,
        },
        Some(("rm", rm_matches)) => rm::one(rm_matches, app)?,
        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("password", password_matches)) => set::password(password_matches, app)?,
            Some(("url", url_matches)) => set::url(url_matches, app)?,
            Some(("user", user_matches)) => set::user(user_matches, app)?,
            Some(("type", type_matches)) => set::account_type(type_matches, app)?,
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some(("show", show_matches)) => match show_matches.subcommand() {
            Some(("config-file", cfgfile_matches)) => show::config_file(cfgfile_matches, app)?,
            Some(("config", cfg_matches)) => show::config(cfg_matches, app)?,
            Some(("data-dir", datadir_matches)) => show::data_dir(datadir_matches, app)?,
            Some(("db-file", dbfile_matches)) => show::db_file(dbfile_matches, app)?,
            Some(("db-version", dbvsn_matches)) => show::db_version(dbvsn_matches, app)?,
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

fn main() -> Result<()> {
    let mut rucksack = cli::command::setup();
    let matches = rucksack.clone().get_matches();
    let mut config_file = String::new();
    if let Some(cfg_file) = matches.get_one::<String>("config-file") {
        config_file = cfg_file.to_string();
    }
    let mut log_level = String::new();
    if let Some(level) = matches.get_one::<String>("log-level") {
        log_level = level.to_string();
    }
    let mut cfg = config::load(config_file, log_level);
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => {
            panic!("Could not setup logger: {:?}", error)
        }
    }
    log::debug!("Config setup complete (using {})", cfg.rucksack.cfg_file);
    log::debug!("Logger setup complete");

    // Shell completion generation is completely independent, so perform it before
    // any config or subcommand operations.
    if let Some(is_version) = matches.get_one::<bool>("version") {
        if *is_version {
            return util::display(util::version().to_string().as_str());
        }
    } else if let Some(shell) = matches.get_one::<clap_complete::Shell>("completions") {
        clap_complete::generate(*shell, &mut rucksack, NAME, &mut io::stdout());
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
    let db = cli::command::setup_db(subcmd_matches)?;
    cfg.rucksack.db_file = db.path();
    cfg.rucksack.data_dir = util::dir_parent(db.path());
    log::debug!("Setting up rucksack application ...");
    let app = rucksack::app::new(cfg, db);
    run(&matches, &app)
}
