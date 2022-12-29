// use std::ffi::OsStr;
// use std::process;
use std::{io};

// use anyhow::{anyhow, Context, Result};
use anyhow::{Context, Result};
// use clap::builder::{EnumValueParser, PossibleValuesParser, ValueParser};
use clap::builder::{EnumValueParser};
use clap::{Arg, ArgAction, ArgMatches, Command};

mod cli;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESC: &str = env!("CARGO_PKG_DESCRIPTION");

fn cli() -> Command {
    Command::new(NAME)
    .about(DESC)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .arg(
        Arg::new("completions")
            .help("emit shell tab completions")
            .long("completions")
            .value_name("SHELL")
            .value_parser(EnumValueParser::<clap_complete::Shell>::new()),
    )
    .arg(
        Arg::new("version")
            .help("Print version information")
            .short('v')
            .long("version")
            .action(ArgAction::SetTrue)
    )
    .subcommand(
        Command::new("gen")
            .about("generate a secret")
            .arg(
                Arg::new("type")
                    .help("the type of generator to use")
                    .short('t')
                    .long("type")
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
    if let Some(is_version) = matches.get_one::<bool>("version") {
        if *is_version {
            return cli::command::version(VERSION);
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

    // match run(&matches, &config) {
    run(&matches)
}
