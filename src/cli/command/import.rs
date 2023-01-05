use anyhow::Result;
use clap::ArgMatches;

use super::util;
use crate::import;

pub fn new(matches: &ArgMatches) -> Result<()> {
    let import_file = matches.get_one::<String>("file").unwrap().to_string();
    let db = util::setup_db(matches)?;
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("chrome") => import::chrome::from_csv(db, import_file)?,
        Some("firefox") => import::firefox::from_csv(db, import_file)?,
        Some(_) => todo!(),
        None => todo!(),
    };
    Ok(())
}
