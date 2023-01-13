use anyhow::{anyhow, Result};
use clap::ArgMatches;

use crate::app::App;
use crate::time;

use super::util;

pub fn account_type(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account type ...");
    log::warn!("Not implemented!\nmatches: {:?}", matches);
    app.db.close()?;
    Ok(())
}

pub fn password(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account password ...");
    let dr = util::record(&app.db, matches);
    if dr.is_none() {
        return Err(anyhow!(
            "no secret record for given key '{}'",
            util::key(matches)
        ));
    }
    let mut record = dr.unwrap();
    record.creds.password = util::account_pwd_revealed(matches);
    record.metadata.password_changed = time::now();
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
