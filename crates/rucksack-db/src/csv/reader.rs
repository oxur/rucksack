use std::io::Cursor;

use anyhow::{Error, Result};

use rucksack_lib::file;

pub fn from_path(path: String) -> Result<csv::Reader<Cursor<Vec<u8>>>, Error> {
    let bytes = file::read(path)?;
    Ok(csv::Reader::from_reader(Cursor::new(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_from_path_valid_csv() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let csv_data = b"name,age\nAlice,30\nBob,25\n";
        temp_file.write_all(csv_data).unwrap();

        let path = temp_file.path().to_str().unwrap().to_string();
        let result = from_path(path);
        assert!(result.is_ok());

        let mut reader = result.unwrap();
        let records: Vec<_> = reader.records().collect();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_from_path_nonexistent_file() {
        let result = from_path("/nonexistent/path/file.csv".to_string());
        assert!(result.is_err());
    }
}
