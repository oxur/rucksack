use std::io::Cursor;

use anyhow::{Error, Result};

use rucksack_lib::file;

pub fn from_path(path: String) -> Result<csv::Reader<Cursor<Vec<u8>>>, Error> {
    let bytes = file::read(path)?;
    Ok(csv::Reader::from_reader(Cursor::new(bytes)))
}
