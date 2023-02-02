use anyhow::Result;
use clap::ArgMatches;

use crate::app::App;
use crate::store::records::Status;
use crate::time;

use super::util;

pub fn one(matches: &ArgMatches, app: &App) -> Result<()> {
    let key = util::key(matches);
    log::debug!("Marking account '{}' as deleted ...", key);
    let now = time::now();
    let mut record = util::record(&app.db, matches)?;
    record.metadata.state = Status::Deleted;
    record.metadata.updated = now;
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}
