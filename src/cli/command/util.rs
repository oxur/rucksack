use anyhow::Result;
use clap::ArgMatches;

use crate::store::db;

pub fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

pub fn setup_db(matches: &ArgMatches) -> Result<db::DB> {
    let db_file = matches.get_one::<String>("db").unwrap().to_string();
    let pwd = matches.get_one::<String>("password").unwrap().to_string();
    let now = chrono::offset::Local::now().to_rfc3339();
    db::open(db_file, pwd, now)
}
