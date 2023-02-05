use crate::store::records::{default_metadata, v040, v060, v070};
use crate::time;

pub fn store_pwd() -> String {
    "abc123".to_string()
}

pub fn plaintext_record_v040() -> v040::DecryptedRecord {
    let now = time::now();
    let epoch_zero = time::epoch_zero();
    let md = v040::Metadata {
        kind: v040::Kind::default(),
        url: "https://site.com/".to_string(),
        created: now,
        imported: epoch_zero.clone(),
        updated: epoch_zero.clone(),
        password_changed: epoch_zero.clone(),
        last_used: epoch_zero,
        access_count: 0,
    };
    v040::DecryptedRecord {
        creds: v040::Creds {
            user: "alice@site.com".to_string(),
            password: "4 s3kr1t".to_string(),
        },
        metadata: md,
    }
}

pub fn plaintext_record_v060() -> v060::DecryptedRecord {
    let now = time::now();
    let epoch_zero = time::epoch_zero();
    let md = v060::Metadata {
        kind: v060::Kind::default(),
        url: "https://site.com/".to_string(),
        created: now,
        imported: epoch_zero.clone(),
        updated: epoch_zero.clone(),
        password_changed: epoch_zero.clone(),
        last_used: epoch_zero,
        access_count: 0,
    };
    v060::DecryptedRecord {
        creds: v060::Creds {
            user: "alice@site.com".to_string(),
            password: "4 s3kr1t".to_string(),
        },
        metadata: md,
    }
}

pub fn plaintext_record_v070() -> v070::DecryptedRecord {
    let mut md = default_metadata();
    md.url = "https://site.com/".to_string();
    v070::DecryptedRecord {
        secrets: v070::secrets_from_user_pass("alice@site.com", "4 s3kr1t"),
        metadata: md,
    }
}
