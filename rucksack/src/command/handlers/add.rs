//! # Adding Records
//!
//! To add a single record via the CLI:
//!
//! ```shell
//! rucksack add \
//!   --url http://example.com \
//!   --user shelly \
//!   --password whyyyyyy
//! ```
//!
//! Note that `--user` and `--url` are required when adding a new record. A password is required, too: if one is not provided with `--password`, then you will be prompted:
//!
//! ```shell
//! rucksack add \
//!   --url http://example.com \
//!   --user shelly
//! ```
//!
//! ```shell
//! Enter db password:
//! ```
//!
//! ```shell
//! Enter password for record:
//! ```
//!
use anyhow::{anyhow, Result};
use clap::ArgMatches;

use rucksack_db as store;
use rucksack_db::records;
use rucksack_db::{default_metadata, DecryptedRecord};

use crate::app::App;
use crate::input::{query, Flag};

pub fn new(_matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    let kind = app.inputs.record_kind();
    if let Ok(_dr) = query::record(app) {
        return Err(anyhow!(
            "Record already exists -- please use the 'set' command"
        ));
    }
    // Password and Account kinds
    let mut secrets = store::default_secrets();
    if kind == records::Kind::Password || kind == records::Kind::Account {
        secrets.user = app.inputs.user();
        secrets.password = app.inputs.record_passwd();
    };
    if kind == records::Kind::Account {
        secrets.account_id = app.inputs.account_id();
    }
    // Asymmetric crypto kind
    if kind == records::Kind::AsymmetricCrypto {
        secrets.public_key = app.inputs.public();
        secrets.private_key = app.inputs.private();
    }
    // Certs kind
    if kind == records::Kind::Certificates {
        secrets.public_cert = app.inputs.public();
        secrets.private_cert = app.inputs.private();
        secrets.root_cert = app.inputs.root();
    }
    // Service creds kind
    if kind == records::Kind::ServiceCredentials {
        secrets.key = app.inputs.service_key();
        secrets.secret = app.inputs.service_secret();
    }
    let mut metadata = default_metadata();
    metadata.category = app.inputs.category(Flag::One);
    if let Some(tags) = app.inputs.tags() {
        metadata.tags = tags
    }
    metadata.name = app.inputs.name();
    metadata.kind = kind;
    metadata.url = app.inputs.url();
    let dr = DecryptedRecord {
        secrets,
        metadata,
        history: vec![],
    };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
