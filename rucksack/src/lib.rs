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
//!       --config-file <config-file>  The path to the config file to use or create [default: "<system config dir>/rucksack/config.toml"]
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
//! * [add](command/add/index.html)
//! * [export](command/export/index.html)
//! * [gen](command/gen/index.html)
//! * [import](command/import/index.html)
//! * [list](command/list/index.html)
//! * [rm](command/rm/index.html)
//! * [set](command/set/index.html)
//! * [show](command/show/index.html)
//!
//! # License
//!
//! Copyright © 2022-2023, Oxur Group
//!
//! Apache License, Version 2.0
//!
pub mod app;
pub mod command;
pub mod constant;

pub use app::App;
