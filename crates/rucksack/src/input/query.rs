use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, SecretString};

use rucksack_db::records::DecryptedRecord;

use crate::app::App;

pub fn record(app: &App) -> Result<DecryptedRecord> {
    log::trace!(key = app.inputs.key().as_str(), operation = "get_record"; "Getting record key by app inputs");
    record_by_key(app, app.inputs.key())
}

pub fn record_with_default(app: &App) -> Result<DecryptedRecord> {
    let key = app.inputs.key();
    log::debug!(key = key.as_str(), operation = "query"; "Querying record by key");
    match app.db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            log::debug!(operation = "create_default"; "Record not found; creating new one");
            Ok(DecryptedRecord::new())
        }
    }
}

pub fn record_by_key(app: &App, key: String) -> Result<DecryptedRecord> {
    log::debug!(key = key.as_str(), operation = "query"; "Querying record by key");
    match app.db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            let msg = format!("No secret record for given key '{key}'");
            log::info!(key = key.as_str(), operation = "query"; "{}", msg);
            Err(anyhow!(msg))
        }
    }
}

pub fn remove(app: &App) -> Result<()> {
    remove_by_key(app, app.inputs.key())
}

pub fn remove_by_key(app: &App, key: String) -> Result<()> {
    log::debug!(key = key.as_str(), operation = "remove"; "Removing record");
    match app.db.delete(key.clone()) {
        Some(true) => Ok(()),
        Some(false) => {
            let msg = format!("Could not delete record with given key '{key}'");
            log::error!(key = key.as_str(), operation = "remove"; "{}", msg);
            Err(anyhow!(msg))
        }
        None => unreachable!(),
    }
}

pub fn reveal(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}
