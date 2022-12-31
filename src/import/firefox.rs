use std::error::Error;
use std::io;
use std::process;

use crate::store;

pub fn import_csv(
    db_path: String,
    store_pwd: String,
    updated: String,
    csv_path: String,
) -> Result<(), Box<dyn Error>> {
    // XXX open the DB
    let db = store::db::open(db_path, store_pwd, updated);
    // XXX read from the file at the given path

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
