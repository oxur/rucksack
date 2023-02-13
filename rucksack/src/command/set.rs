//! # Updating Records
//!
//! Changing a password:
//!
//! ```shell
//! rucksack set password \
//!   --url http://example.com \
//!   --user shelly
//!   --password whyyyyyyyyyyyyyyyyyyy
//! ```
//!
//! If the password isn't provided, you will be prompted at the terminal:
//!
//! ```shell
//! rucksack set password \
//!   --url http://example.com \
//!   --user shelly
//! ```
//!
//! ```shell
//! Enter record password:
//! ```
//!
//! Changing a user:
//!
//! ```shell
//! rucksack set user \
//!   --url http://example.com \
//!   --old-user shelly
//!   --new-user clammy
//! ```
//!
//! Changing a URL:
//!
//! ```shell
//! rucksack set url \
//!   --old-url http://example.com \
//!   --new-url http://shelly.com \
//!   --user clammy
//! ```
//!
//! Changing the record type:
//!
//! ```shell
//! rucksack set type \
//!   --url http://example.com \
//!   --user clammy
//!   --type password
//! ```
//!
//! //! # All Subcommands
//!
//! See the full list of supported subcommands with:
//! ```shell
//! rucksack set -h
//! ```
//!
use anyhow::Result;
use clap::ArgMatches;

use rucksack_db as store;

use crate::app::App;
use crate::option;
use crate::query;

pub fn record_type(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting record type ...");
    let mut record = query::record(&app.db, matches)?;
    record.set_kind(option::record_kind(matches));
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn password(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting record password ...");
    let mut record = query::record(&app.db, matches)?;
    record.set_password(option::record_pwd_revealed(matches));
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn status(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting record status ...");
    let mut record = query::record(&app.db, matches)?;
    record.set_status(option::record_state(matches));
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn url(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting record URL ...");
    let category = option::category(matches);
    let kind = option::record_kind(matches);
    let user = option::user(matches);
    let old_url = option::url_old(matches);
    let new_url = option::url_new(matches);
    let key = store::key(&category, kind, &user, &old_url);
    let mut record = query::record_by_key(&app.db, key.clone())?;
    record.set_url(new_url);
    let msg = "there was a problem deleting the old record";
    match app.db.delete(key) {
        Some(false) => log::error!("{msg}"),
        Some(_) => (),
        None => log::error!("{msg}"),
    }
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}

pub fn user(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Setting record user ...");
    let category = option::category(matches);
    let kind = option::record_kind(matches);
    let old_user = option::user_old(matches);
    let new_user = option::user_new(matches);
    let url = option::url(matches);
    let key = store::key(&category, kind, &old_user, &url);
    let mut record = query::record_by_key(&app.db, key.clone())?;
    record.set_user(new_user);
    let msg = "there was a problem deleting the old record";
    match app.db.delete(key) {
        Some(false) => log::error!("{msg}"),
        Some(_) => (),
        None => log::error!("{msg}"),
    }
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}
