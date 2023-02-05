use anyhow::{anyhow, Result};
use clap::ArgMatches;

use super::util;

use crate::app::App;
use crate::store::records::v070::secrets_from_user_pass;
use crate::store::{default_metadata, DecryptedRecord};

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    if let Ok(_dr) = util::record(&app.db, matches) {
        return Err(anyhow!(
            "Record already exists -- please use the 'update' command"
        ));
    }
    let secrets = secrets_from_user_pass(
        util::user(matches).as_str(),
        util::record_pwd_revealed(matches).as_str(),
    );
    let mut metadata = default_metadata();
    metadata.kind = util::record_kind(matches);
    metadata.url = util::url(matches);
    let dr = DecryptedRecord { secrets, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
