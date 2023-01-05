use std::io::{BufReader, Cursor};

use anyhow::Result;
use serde::Deserialize;

use crate::store;
use crate::util;

use super::csv as csv_importer;

#[derive(Debug, Deserialize)]
pub struct ChromeRecord {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl ChromeRecord {
    fn to_firefox(&self) -> csv_importer::Record {
        csv_importer::new_with_password(
            self.url.clone(),
            self.username.clone(),
            self.password.clone(),
        )
    }
}

pub fn from_csv(db: store::db::DB, csv_path: String) -> Result<(), anyhow::Error> {
    let bytes = util::read_file(csv_path.clone())?;
    let reader = BufReader::new(Cursor::new(bytes));
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    println!("Importing data from {}:", csv_path);
    let mut count = 0;
    for result in rdr.deserialize() {
        let chrome: ChromeRecord = result?;
        let csv_record = chrome.to_firefox();
        db.insert(csv_record.to_decrypted());
        count += 1;
        print!(".");
    }
    csv_importer::print_report(count, db.hash_map().len());
    db.close()
}
