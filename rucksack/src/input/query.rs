use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, SecretString};

use rucksack_db::records::DecryptedRecord;

use crate::app::App;

pub fn record(app: &App) -> Result<DecryptedRecord> {
    record_by_key(app, app.inputs.key())
}

pub fn record_by_key(app: &App, key: String) -> Result<DecryptedRecord> {
    log::debug!("Querying record by key '{key}' ...");
    match app.db.get(key.clone()) {
        Some(dr) => Ok(dr),
        None => {
            let msg = format!("No secret record for given key '{key}'");
            log::info!("{msg}");
            Err(anyhow!(msg))
        }
    }
}

pub fn remove(app: &App) -> Result<()> {
    remove_by_key(app, app.inputs.key())
}

pub fn remove_by_key(app: &App, key: String) -> Result<()> {
    log::debug!("Removing record associated with {} ...", key);
    match app.db.delete(key.clone()) {
        Some(true) => Ok(()),
        Some(false) => {
            let msg = format!("Could not delete record with given key '{key}'");
            log::error!("{msg}");
            Err(anyhow!(msg))
        }
        None => unreachable!(),
    }
}

pub fn reveal(pwd: SecretString) -> String {
    pwd.expose_secret().to_string()
}
