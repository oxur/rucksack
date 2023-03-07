//! # Install
//!
//! The project CI/CD pipeline is not currently building binary executables for
//! download, so you must have `rust` and `cargo` installed. Then you may
//! install the latest version of `rucksack` with the following:
//!
//! ```shell
//! cargo install rucksack
//! ```
//!
//! # Getting Started
//!
//! The quickest way to get started is to explore the CLI help text:
//!
//! ```shell
//! rucksack help
//! ```
//!
//!
//! ```text
//! rucksack: A terminal-based secrets manager, generator, and importer/exporter (Firefox, Chrome) backed with a concurrent hashmap
//!
//! Usage: rucksack [OPTIONS] [COMMAND]
//!
//! Commands:
//!   add     Add a new secret
//!   export  Export the rucksack db
//!   gen     Generate a secret
//!   import  Pull in creds from other sources
//!   list    List all secrets
//!   rm      Delete a single record [aliases: delete, remove]
//!   set     Perform various 'write' operations
//!   show    Display rucksack-specific information
//!   help    Print this message or the help of the given subcommand(s)
//!
//! Options:
//!       --config-file <config-file>  The path to the config file to use or create [default: "<user config dir>/rucksack/config.toml"]
//!       --log-level <log-level>      Override the configured log-level setting [default: ] [possible values: error, warn, info, debug, trace, ]
//!       --completions <SHELL>        Emit shell tab completions [possible values: bash, elvish, fish, powershell, zsh]
//!   -v, --version                    Print version information
//!   -h, --help                       Print help
//! ```
//!
//! # Example Usage
//!
//! Be sure to see the documentation for the following `rucksack` CLI subcommands
//! here:
//! * [add](handlers/add/index.html)
//! * [backup](handlers/backup/index.html)
//! * [completions](handlers/completions/index.html)
//! * [config](handlers/config/index.html)
//! * [export](handlers/export/index.html)
//! * [gen](handlers/gen/index.html)
//! * [help](handlers/help/index.html)
//! * [import](handlers/import/index.html)
//! * [list](handlers/list/index.html)
//! * [rm](handlers/rm/index.html)
//! * [set](handlers/set/index.html)
//! * [show](handlers/show/index.html)
//! * [version](handlers/version/index.html)
//!
//! # License
//!
//! Copyright © 2022-2023, Oxur Group
//!
//! Apache License, Version 2.0
//!
#[doc(hidden)]
pub mod app;
pub mod command;
pub mod handlers;
#[doc(hidden)]
pub mod input;
#[doc(hidden)]
pub mod output;
pub mod service;
#[doc(hidden)]
pub use app::App;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
