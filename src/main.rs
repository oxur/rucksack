// use std::ffi::OsStr;
// use std::process;
use std::{io};

// use anyhow::{anyhow, Context, Result};
use anyhow::{Context, Result};
// use clap::builder::{EnumValueParser, PossibleValuesParser, ValueParser};
use clap::{Arg, ArgMatches, Command};

mod cli;

const TOOL_NAME: &str = "rucksack";

fn cli() -> Command {
    Command::new(TOOL_NAME)
    .about("A terminal-based password manager, importer (Firefox Sync), and generator")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .subcommand(
        Command::new("gen")
            .about("generate a secret")
            .arg(
                Arg::new("type")
                    .help("the type of generator to use")
                    .index(1)
                    .default_value("default"),
            ),
    )
}

// fn run(matches: &ArgMatches, config: &kbs2::config::Config) -> Result<()> {
fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("gen", matches)) => cli::command::generate(matches)?,
        Some((&_, _)) => todo!(),
        None => todo!(),
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut rucksack = cli();
    let matches = rucksack.clone().get_matches();

    // Shell completion generation is completely independent, so perform it before
    // any config or subcommand operations.
    if let Some(shell) = matches.get_one::<clap_complete::Shell>("completions") {
        clap_complete::generate(*shell, &mut rucksack, env!("CARGO_PKG_NAME"), &mut io::stdout());
        return Ok(());
    }

    if matches.subcommand().is_none() {
        return rucksack
            .clone()
            .print_long_help()
            .with_context(|| "failed to print help".to_string());
    }

    // match run(&matches, &config) {
    run(&matches)
}
