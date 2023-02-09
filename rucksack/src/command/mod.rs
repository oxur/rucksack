use clap::builder::EnumValueParser;
use clap::{value_parser, Arg, ArgAction, Command};

use rucksack_db::records;

pub mod add;
pub mod export;
pub mod gen;
pub mod import;
pub mod list;
pub mod rm;
pub mod set;
pub mod show;
pub mod util;

pub use util::setup_db;

const NAME: &str = env!("CARGO_PKG_NAME");
const DESC: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn setup() -> Command {
    Command::new(NAME)
    .about(format!("{NAME}: {DESC}"))
    .arg_required_else_help(true)
    // .allow_external_subcommands(true)
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
            .arg(record_category())
            .arg(record_type())
            .arg(record_name())
            .arg(record_user().required(true))
            .arg(record_pass())
            .arg(record_account_id())
            .arg(record_secret_public())
            .arg(record_secret_private())
            .arg(record_root_cert())
            .arg(record_key())
            .arg(record_secret())
            .arg(record_tags())
            .arg(record_url().required(true))
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
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
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
            .arg(serialised_format())
            .arg(record_type())
            .arg(record_category())
    )
    .subcommand(
        Command::new("gen")
            .about("Generate a secret")
            .arg(db_not_needed())
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
            .arg(serialised_format())
            .arg(
                Arg::new("file")
                    .help("Credential file to import (for file-based importers)")
                    .short('f')
                    .long("file"),
            )
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
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
                    .value_parser(["password", "user"])
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
                    .value_parser(["score", "url", "user"])
                    .global(true)
            )
            .arg(
                Arg::new("with-status")
                    .help("Display the actual state of the record")
                    .long("with-status")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
            .arg(record_category().default_value(records::ANY_CATEGORY))
            .arg(record_type_list())
            .arg(record_all_tags())
            .arg(record_any_tags())
            .arg(record_name())
            .subcommand(
                Command::new("deleted")
                    .about("List the records that have been flagged for deletion"))
    )
    .subcommand(
        Command::new("rm")
            .about("Delete a single record")
            .visible_aliases(["delete","remove"])
            .arg(record_category())
            .arg(record_type())
            .arg(record_name())
            .arg(record_user().required(true))
            .arg(record_url().required(true))
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
    )
    .subcommand(
        Command::new("set")
            .about("Perform various 'write' operations")
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
            .arg(record_category())
            .arg(record_type())
            .arg(record_name())
            .subcommand(
                Command::new("password")
                    .about("Change the password for the given record")
                    .arg(record_pass())
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("status")
                    .about("Set the status for the given record")
                    .arg(record_status().required(true))
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("url")
                    .about("Change the url for the given record")
                    .arg(record_url_old().required(true))
                    .arg(record_url_new().required(true))
                    .arg(record_user().required(true))
            )
            .subcommand(
                Command::new("user")
                    .about("Change the user (login name) for the given record")
                    .arg(record_user_old().required(true))
                    .arg(record_user_new().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("type")
                    .about("Change the type of the given record")
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
    )
    .subcommand(
        Command::new("show")
            .about("Display rucksack-specific information")
            .arg(db_arg())
            .arg(db_not_needed())
            .subcommand(
                Command::new("categories")
                    .about("Display the categories currently used across all records")
                    .arg(db_needed())
                    .arg(pwd_arg())
                    .arg(salt_arg())
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
                    .arg(db_needed())
                    .arg(pwd_arg())
                    .arg(salt_arg())
            )
            .subcommand(
                Command::new("tags")
                    .about("Display the tags currently used across all records")
                    .arg(db_needed())
                    .arg(pwd_arg())
                    .arg(salt_arg())
            )
            .subcommand(
                Command::new("types")
                    .about("Display the record types supported")
            )
    )
}

// Top-level Flags

pub fn config_arg() -> Arg {
    let config_file = rucksack_lib::util::config_file();
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

// Database Flags

pub fn db_arg() -> Arg {
    Arg::new("db")
        .help("Path to the encrypted database to use")
        .short('d')
        .long("db")
        .global(true)
}

pub fn pwd_arg() -> Arg {
    Arg::new("db-pass")
        .help("Password used to encrypt the database")
        .long("db-pass")
        .global(true)
}

pub fn salt_arg() -> Arg {
    Arg::new("salt")
        .help("The salt to use for encrypting the database")
        .default_value(default_salt())
        .short('s')
        .long("salt")
        .global(true)
}

fn default_salt() -> String {
    match std::env::var("USER") {
        Ok(user) => user,
        Err(_) => "rucksack".to_string(),
    }
}

// Record Flags

pub fn record_category() -> Arg {
    Arg::new("category")
        .help("The user-supplied category of the given record")
        .long("category")
        .default_value(records::DEFAULT_CATEGORY)
        .global(true)
}

pub fn record_status() -> Arg {
    Arg::new("status")
        .help("The status of the given record")
        .default_value("active")
        .value_parser(["active", "inactive", "deleted"])
}

pub fn record_types_allowed() -> Vec<&'static str> {
    vec![
        "",
        "account",
        "asymmetric-crypto",
        "asymmetric",
        "certs",
        "certificates",
        "password",
        "service-creds",
        "service-credentials",
    ]
}

pub fn record_types_list_allowed() -> Vec<&'static str> {
    let mut rta = record_types_allowed();
    rta.push("any");
    rta
}

pub fn record_base() -> Arg {
    Arg::new("type")
        .help("The type of secret for the record")
        .short('t')
        .long("type")
        .global(true)
}

pub fn record_type() -> Arg {
    record_base()
        .default_value("password")
        .value_parser(record_types_allowed())
}

pub fn record_type_list() -> Arg {
    record_base()
        .default_value("any")
        .value_parser(record_types_list_allowed())
}

pub fn record_name() -> Arg {
    Arg::new("name").help("the record name").long("name")
}

pub fn record_user() -> Arg {
    Arg::new("user")
        .help("The user/login identifier")
        .short('u')
        .long("user")
}

pub fn record_user_old() -> Arg {
    Arg::new("old-user")
        .help("The old user login name")
        .short('u')
        .long("old-user")
}

pub fn record_user_new() -> Arg {
    Arg::new("new-user")
        .help("The new user login name to use")
        .short('u')
        .long("new-user")
}

pub fn record_pass() -> Arg {
    Arg::new("password")
        .help("The login password")
        .long("password")
}

pub fn record_url() -> Arg {
    Arg::new("url").help("the login URL").long("url")
}

pub fn record_url_old() -> Arg {
    Arg::new("old-url")
        .help("The old login URL")
        .long("old-url")
}

pub fn record_url_new() -> Arg {
    Arg::new("new-url")
        .help("The new URL for the login")
        .long("new-url")
}

pub fn record_account_id() -> Arg {
    Arg::new("account-id")
        .help("The account ID for secrets of type 'account'")
        .long("account-id")
}

pub fn record_secret_public() -> Arg {
    Arg::new("public")
        .help("The public key for asymmetric-crypto secrets; the public cert for certificate-based secrets")
        .long("public")
}

pub fn record_secret_private() -> Arg {
    Arg::new("private")
        .help("The private key for asymmetric-crypto secrets; the private cert for certificate-based secrets")
        .long("private")
}

pub fn record_root_cert() -> Arg {
    Arg::new("root")
        .help("The root cert for certificate-based secrets")
        .long("root")
}

pub fn record_key() -> Arg {
    Arg::new("key")
        .help("The key for service-credential-based secrets")
        .long("key")
}

pub fn record_secret() -> Arg {
    Arg::new("secret")
        .help("The secret for service-credential-based secrets")
        .long("secret")
}

pub fn record_tags() -> Arg {
    Arg::new("tags")
        .help("One or more tags for a record (use a ',' to delimit multiple)")
        .long("tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

pub fn record_all_tags() -> Arg {
    Arg::new("all-tags")
        .help("Limit results to records that have ALL of the tags passed")
        .long("all-tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

pub fn record_any_tags() -> Arg {
    Arg::new("any-tags")
        .help("Limit results to records that have ANY of the tags passed")
        .long("any-tags")
        .use_value_delimiter(true)
        .num_args(0..)
        .value_parser(value_parser!(String))
        .action(ArgAction::Append)
}

// Miscellaneous

pub fn db_not_needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("false")
        .global(true)
}

pub fn db_needed() -> Arg {
    Arg::new("db-needed")
        .hide(true)
        .long("db-needed")
        .value_parser(clap::builder::BoolValueParser::new())
        .default_value("true")
        .global(true)
}

pub fn serialised_format() -> Arg {
    Arg::new("format")
        .help("the de/serialisation format to use for import/export")
        .long("format")
        .value_parser(["", "chrome", "debug", "firefox"])
        .global(true)
}

mod tests {

    #[test]
    fn record_types_allowed() {
        assert_eq!(
            super::record_types_allowed(),
            vec![
                "",
                "account",
                "asymmetric-crypto",
                "asymmetric",
                "certs",
                "certificates",
                "password",
                "service-creds",
                "service-credentials",
            ]
        );
    }

    #[test]
    fn record_types_list_allowed() {
        assert_eq!(
            super::record_types_list_allowed(),
            vec![
                "",
                "account",
                "asymmetric-crypto",
                "asymmetric",
                "certs",
                "certificates",
                "password",
                "service-creds",
                "service-credentials",
                "any"
            ]
        );
    }
}
