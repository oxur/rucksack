use anyhow::{Error, Result};

pub fn to_bytes() -> Result<csv::Writer<Vec<u8>>, Error> {
    Ok(csv::Writer::from_writer(Vec::<u8>::new()))
}
