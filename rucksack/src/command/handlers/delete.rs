//! # Deleting Records
//!
//! By default, accounts are not removed; instead, they are flagged as `deleted`. To delete an account entry:
//!
//! ```shell
//! rucksack delete \
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

use crate::app::App;
use crate::input::query;

pub fn one(_matches: &ArgMatches, app: &App) -> Result<()> {
    let key = app.inputs.key();
    log::debug!("Marking record '{}' as deleted ...", key);
    let mut record = query::record(app)?;
    record.set_status(Status::Deleted);
    app.db.update(key, record);
    app.db.close()?;
    Ok(())
}
