use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{store, time};

// This started as the Firefox login data struct, but it has more fields than
// others, so it has become the default interim struct to which others convert
// for imports.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub url: String,
    pub username: String,
    pub password: String,
    pub http_realm: String,
    pub form_action_origin: String,
    pub guid: String,
    pub time_created: i64,
    pub time_last_used: i64,
    pub time_password_changed: i64,
}

pub fn new(url: String, username: String) -> Record {
    new_with_password(url, username, "".to_string())
}

pub fn new_with_password(url: String, username: String, password: String) -> Record {
    Record {
        url,
        username,
        password,

        ..Default::default()
    }
}

impl Record {
    pub fn to_decrypted(&self) -> store::DecryptedRecord {
        let creds = store::Creds {
            user: self.username.clone(),
            password: self.password.clone(),
        };
        let mut metadata = store::default_metadata();
        metadata.url = self.url.clone();
        metadata.created = time::epoch_to_string(self.time_created);
        metadata.password_changed = time::epoch_to_string(self.time_password_changed);
        metadata.last_used = time::epoch_to_string(self.time_last_used);
        store::DecryptedRecord { creds, metadata }
    }
}

pub fn from_decrypted(r: store::DecryptedRecord) -> Record {
    let md = r.metadata();
    Record {
        url: r.metadata().url,
        username: r.user(),
        password: r.password(),
        form_action_origin: md.url,
        guid: Uuid::new_v4().to_string(),
        time_created: time::string_to_epoch(md.created),
        time_last_used: time::string_to_epoch(md.last_used),
        time_password_changed: time::string_to_epoch(md.password_changed),

        ..Default::default()
    }
}
