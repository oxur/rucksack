use anyhow::{Error, Result};

pub fn to_bytes() -> Result<csv::Writer<Vec<u8>>, Error> {
    Ok(csv::Writer::from_writer(Vec::<u8>::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let result = to_bytes();
        assert!(result.is_ok());
        let writer = result.unwrap();
        assert_eq!(writer.into_inner().unwrap().len(), 0);
    }

    #[test]
    fn test_to_bytes_write_data() {
        let mut writer = to_bytes().unwrap();
        writer.write_record(&["name", "age"]).unwrap();
        writer.write_record(&["Alice", "30"]).unwrap();
        writer.flush().unwrap();

        let bytes = writer.into_inner().unwrap();
        let csv_string = String::from_utf8(bytes).unwrap();
        assert!(csv_string.contains("name,age"));
        assert!(csv_string.contains("Alice,30"));
    }
}
