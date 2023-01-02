use std::error::Error;

use chrono::TimeZone;
use serde::Deserialize;

use crate::store;

#[derive(Deserialize)]
struct CSVRecord {
    url: String,
    username: String,
    password: String,
    // httpRealm: String,
    // formActionOrigin: String,
    // guid: String,
    // timeCreated: i64,
    time_created: i64,
    // timeLastUsed: i64,
    time_last_used: i64,
    // timePasswordChanged: i64,
    time_password_changed: i64,
}

fn convert_epoch(e: i64) -> String {
    chrono::Utc.timestamp_opt(e, 0).unwrap().to_rfc3339()
}

fn record_from_csv(csv: CSVRecord) -> store::record::DecryptedRecord {
    let now = chrono::offset::Local::now().to_rfc3339();
    let creds = store::record::Creds {
        user: csv.username,
        password: csv.password,
    };
    let metadata = store::record::Metadata {
        kind: store::record::Kind::Password,
        url: csv.url,
        // created: convert_epoch(csv.timeCreated),
        created: convert_epoch(csv.time_created),
        imported: now.clone(),
        updated: now,
        // password_changed: convert_epoch(csv.timePasswordChanged),
        password_changed: convert_epoch(csv.time_password_changed),
        // last_used: convert_epoch(csv.timeLastUsed),
        last_used: convert_epoch(csv.time_last_used),
        access_count: 0,
    };

    store::record::DecryptedRecord { creds, metadata }
}

pub fn import_csv(db: store::db::DB, csv_path: String) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(csv_path)?;
    let reader = std::io::BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let csv_record: CSVRecord = result?;
        db.insert(record_from_csv(csv_record));
    }
    Ok(())
}
