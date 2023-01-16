use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::store::db::V1;

use super::util;

pub fn one(matches: &ArgMatches, app: &App) -> Result<()> {
    let key = util::key(matches);
    log::debug!("Removing account record {} ...", key);
    util::remove_by_key(&app.db, key)?;
    app.db.close()?;
    Ok(())
}
