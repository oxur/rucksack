use rucksack_lib::time;

use crate::records::{v040, v060, v070, v080, v090};

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
    let dr060 = plaintext_record_v060();
    v070::migrate_decrypted_record_from_v060(dr060)
}

pub fn plaintext_record_v080() -> v080::DecryptedRecord {
    let dr070 = plaintext_record_v070();
    v080::migrate_decrypted_record_from_v070(dr070)
}

pub fn plaintext_record_v090() -> v090::DecryptedRecord {
    let dr080 = plaintext_record_v080();
    let mut dr = v090::migrate_decrypted_record_from_v080(dr080);
    dr.set_password("5 s3kr1t".to_string());
    dr.set_password("6 s3kr1t".to_string());
    dr
}
