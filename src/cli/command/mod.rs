use clap::builder::EnumValueParser;
use clap::{Arg, ArgAction, Command};

use crate::store::records;

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
        Command::new("add")
            .about("add a new secret")
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
            .arg(record_url().required(true))
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
    )
    .subcommand(
        Command::new("export")
            .about("export the rucksack db")
            .arg(
                Arg::new("output")
                    .help("path to the file that will contain the exported data")
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
            .about("generate a secret")
            .arg(db_not_needed())
            .arg(
                Arg::new("type")
                    .help("the type of generator to use")
                    .short('t')
                    .long("type")
                    .default_value("uuid++")
                    .value_parser(["lipsum", "random", "uuid", "uuid+", "uuid++", ]),
            )
            .arg(
                Arg::new("length")
                    .help("the character length of secret to generate (ignored for fixed-length generator types)")
                    .short('l')
                    .long("length")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("12"),
            )
            .arg(
                Arg::new("suffix-length")
                    .help("the character length of a random suffix (for generator types that support suffixes)")
                    .long("suffix-length")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("4"),
            )
            .arg(
                Arg::new("word-count")
                    .help("the number of words to generate (for generator types that assemble words)")
                    .short('w')
                    .long("word-count")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("4"),
            )
            .arg(
                Arg::new("delimiter")
                    .help("the character used to join parts (for generator types that join parts)")
                    .short('d')
                    .long("delimiter")
                    .default_value("-"),
            )
            .arg(
                Arg::new("encode")
                    .help("encode the generated password (uses base64)")
                    .short('e')
                    .long("encode")
                    .action(ArgAction::SetTrue),
            ),
    )
    .subcommand(
        Command::new("import")
            .about("pull in creds from other sources")
            .arg(serialised_format())
            .arg(
                Arg::new("file")
                    .help("credential file to import (for file-based importers)")
                    .short('f')
                    .long("file"),
            )
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
    )
    .subcommand(
        Command::new("list")
            .about("list all secrets")
            .arg(
                Arg::new("decrypt")
                    .help("using this flag causes all secrets to be decrypted to allow for scoring, etc.")
                    .long("decrypt")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(
                Arg::new("filter")
                    .help("show records where the user or the URL contain the given string")
                    .short('f')
                    .long("filter")
                    .visible_alias("include")
                    .global(true)
            )
            .arg(
                Arg::new("exclude")
                    .help("don't show records where the user or the URL contain the given string")
                    .short('x')
                    .long("exclude")
                    .global(true)
            )
            .arg(
                Arg::new("group-by")
                    .help("group results that have the same value for the given field")
                    .short('g')
                    .long("group-by")
                    .visible_alias("partition")
                    .value_parser(["password", "user"])
                    .global(true)
            )
            .arg(
                Arg::new("max-score")
                    .help("limit results to secrets that do not exceed the given maximum score")
                    .long("max-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("100")
                    .global(true)
            )
            .arg(
                Arg::new("min-score")
                    .help("limit results to secrets that are not less than the given minimum score")
                    .long("min-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("0").global(true)
            )
            .arg(
                Arg::new("reveal")
                    .help("display the actual the passwords")
                    .long("reveal")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(
                Arg::new("sort-by")
                    .help("display the actual the passwords")
                    .short('s')
                    .long("sort-by")
                    .visible_alias("order-by")
                    .default_value("url")
                    .value_parser(["score", "url", "user"])
                    .global(true)
            )
            .arg(
                Arg::new("with-status")
                    .help("display the actual state of the record")
                    .long("with-status")
                    .action(ArgAction::SetTrue)
                    .global(true)
            )
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
            .arg(record_category().default_value(records::ANY_CATEGORY))
            .arg(record_type_list())
            .arg(record_name())
            .subcommand(
                Command::new("deleted")
                    .about("list the records that have been flagged for deletion"))
    )
    .subcommand(
        Command::new("rm")
            .about("delete a single record")
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
            .about("perform various 'write' operations")
            .arg(db_arg())
            .arg(pwd_arg())
            .arg(salt_arg())
            .arg(record_category())
            .arg(record_type())
            .arg(record_name())
            .subcommand(
                Command::new("password")
                    .about("change the password for the given record")
                    .arg(record_pass())
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("status")
                    .about("set the status for the given record")
                    .arg(record_status().required(true))
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("url")
                    .about("change the url for the given record")
                    .arg(record_url_old().required(true))
                    .arg(record_url_new().required(true))
                    .arg(record_user().required(true))
            )
            .subcommand(
                Command::new("user")
                    .about("change the user (login name) for the given record")
                    .arg(record_user_old().required(true))
                    .arg(record_user_new().required(true))
                    .arg(record_url().required(true))
            )
            .subcommand(
                Command::new("type")
                    .about("change the type of the given record")
                    .arg(record_user().required(true))
                    .arg(record_url().required(true))
            )
    )
    .subcommand(
        Command::new("show")
            .about("display rucksack-specific information")
            .arg(db_arg())
            .arg(db_not_needed())
            .subcommand(
                Command::new("categories")
                    .about("display the categories currently used across all records")
                    .arg(db_needed())
                    .arg(pwd_arg())
                    .arg(salt_arg())
            )
            .subcommand(
                Command::new("config-file")
                    .about("display the location of the config file used by rucksack")
            )
            .subcommand(
                Command::new("config")
                    .about("display rucksack's current configuration")
            )
            .subcommand(
                Command::new("data-dir")
                    .about("display the location of the rucksack data directory")
            )
            .subcommand(
                Command::new("db-file")
                    .about("display the location of the rucksack database file")
            )
            .subcommand(
                Command::new("db-version")
                    .about("display the file schema version of a given database file")
                    .arg(db_needed())
                    .arg(pwd_arg())
                    .arg(salt_arg())
            )
    )
}

// Top-level Flags

pub fn config_arg() -> Arg {
    let config_file = crate::util::config_file();
    Arg::new("config-file")
        .help("the path to the config file to use or create")
        .long("config-file")
        .default_value(config_file)
        .global(true)
}

pub fn log_level_arg() -> Arg {
    Arg::new("log-level")
        .help("override the configured log-level setting")
        .long("log-level")
        .default_value("")
        .value_parser(["error", "warn", "info", "debug", "trace", ""])
        .global(true)
}

// Database Flags

pub fn db_arg() -> Arg {
    Arg::new("db")
        .help("path to the encrypted database to use")
        .short('d')
        .long("db")
        .global(true)
}

pub fn pwd_arg() -> Arg {
    Arg::new("db-pass")
        .help("password used to encrypt the database")
        .long("db-pass")
        .global(true)
}

pub fn salt_arg() -> Arg {
    Arg::new("salt")
        .help("the salt to use for encrypting the database")
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
        .help("the user-supplied category of the given record")
        .long("category")
        .default_value(records::DEFAULT_CATEGORY)
        .global(true)
}

pub fn record_status() -> Arg {
    Arg::new("status")
        .help("the status of the given record")
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
        .help("the type of secret for the record")
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
        .help("the user/login identifier")
        .short('u')
        .long("user")
}

pub fn record_user_old() -> Arg {
    Arg::new("old-user")
        .help("the old user login name")
        .short('u')
        .long("old-user")
}

pub fn record_user_new() -> Arg {
    Arg::new("new-user")
        .help("the new user login name to use")
        .short('u')
        .long("new-user")
}

pub fn record_pass() -> Arg {
    Arg::new("password")
        .help("the login password")
        .long("password")
}

pub fn record_url() -> Arg {
    Arg::new("url").help("the login URL").long("url")
}

pub fn record_url_old() -> Arg {
    Arg::new("old-url")
        .help("the old login URL")
        .long("old-url")
}

pub fn record_url_new() -> Arg {
    Arg::new("new-url")
        .help("the new URL for the login")
        .long("new-url")
}

pub fn record_account_id() -> Arg {
    Arg::new("account-id")
        .help("the account ID for secrets of type 'account'")
        .long("account-id")
}

pub fn record_secret_public() -> Arg {
    Arg::new("public")
        .help("the public key for asymmetric-crypto secrets; the public cert for certificate-based secrets")
        .long("public")
}

pub fn record_secret_private() -> Arg {
    Arg::new("private")
        .help("the private key for asymmetric-crypto secrets; the private cert for certificate-based secrets")
        .long("private")
}

pub fn record_root_cert() -> Arg {
    Arg::new("root")
        .help("the root cert for certificate-based secrets")
        .long("root")
}

pub fn record_key() -> Arg {
    Arg::new("key")
        .help("the key for service-credential-based secrets")
        .long("key")
}

pub fn record_secret() -> Arg {
    Arg::new("secret")
        .help("the secret for service-credential-based secrets")
        .long("secret")
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
