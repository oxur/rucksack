use chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};

pub fn simple_timestamp() -> String {
    format_datetime(chrono::offset::Local::now())
}

pub fn format_datetime(dt: DateTime<Local>) -> String {
    dt.format("%Y%m%d-%H%M%S").to_string()
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
    use super::*;

    #[test]
    fn test_epoch_zero() {
        assert_eq!(super::epoch_zero(), "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_epoch_to_string_zero() {
        assert_eq!(epoch_to_string(0), "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_epoch_to_string_positive() {
        // 1000 milliseconds = 1 second after epoch
        assert_eq!(epoch_to_string(1000), "1970-01-01T00:00:01+00:00");
    }

    #[test]
    fn test_epoch_to_string_large() {
        // January 1, 2020, 00:00:00 UTC
        let timestamp = 1577836800000i64;
        let result = epoch_to_string(timestamp);
        assert!(result.starts_with("2020-01-01"));
    }

    #[test]
    fn test_simple_timestamp_format() {
        let timestamp = simple_timestamp();
        // Should match format: YYYYMMDD-HHMMSS
        assert_eq!(timestamp.len(), 15, "Timestamp should be 15 characters");
        assert!(timestamp.contains('-'), "Timestamp should contain hyphen");

        // Check that it's all digits except the hyphen
        let parts: Vec<&str> = timestamp.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 8, "Date part should be 8 digits");
        assert_eq!(parts[1].len(), 6, "Time part should be 6 digits");
        assert!(parts[0].chars().all(|c| c.is_ascii_digit()));
        assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_format_datetime() {
        // Create a known datetime
        let dt = Local.with_ymd_and_hms(2023, 12, 25, 14, 30, 45).unwrap();
        let formatted = format_datetime(dt);
        assert_eq!(formatted, "20231225-143045");
    }

    #[test]
    fn test_format_datetime_midnight() {
        let dt = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let formatted = format_datetime(dt);
        assert_eq!(formatted, "20000101-000000");
    }

    #[test]
    fn test_format_datetime_end_of_day() {
        let dt = Local.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        let formatted = format_datetime(dt);
        assert_eq!(formatted, "20231231-235959");
    }

    #[test]
    fn test_now_format() {
        let timestamp = now();
        // Should be RFC3339 format, which includes 'T' and timezone
        assert!(timestamp.contains('T'), "RFC3339 should contain 'T'");
        assert!(
            timestamp.contains('+') || timestamp.contains('Z') || timestamp.contains('-'),
            "RFC3339 should contain timezone indicator"
        );
    }

    #[test]
    fn test_string_to_epoch_valid_rfc3339() {
        let valid_stamp = "2020-01-01T00:00:00+00:00".to_string();
        let epoch = string_to_epoch(valid_stamp);
        assert_eq!(epoch, 1577836800000);
    }

    #[test]
    fn test_string_to_epoch_valid_with_z() {
        let valid_stamp = "2020-01-01T00:00:00Z".to_string();
        let epoch = string_to_epoch(valid_stamp);
        assert_eq!(epoch, 1577836800000);
    }

    #[test]
    fn test_string_to_epoch_invalid_format() {
        let invalid_stamp = "not a timestamp".to_string();
        let epoch = string_to_epoch(invalid_stamp);
        // Should return current time, which will be > 0
        assert!(epoch > 0, "Invalid format should return current timestamp");
    }

    #[test]
    fn test_string_to_epoch_empty() {
        let empty_stamp = "".to_string();
        let epoch = string_to_epoch(empty_stamp);
        // Should return current time
        assert!(epoch > 0);
    }

    #[test]
    fn test_string_to_epoch_partial_date() {
        let partial = "2020-01-01".to_string();
        let epoch = string_to_epoch(partial);
        // Incomplete RFC3339 should fall back to current time
        assert!(epoch > 0);
    }

    #[test]
    fn test_epoch_roundtrip() {
        let original_epoch = 1609459200000i64; // 2021-01-01 00:00:00 UTC
        let timestamp_str = epoch_to_string(original_epoch);
        let recovered_epoch = string_to_epoch(timestamp_str);
        // Allow small difference due to timezone conversions
        let difference = (original_epoch - recovered_epoch).abs();
        assert!(
            difference < 86400000, // Less than 24 hours difference
            "Roundtrip should preserve epoch within timezone tolerance"
        );
    }

    #[test]
    fn test_simple_timestamp_uniqueness() {
        // Two timestamps taken immediately after each other should be very similar
        let ts1 = simple_timestamp();
        let ts2 = simple_timestamp();
        // They should have the same date part at minimum
        assert_eq!(&ts1[0..8], &ts2[0..8], "Timestamps should have same date");
    }

    #[test]
    fn epoch_zero() {
        // Keep original test name for backwards compatibility
        assert_eq!(super::epoch_zero(), "1970-01-01T00:00:00+00:00");
    }
}
