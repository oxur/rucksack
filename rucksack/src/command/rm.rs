//! # Deleting Records
//!
//! By default, accounts are not removed; instead, they are flagged as `deleted`. To delete an account entry:
//!
//! ```shell
//! rucksack rm \
//!     --url http://example.com \
//!     --user clammy
//! ```
//!
//! To see the list of records that have been deleted:
//!
//! ```shell
//! rucksack list deleted
//! ```
//!
//! All the same flags and filtering used with the `list` command are available with `list deleted`.
//!
use anyhow::Result;
use clap::ArgMatches;

use rucksack_db::records::Status;
use rucksack_lib::time;

use crate::app::App;

use super::util;

pub fn one(matches: &ArgMatches, app: &App) -> Result<()> {
    let key = util::key(matches);
    log::debug!("Marking record '{}' as deleted ...", key);
    let now = time::now();
    let mut record = util::record(&app.db, matches)?;
    record.metadata.state = Status::Deleted;
    record.metadata.updated = now;
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}
