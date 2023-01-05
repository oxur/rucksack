use std::io::{BufReader, Cursor};

use anyhow::Result;

use crate::csv::firefox;
use crate::store;
use crate::util;

use super::shared::print_report;

pub fn from_csv(db: store::db::DB, csv_path: String) -> Result<(), anyhow::Error> {
    let bytes = util::read_file(csv_path.clone())?;
    let reader = BufReader::new(Cursor::new(bytes));
    let mut rdr = csv::Reader::from_reader(reader);
    println!("Importing data from {}:", csv_path);
    let mut count: usize = 0;
    for result in rdr.deserialize() {
        let csv_record: firefox::Record = result?;
        db.insert(csv_record.to_decrypted());
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
}
