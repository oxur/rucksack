use anyhow::{anyhow, Result};
use clap::ArgMatches;

use super::util;

use crate::app::App;
use crate::store::{Creds, DecryptedRecord, Metadata, Status};
use crate::time;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    if let Ok(_dr) = util::record(&app.db, matches) {
        return Err(anyhow!(
            "Record already exists -- please use the 'update' command"
        ));
    }
    let now = time::now();
    let creds = Creds {
        user: util::user(matches),
        password: util::account_pwd_revealed(matches),
    };
    let metadata = Metadata {
        kind: util::account_kind(matches),
        url: util::url(matches),
        created: now.clone(),
        imported: now.clone(),
        updated: now.clone(),
        password_changed: now.clone(),
        last_used: now.clone(),
        access_count: 0,
        synced: now,
        state: Status::Active,
    };
    let dr = DecryptedRecord { creds, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
