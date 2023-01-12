use anyhow::{anyhow, Result};
use clap::ArgMatches;

use crate::app::App;
use crate::store::record;
use crate::time;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'update' subcommand ...");
    let user = matches.get_one::<String>("user").unwrap().to_string();
    let url = matches.get_one::<String>("url").unwrap().to_string();
    let key = record::key(&user, &url);
    let dr = app.db.get(key.clone());
    if dr.is_none() {
        return Err(anyhow!("no secret record for given key '{}'", key));
    }
    let now = time::now();
    let mut record = dr.unwrap();
    let kind = match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("account") => record::Kind::Account,
        Some("creds") => record::Kind::Credential,
        Some("credential") => record::Kind::Credential,
        Some("password") => record::Kind::Password,
        Some(&_) => record.metadata().kind,
        None => record.metadata().kind,
    };
    record.metadata.kind = kind;
    record.metadata.updated = now.clone();
    let password = match matches.get_one::<String>("password") {
        Some(pwd) => {
            record.metadata.password_changed = now;
            pwd.to_owned()
        }
        None => record.creds.password,
    };
    record.creds.password = password;
    app.db.insert(record);
    app.db.close()?;
    Ok(())
}
