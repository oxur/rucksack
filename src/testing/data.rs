use crate::store::records::{default_metadata, Creds, DecryptedRecord};

pub fn store_pwd() -> String {
    "abc123".to_string()
}

pub fn plaintext_record() -> DecryptedRecord {
    let mut md = default_metadata();
    md.url = "https://site.com/".to_string();
    DecryptedRecord {
        creds: Creds {
            user: "alice@site.com".to_string(),
            password: "4 s3kr1t".to_string(),
        },
        metadata: md,
    }
}
