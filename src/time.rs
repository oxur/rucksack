use chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};

pub fn simple_timestamp() -> String {
    chrono::offset::Local::now()
        .format("%Y%m%d-%H%M%S")
        .to_string()
}

pub fn now() -> String {
    Local::now().to_rfc3339()
}

pub fn epoch_to_string(e: i64) -> String {
    Utc.timestamp_millis_opt(e).unwrap().to_rfc3339()
}

pub fn epoch_zero() -> String {
    epoch_to_string(0)
}

pub fn string_to_epoch(stamp: String) -> i64 {
    match DateTime::parse_from_rfc3339(&stamp) {
        Ok(dt) => dt.timestamp_millis(),
        Err(e) => {
            log::debug!("{:?}", e);
            Local::now().timestamp_millis()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time;

    #[test]
    fn epoch_zero() {
        assert_eq!(time::epoch_zero(), "1970-01-01T00:00:00+00:00");
    }
}
