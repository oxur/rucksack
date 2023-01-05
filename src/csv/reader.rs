use std::io::Cursor;

use anyhow::{Error, Result};

use crate::util;

pub fn from_path(path: String) -> Result<csv::Reader<Cursor<Vec<u8>>>, Error> {
    let bytes = util::read_file(path)?;
    Ok(csv::Reader::from_reader(Cursor::new(bytes)))
}