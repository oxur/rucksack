use std::io;

use anyhow::{Context, Result};
use clap::ArgMatches;

use rucksack::cli;
use rucksack::cli::command::{add, export, gen, import, list, rm, set, setup_db};
use rucksack::{config, util};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn run(matches: &ArgMatches, app: &rucksack::App) -> Result<()> {
    match matches.subcommand() {
        Some(("add", add_matches)) => add::new(add_matches, app)?,
        Some(("export", export_matches)) => export::new(export_matches, app)?,
        Some(("gen", gen_matches)) => gen::new(gen_matches)?,
        Some(("import", import_matches)) => import::new(import_matches, app)?,
        Some(("list", list_matches)) => list::all(list_matches, app)?,
        Some(("rm", rm_matches)) => rm::one(rm_matches, app)?,
        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("password", password_matches)) => set::password(password_matches, app)?,
            Some(("url", url_matches)) => set::url(url_matches, app)?,
            Some(("user", user_matches)) => set::user(user_matches, app)?,
            Some(("type", type_matches)) => set::account_type(type_matches, app)?,
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some((&_, _)) => todo!(),
        None => todo!(),
    }
    Ok(())
}

fn main() -> Result<()> {
    let cfg = config::load();
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => {
            panic!("Could not setup logger: {:?}", error)
        }
    }
    log::debug!("Config setup complete.");
    log::debug!("Logger setup complete.");
    let mut rucksack = cli::command::setup();
    let matches = rucksack.clone().get_matches();

    // Shell completion generation is completely independent, so perform it before
    // any config or subcommand operations.
    if let Some(is_version) = matches.get_one::<bool>("version") {
        if *is_version {
            return util::display(VERSION);
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
    let db = setup_db(subcmd_matches)?;
    let app = rucksack::App { cfg, db };
    run(&matches, &app)
}
