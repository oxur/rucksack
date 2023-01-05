use anyhow::Result;

use crate::csv;
use crate::csv::chrome;
use crate::store;

use super::shared::print_report;

pub fn from_csv(db: store::db::DB, csv_path: String) -> Result<(), anyhow::Error> {
    println!("Importing data from {}:", csv_path);
    let mut rdr = csv::reader(csv_path)?;
    let mut count = 0;
    for result in rdr.deserialize() {
        let chr: chrome::Record = result?;
        db.insert(chr.to_decrypted());
        count += 1;
        print!(".");
    }
    print_report(count, db.hash_map().len());
    db.close()
}
