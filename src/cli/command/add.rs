use anyhow::{anyhow, Result};
use clap::ArgMatches;

use super::util;

use crate::app::App;
use crate::store::{default_metadata, Creds, DecryptedRecord};

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    if let Ok(_dr) = util::record(&app.db, matches) {
        return Err(anyhow!(
            "Record already exists -- please use the 'update' command"
        ));
    }
    let creds = Creds {
        user: util::user(matches),
        password: util::record_pwd_revealed(matches),
    };
    let mut metadata = default_metadata();
    metadata.kind = util::record_kind(matches);
    metadata.url = util::url(matches);
    let dr = DecryptedRecord { creds, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
