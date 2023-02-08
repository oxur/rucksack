use anyhow::{anyhow, Result};
use clap::ArgMatches;

use rucksack_db as store;
use rucksack_db::records;
use rucksack_db::{default_metadata, DecryptedRecord};

use crate::app::App;

use super::util;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    let kind = util::record_kind(matches);
    if let Ok(_dr) = util::record(&app.db, matches) {
        return Err(anyhow!(
            "Record already exists -- please use the 'set' command"
        ));
    }
    // Password and Account kinds
    let mut secrets = store::default_secrets();
    if kind == records::Kind::Password || kind == records::Kind::Account {
        secrets.user = util::user(matches);
        secrets.password = util::record_pwd_revealed(matches);
    };
    if kind == records::Kind::Account {
        secrets.account_id = util::account_id(matches);
    }
    // Asymmetric crypto kind
    if kind == records::Kind::AsymmetricCrypto {
        secrets.public_key = util::public(matches);
        secrets.private_key = util::private(matches);
    }
    // Certs kind
    if kind == records::Kind::Certificates {
        secrets.public_cert = util::public(matches);
        secrets.private_cert = util::private(matches);
        secrets.root_cert = util::root(matches);
    }
    // Service creds kind
    if kind == records::Kind::ServiceCredentials {
        secrets.key = util::service_key(matches);
        secrets.secret = util::service_secret(matches);
    }
    let mut metadata = default_metadata();
    metadata.category = util::category(matches);
    if let Some(tags) = util::tags(matches) {
        metadata.tags = tags
    }
    metadata.name = util::name(matches);
    metadata.kind = kind;
    metadata.url = util::url(matches);
    let dr = DecryptedRecord { secrets, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
