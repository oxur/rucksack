use serde::{Deserialize, Serialize};
use url::Url;

use crate::store;

use super::firefox;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Record {
    pub fn to_decrypted(&self) -> store::DecryptedRecord {
        let ffr = firefox::new_with_password(
            self.url.clone(),
            self.username.clone(),
            self.password.clone(),
        );
        ffr.to_decrypted()
    }
}

pub fn from_decrypted(r: store::DecryptedRecord) -> Record {
    let url = r.metadata().url;
    let parsed = Url::parse(&url).unwrap();
    Record {
        name: parsed.host_str().unwrap().to_string(),
        url,
        username: r.user(),
        password: r.password(),
    }
}
