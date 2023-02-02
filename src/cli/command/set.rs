use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::store;
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

pub fn active(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account 'active' flag ...");
    let now = time::now();
    let mut record = util::record(&app.db, matches)?;
    record.metadata.state = util::account_active_state(matches);
    record.metadata.updated = now;
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
    let old_url = util::url_old(matches);
    let new_url = util::url_new(matches);
    let user = util::user(matches);
    let key = store::key(&user, &old_url);
    let mut record = util::record_by_key(&app.db, key.clone())?;
    record.metadata.url = new_url;
    record.metadata.updated = time::now();
    match app.db.delete(key) {
        Some(false) => log::error!("there was a problem deleting the record"),
        Some(_) => (),
        None => log::error!("there was a problem deleting the record"),
    }
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn user(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting account user ...");
    let old_user = util::user_old(matches);
    let new_user = util::user_new(matches);
    let url = util::url(matches);
    let key = store::key(&old_user, &url);
    let mut record = util::record_by_key(&app.db, key.clone())?;
    record.creds.user = new_user;
    record.metadata.updated = time::now();
    match app.db.delete(key) {
        Some(false) => log::error!("there was a problem deleting the record"),
        Some(_) => (),
        None => log::error!("there was a problem deleting the record"),
    }
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}
