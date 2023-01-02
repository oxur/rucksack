use anyhow::Result;
use clap::ArgMatches;

use crate::import;
use crate::store;

pub fn new(matches: &ArgMatches) -> Result<()> {
    let db_file = matches.get_one::<String>("db").unwrap().to_string();
    let import_file = matches.get_one::<String>("file").unwrap().to_string();
    let pwd = matches.get_one::<String>("password").unwrap().to_string();
    let now = chrono::offset::Local::now().to_rfc3339();
    let db = store::db::open(db_file, pwd, now)?;
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("firefox") => import::firefox::from_csv(db, import_file)?,
        Some(_) => todo!(),
        None => todo!(),
    };
    Ok(())
}
