use super::record::{Creds, DecryptedRecord, Kind, Metadata};

pub fn store_pwd() -> String {
    "abc123".to_string()
}

pub fn now() -> String {
    chrono::offset::Local::now().to_rfc3339()
}

pub fn plaintext_record() -> DecryptedRecord {
    let date_time = now();

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
            last_used: date_time,
            access_count: 0,
        },
    }
}
