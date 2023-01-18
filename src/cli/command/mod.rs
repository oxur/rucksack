use clap::builder::EnumValueParser;
use clap::{Arg, ArgAction, Command};

pub mod add;
pub mod arg;
pub mod export;
pub mod gen;
pub mod import;
pub mod list;
pub mod rm;
pub mod set;
pub mod util;

pub use util::setup_db;

const NAME: &str = env!("CARGO_PKG_NAME");
const DESC: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn setup() -> Command {
    Command::new(NAME)
    .about(format!("{}: {}", NAME, DESC))
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
        Command::new("add")
            .about("add a new secret")
            .arg(arg::account_type())
            .arg(arg::account_user().required(true))
            .arg(arg::account_pass())
            .arg(arg::account_url().required(true))
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
    )
    .subcommand(
        Command::new("export")
            .about("export the rucksack db")
            .arg(
                Arg::new("type")
                    .help("the type of export to create")
                    .short('t')
                    .long("type")
                    .default_value("firefox")
                    .value_parser(["chrome", "debug", "firefox"]),
            )
            .arg(
                Arg::new("file")
                    .help("path to the file that will contain the exported data")
                    .short('f')
                    .long("file"),
            )
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
    )
    .subcommand(
        Command::new("gen")
            .about("generate a secret")
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
            .arg(
                Arg::new("type")
                    .help("the type of importer to use")
                    .short('t')
                    .long("type")
                    .default_value("firefox")
                    .value_parser(["chrome", "firefox"]),
            )
            .arg(
                Arg::new("file")
                    .help("credential file to import (for file-based importers)")
                    .short('f')
                    .long("file"),
            )
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
    )
    .subcommand(
        Command::new("list")
            .about("list all secrets")
            .arg(
                Arg::new("decrypt")
                    .help("using this flag causes all secrets to be decrypted to allow for scoring, etc.")
                    .long("decrypt")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("filter")
                    .help("show records where the user or the URL contain the given string")
                    .short('f')
                    .long("filter")
                    .alias("include")
            )
            .arg(
                Arg::new("exclude")
                    .help("don't show records where the user or the URL contain the given string")
                    .short('x')
                    .long("exclude")
            )
            .arg(
                Arg::new("group-by")
                    .help("group results that have the same value for the given field")
                    .short('g')
                    .long("group-by")
                    .alias("partition")
                    .value_parser(["password", "user"]),
            )
            .arg(
                Arg::new("max-score")
                    .help("limit results to secrets that do not exceed the given maximum score")
                    .long("max-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("100")
            )
            .arg(
                Arg::new("min-score")
                    .help("limit results to secrets that are not less than the given minimum score")
                    .long("min-score")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("0")
            )
            .arg(
                Arg::new("reveal")
                    .help("display the actual the passwords")
                    .long("reveal")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("sort-by")
                    .help("display the actual the passwords")
                    .short('s')
                    .long("sort-by")
                    .alias("order-by")
                    .default_value("url")
                    .value_parser(["score", "url", "user"]),
            )
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
    )
    .subcommand(
        Command::new("rm")
            .about("delete a single record")
            .alias("remove")
            .alias("delete")
            .arg(arg::account_user().required(true))
            .arg(arg::account_url().required(true))
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
    )
    .subcommand(
        Command::new("set")
            .about("perform various 'write' operations")
            .arg(arg::db_arg())
            .arg(arg::pwd_arg())
            .arg(arg::salt_arg())
            .subcommand(
                Command::new("password")
                    .about("change the password for the given account")
                    .arg(arg::account_pass())
                    .arg(arg::account_user().required(true))
                    .arg(arg::account_url().required(true))
            )
            .subcommand(
                Command::new("url")
                    .about("change the url for the given account")
                    .arg(arg::account_url_old().required(true))
                    .arg(arg::account_url_new().required(true))
                    .arg(arg::account_user().required(true))
            )
            .subcommand(
                Command::new("user")
                    .about("change the user (login name) for the given account")
                    .arg(arg::account_user_old().required(true))
                    .arg(arg::account_user_new().required(true))
                    .arg(arg::account_url().required(true))
            )
            .subcommand(
                Command::new("type")
                    .about("change the type of the given account")
                    .arg(arg::account_type().required(true))
                    .arg(arg::account_user().required(true))
                    .arg(arg::account_url().required(true))
                    .arg(arg::db_arg())
                    .arg(arg::pwd_arg())
                    .arg(arg::salt_arg())
            )
    )
}
