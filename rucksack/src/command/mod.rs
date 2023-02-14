use clap::builder::EnumValueParser;
use clap::{Arg, ArgAction, Command};

use rucksack_db::records;

use crate::constant;

pub mod add;
pub mod arg;
pub mod export;
pub mod gen;
pub mod import;
pub mod list;
pub mod output;
pub mod rm;
pub mod set;
pub mod show;

use arg::{db, record};

pub fn setup() -> Command {
    Command::new(constant::NAME)
    .about(format!("{}: {}", constant::NAME, constant::DESC))
    .arg_required_else_help(true)
    .arg(config_arg())
    .arg(log_level_arg())
    .arg(
        Arg::new("completions")
            .help("Emit shell tab completions")
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
        Command::new("add")
            .about("Add a new secret")
            .arg(record::category())
            .arg(record::kind())
            .arg(record::name())
            .arg(record::user().required(true))
            .arg(record::pass())
            .arg(record::account_id())
            .arg(record::secret_public())
            .arg(record::secret_private())
            .arg(record::root_cert())
            .arg(record::key())
            .arg(record::secret())
            .arg(record::tags())
            .arg(record::url().required(true))
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
    )
    .subcommand(
        Command::new("export")
            .about("Export the rucksack db")
            .arg(
                Arg::new("output")
                    .help("Path to the file that will contain the exported data")
                    .short('o')
                    .long("output"),
            )
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
            .arg(db::serialised_format())
            .arg(record::kind())
            .arg(record::category())
    )
    .subcommand(
        Command::new("gen")
            .about("Generate a secret")
            .arg(db::not_needed())
            .arg(
                Arg::new("type")
                    .help("The type of generator to use")
                    .short('t')
                    .long("type")
                    .default_value("uuid++")
                    .value_parser(["lipsum", "random", "uuid", "uuid+", "uuid++", ]),
            )
            .arg(
                Arg::new("length")
                    .help("The character length of secret to generate (ignored for fixed-length generator types)")
                    .short('l')
                    .long("length")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("12"),
            )
            .arg(
                Arg::new("suffix-length")
                    .help("The character length of a random suffix (for generator types that support suffixes)")
                    .long("suffix-length")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("4"),
            )
            .arg(
                Arg::new("word-count")
                    .help("The number of words to generate (for generator types that assemble words)")
                    .short('w')
                    .long("word-count")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("4"),
            )
            .arg(
                Arg::new("delimiter")
                    .help("The character used to join parts (for generator types that join parts)")
                    .short('d')
                    .long("delimiter")
                    .default_value("-"),
            )
            .arg(
                Arg::new("encode")
                    .help("Encode the generated password (uses base64)")
                    .short('e')
                    .long("encode")
                    .action(ArgAction::SetTrue),
            ),
    )
    .subcommand(
        Command::new("import")
            .about("Pull in creds from other sources")
            .arg(db::serialised_format())
            .arg(
                Arg::new("file")
                    .help("Credential file to import (for file-based importers)")
                    .short('f')
                    .long("file"),
            )
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
    )
    .subcommand(
        Command::new("list")
            .about("List all secrets")
            .arg(
                Arg::new("decrypt")
                    .help("Using this flag causes all secrets to be decrypted to allow for scoring, etc.")
                    .long("decrypt")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(
                Arg::new("filter")
                    .help("List records where the user or the URL contain the given string")
                    .short('f')
                    .long("filter")
                    .visible_alias("include")
                    .global(true)
            )
            .arg(
                Arg::new("exclude")
                    .help("Don't show records where the user or the URL contain the given string")
                    .short('x')
                    .long("exclude")
                    .global(true)
            )
            .arg(
                Arg::new("group-by")
                    .help("Group results that have the same value for the given field")
                    .short('g')
                    .long("group-by")
                    .visible_alias("partition")
                    .value_parser(["password", "name"])
                    .global(true)
            )
            .arg(
                Arg::new("max-score")
                    .help("Limit results to secrets that do not exceed the given maximum score")
                    .long("max-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("100")
                    .global(true)
            )
            .arg(
                Arg::new("min-score")
                    .help("Limit results to secrets that are not less than the given minimum score")
                    .long("min-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("0").global(true)
            )
            .arg(
                Arg::new("reveal")
                    .help("Display the actual the passwords")
                    .long("reveal")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(
                Arg::new("sort-by")
                    .help("Sort by the given field")
                    .short('s')
                    .long("sort-by")
                    .visible_alias("order-by")
                    .default_value("url")
                    .value_parser(["score", "url", "name"])
                    .global(true)
            )
            .arg(
                Arg::new("with-status")
                    .help("Display the actual state of the record")
                    .long("with-status")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
            .arg(record::category().default_value(records::ANY_CATEGORY))
            .arg(record::type_list())
            .arg(record::all_tags())
            .arg(record::any_tags())
            .arg(record::name())
            .subcommand(
                Command::new("deleted")
                    .about("List the records that have been flagged for deletion"))
            .subcommand(
                Command::new("passwords")
                    .about("List the historical passwords (including current) for the given record")
                    .arg(record::category())
                    .arg(record::kind())
                    .arg(record::name())
                    .arg(record::user().required(true))
                    .arg(record::url().required(true)))
    )
    .subcommand(
        Command::new("rm")
            .about("Delete a single record")
            .visible_aliases(["delete","remove"])
            .arg(record::category())
            .arg(record::kind())
            .arg(record::name())
            .arg(record::user().required(true))
            .arg(record::url().required(true))
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
    )
    .subcommand(
        Command::new("set")
            .about("Perform various 'write' operations")
            .arg(db::path())
            .arg(db::pwd())
            .arg(db::salt())
            .arg(record::category())
            .arg(record::kind())
            .arg(record::name())
            .subcommand(
                Command::new("password")
                    .about("Change the password for the given record")
                    .arg(record::pass())
                    .arg(record::user().required(true))
                    .arg(record::url().required(true))
            )
            .subcommand(
                Command::new("status")
                    .about("Set the status for the given record")
                    .arg(record::status().required(true))
                    .arg(record::user().required(true))
                    .arg(record::url().required(true))
            )
            .subcommand(
                Command::new("url")
                    .about("Change the url for the given record")
                    .arg(record::url_old().required(true))
                    .arg(record::url_new().required(true))
                    .arg(record::user().required(true))
            )
            .subcommand(
                Command::new("user")
                    .about("Change the user (login name) for the given record")
                    .arg(record::user_old().required(true))
                    .arg(record::user_new().required(true))
                    .arg(record::url().required(true))
            )
            .subcommand(
                Command::new("type")
                    .about("Change the type of the given record")
                    .arg(record::user().required(true))
                    .arg(record::url().required(true))
            )
    )
    .subcommand(
        Command::new("show")
            .about("Display rucksack-specific information")
            .arg(db::path())
            .arg(db::not_needed())
            .subcommand(
                Command::new("categories")
                    .about("Display the categories currently used across all records")
                    .arg(db::needed())
                    .arg(db::pwd())
                    .arg(db::salt())
            )
            .subcommand(
                Command::new("config-file")
                    .about("Display the location of the config file used by rucksack")
            )
            .subcommand(
                Command::new("config")
                    .about("Display rucksack's current configuration")
            )
            .subcommand(
                Command::new("data-dir")
                    .about("Display the location of the rucksack data directory")
            )
            .subcommand(
                Command::new("db-file")
                    .about("display the location of the rucksack database file")
            )
            .subcommand(
                Command::new("db-version")
                    .about("Display the file schema version of a given database file")
                    .arg(db::needed())
                    .arg(db::pwd())
                    .arg(db::salt())
            )
            .subcommand(
                Command::new("tags")
                    .about("Display the tags currently used across all records")
                    .arg(db::needed())
                    .arg(db::pwd())
                    .arg(db::salt())
            )
            .subcommand(
                Command::new("types")
                    .about("Display the record types supported")
            )
    )
}

// Top-level Flags

pub fn config_arg() -> Arg {
    let config_file = rucksack_lib::util::config_file(constant::NAME);
    Arg::new("config-file")
        .help("The path to the config file to use or create")
        .long("config-file")
        .default_value(config_file)
        .global(true)
}

pub fn log_level_arg() -> Arg {
    Arg::new("log-level")
        .help("Override the configured log-level setting")
        .long("log-level")
        .default_value("")
        .value_parser(["error", "warn", "info", "debug", "trace", ""])
        .global(true)
}
