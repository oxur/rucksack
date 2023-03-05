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
use crate::input::{options, query};

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    let kind = options::record_kind(matches);
    if let Ok(_dr) = query::record(&app.db, matches) {
        return Err(anyhow!(
            "Record already exists -- please use the 'set' command"
        ));
    }
    // Password and Account kinds
    let mut secrets = store::default_secrets();
    if kind == records::Kind::Password || kind == records::Kind::Account {
        secrets.user = options::user(matches);
        secrets.password = options::record_pwd_revealed(matches);
    };
    if kind == records::Kind::Account {
        secrets.account_id = options::account_id(matches);
    }
    // Asymmetric crypto kind
    if kind == records::Kind::AsymmetricCrypto {
        secrets.public_key = options::public(matches);
        secrets.private_key = options::private(matches);
    }
    // Certs kind
    if kind == records::Kind::Certificates {
        secrets.public_cert = options::public(matches);
        secrets.private_cert = options::private(matches);
        secrets.root_cert = options::root(matches);
    }
    // Service creds kind
    if kind == records::Kind::ServiceCredentials {
        secrets.key = options::service_key(matches);
        secrets.secret = options::service_secret(matches);
    }
    let mut metadata = default_metadata();
    metadata.category = options::category(matches);
    if let Some(tags) = options::tags(matches) {
        metadata.tags = tags
    }
    metadata.name = options::name(matches);
    metadata.kind = kind;
    metadata.url = options::url(matches);
    let dr = DecryptedRecord {
        secrets,
        metadata,
        history: vec![],
    };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
