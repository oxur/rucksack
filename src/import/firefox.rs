use std::error::Error;
use std::io;

use crate::store;

pub fn import_csv(db: store::db::DB, csv_path: String) -> Result<(), Box<dyn Error>> {
    // XXX read from the CSV file at the given path
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        // XXX add each record to the db
        println!("{:?}", record);
    }
    Ok(())
}
