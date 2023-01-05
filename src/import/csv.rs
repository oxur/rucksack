use serde::Deserialize;

use crate::store;
use crate::util;

// This started as the Firefox login data struct, but it has more fields than
// others, so it has become the default interim struct to which others convert
// to.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub url: String,
    pub user: String,
    pub password: String,
    pub http_realm: String,
    pub form_action_origin: String,
    pub guid: String,
    pub time_created: i64,
    pub time_last_used: i64,
    pub time_password_changed: i64,
}

pub fn new(url: String, user: String) -> Record {
    new_with_password(url, user, "".to_string())
}

pub fn new_with_password(url: String, user: String, password: String) -> Record {
    Record {
        url,
        user,
        password,

        ..Default::default()
    }
}

impl Record {
    pub fn to_decrypted(&self) -> store::record::DecryptedRecord {
        let now = chrono::offset::Local::now().to_rfc3339();
        let creds = store::record::Creds {
            user: self.user.clone(),
            password: self.password.clone(),
        };
        let metadata = store::record::Metadata {
            kind: store::record::Kind::Password,
            url: self.url.clone(),
            created: util::epoch_to_string(self.time_created),
            imported: now.clone(),
            updated: now,
            password_changed: util::epoch_to_string(self.time_password_changed),
            last_used: util::epoch_to_string(self.time_last_used),
            access_count: 0,
        };
        store::record::DecryptedRecord { creds, metadata }
    }
}

pub fn print_report(count: usize, total: usize) {
    println!();
    println!(
        "Imported {} records (total records in DB: {})",
        count, total
    )
}
