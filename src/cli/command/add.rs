use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::store::record;
use crate::store::{Creds, DecryptedRecord, Metadata};
use crate::time;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    let account_type = matches.get_one::<String>("type").map(|s| s.as_str());
    let default_kind = record::Kind::Password;
    let kind = match account_type {
        Some("account") => record::Kind::Account,
        Some("creds") => record::Kind::Credential,
        Some("credential") => record::Kind::Credential,
        Some("password") => record::Kind::Password,
        Some("") => default_kind,
        Some(&_) => todo!(),
        None => default_kind,
    };

    let user = matches.get_one::<String>("user").unwrap().to_string();
    let password = matches.get_one::<String>("password").unwrap().to_string();
    let url = matches.get_one::<String>("url").unwrap().to_string();
    let now = time::now();

    let creds = Creds { user, password };
    let metadata = Metadata {
        kind,
        url,
        created: now.clone(),
        imported: now.clone(),
        updated: now.clone(),
        password_changed: now.clone(),
        last_used: now,
        access_count: 0,
    };
    let dr = DecryptedRecord { creds, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
