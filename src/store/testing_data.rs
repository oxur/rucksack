use crate::time;

use super::records::{Creds, DecryptedRecord, Kind, Metadata};

pub fn store_pwd() -> String {
    "abc123".to_string()
}

pub fn plaintext_record() -> DecryptedRecord {
    let date_time = time::now();

    DecryptedRecord {
        creds: Creds {
            user: "alice@site.com".to_string(),
            password: "4 s3kr1t".to_string(),
        },
        metadata: Metadata {
            kind: Kind::Password,
            url: "https://site.com/".to_string(),
            created: date_time.clone(),
            imported: date_time.clone(),
            updated: date_time.clone(),
            password_changed: date_time.clone(),
            last_used: date_time.clone(),
            access_count: 0,
            synced: date_time,
        },
    }
}
