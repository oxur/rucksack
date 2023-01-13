use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::time;

use super::util;

pub fn account_type(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account type ...");
    let mut record = util::record(&app.db, matches)?;
    record.metadata.kind = util::account_kind(matches);
    record.metadata.updated = time::now();
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn password(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account password ...");
    let now = time::now();
    let mut record = util::record(&app.db, matches)?;
    record.creds.password = util::account_pwd_revealed(matches);
    record.metadata.password_changed = now.clone();
    record.metadata.updated = now;
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn url(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account URL ...");
    log::warn!("Not implemented!\nmatches: {:?}", matches);
    app.db.close()?;
    Ok(())
}

pub fn user(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account user ...");
    log::warn!("Not implemented!\nmatches: {:?}", matches);
    app.db.close()?;
    Ok(())
}
