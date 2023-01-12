use anyhow::{anyhow, Result};
use clap::ArgMatches;
use secrecy::{ExposeSecret, SecretString};

use super::prompt;

use crate::app::App;
use crate::store::record;
use crate::store::{Creds, DecryptedRecord, Metadata};
use crate::time;

pub fn new(matches: &ArgMatches, app: &App) -> Result<()> {
    log::debug!("Running 'add' subcommand ...");
    let user = matches.get_one::<String>("user").unwrap().to_string();
    let url = matches.get_one::<String>("url").unwrap().to_string();
    let key = record::key(&user, &url);
    if let Some(_dr) = app.db.get(key) {
        return Err(anyhow!(
            "Record already exists -- please use the 'update' command"
        ));
    }
    let pwd = match matches.get_one::<String>("password") {
        Some(flag_pwd) => SecretString::new(flag_pwd.to_owned()),
        None => prompt::secret("Enter password for new record: ").unwrap(),
    };
    let account_type = matches.get_one::<String>("type").map(|s| s.as_str());
    let default_kind = record::Kind::Password;
    let kind = match account_type {
        Some("account") => record::Kind::Account,
        Some("creds") => record::Kind::Credential,
        Some("credential") => record::Kind::Credential,
        Some("password") => record::Kind::Password,
        Some("") => default_kind,
        Some(&_) => todo!(),
        None => default_kind,
    };

    let now = time::now();

    let creds = Creds {
        user,
        password: pwd.expose_secret().to_string(),
    };
    let metadata = Metadata {
        kind,
        url,
        created: now.clone(),
        imported: now.clone(),
        updated: now.clone(),
        password_changed: now.clone(),
        last_used: now,
        access_count: 0,
    };
    let dr = DecryptedRecord { creds, metadata };
    app.db.insert(dr);
    app.db.close()?;
    Ok(())
}
