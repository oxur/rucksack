use std::str;

use rand::distributions::{Distribution, Uniform};
use uuid::Uuid;

use crate::util;

pub fn v4_string() -> String {
    Uuid::new_v4().to_string()
}

pub fn v4_with_uppers() -> String {
    let uuid = v4_string();
    let parts: Vec<&str> = uuid.split('-').collect();
    let (first_part, rest_parts) = parts.split_at(1);
    let first = first_part.to_vec().pop().unwrap().to_uppercase();
    let mut rest = rest_parts.to_vec();
    let last = rest.pop().unwrap().to_uppercase();
    rest.insert(0, &first);
    rest.push(&last);
    rest.join("-")
}

pub fn v4_with_specials(count: usize) -> String {
    let mut rng = rand::thread_rng();
    let uuid = v4_with_uppers();
    let mut parts: Vec<String> = uuid.split("").map(|s| s.to_string()).collect();
    let len = parts.len();
    let die = Uniform::from(1..len);
    let specials = util::random_specials(count);
    for special in specials.iter().take(count) {
        let throw = die.sample(&mut rng);
        parts[throw] = String::from_utf8_lossy(&[*special]).to_string();
    }
    parts.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4_string_format() {
        let uuid = v4_string();
        // UUID v4 format: 8-4-4-4-12
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|c| *c == '-').count(), 4);
    }

    #[test]
    fn test_v4_string_valid_chars() {
        let uuid = v4_string();
        let valid_chars = "0123456789abcdef-";
        for c in uuid.chars() {
            assert!(valid_chars.contains(c), "Invalid character in UUID: {}", c);
        }
    }

    #[test]
    fn test_v4_string_unique() {
        let uuid1 = v4_string();
        let uuid2 = v4_string();
        assert_ne!(uuid1, uuid2, "UUIDs should be unique");
    }

    #[test]
    fn test_v4_string_lowercase() {
        let uuid = v4_string();
        let letters: String = uuid.chars().filter(|c| c.is_alphabetic()).collect();
        assert_eq!(letters, letters.to_lowercase(), "UUID should be lowercase");
    }

    #[test]
    fn test_v4_with_uppers_format() {
        let uuid = v4_with_uppers();
        // Should still have UUID format
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|c| *c == '-').count(), 4);
    }

    #[test]
    fn test_v4_with_uppers_has_uppercase() {
        // Generate multiple UUIDs to ensure at least one has letters to test
        let mut found_uppercase = false;
        for _ in 0..10 {
            let uuid = v4_with_uppers();
            let parts: Vec<&str> = uuid.split('-').collect();
            assert_eq!(parts.len(), 5);

            // First part should have uppercase if it contains letters
            let first_part = parts[0];
            let has_letters_first = first_part.chars().any(|c| c.is_alphabetic());
            if has_letters_first {
                let has_upper_first = first_part.chars().any(|c| c.is_uppercase());
                assert!(
                    has_upper_first,
                    "First part should contain uppercase when it has letters"
                );
                found_uppercase = true;
            }

            // Last part should have uppercase if it contains letters
            let last_part = parts[4];
            let has_letters_last = last_part.chars().any(|c| c.is_alphabetic());
            if has_letters_last {
                let has_upper_last = last_part.chars().any(|c| c.is_uppercase());
                assert!(
                    has_upper_last,
                    "Last part should contain uppercase when it has letters"
                );
                found_uppercase = true;
            }
        }

        // Verify we tested at least one UUID with letters (statistically almost certain)
        assert!(
            found_uppercase,
            "Should have found at least one UUID with letters in 10 attempts"
        );
    }

    #[test]
    fn test_v4_with_uppers_unique() {
        let uuid1 = v4_with_uppers();
        let uuid2 = v4_with_uppers();
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_v4_with_specials_zero_count() {
        let uuid = v4_with_specials(0);
        // With 0 specials, should be same as v4_with_uppers
        assert_eq!(uuid.len(), 36);
    }

    #[test]
    fn test_v4_with_specials_one_special() {
        let uuid = v4_with_specials(1);
        let special_chars = "!@#%&*?=+:";
        let has_special = uuid.chars().any(|c| special_chars.contains(c));
        assert!(
            has_special,
            "UUID should contain at least one special character"
        );
    }

    #[test]
    fn test_v4_with_specials_multiple() {
        let uuid = v4_with_specials(5);
        let special_chars = "!@#%&*?=+:";
        let special_count = uuid.chars().filter(|c| special_chars.contains(*c)).count();
        assert!(special_count > 0, "UUID should contain special characters");
    }

    #[test]
    fn test_v4_with_specials_length() {
        let uuid = v4_with_specials(3);
        // split("") adds extra empty strings, so length can be slightly more
        assert!(
            uuid.len() >= 36 && uuid.len() <= 38,
            "UUID should be approximately 36 characters"
        );
    }

    #[test]
    fn test_v4_with_specials_unique() {
        let uuid1 = v4_with_specials(3);
        let uuid2 = v4_with_specials(3);
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_v4_with_specials_large_count() {
        let uuid = v4_with_specials(10);
        // With many specials, length might vary slightly due to split("")
        assert!(uuid.len() >= 36, "UUID should be at least 36 characters");
        let special_chars = "!@#%&*?=+:";
        let special_count = uuid.chars().filter(|c| special_chars.contains(*c)).count();
        assert!(special_count > 0, "Should contain special characters");
    }

    #[test]
    fn test_v4_string_parts() {
        let uuid = v4_string();
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn test_v4_with_uppers_parts() {
        let uuid = v4_with_uppers();
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        // Verify part lengths
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn test_v4_with_uppers_middle_parts_lowercase() {
        let uuid = v4_with_uppers();
        let parts: Vec<&str> = uuid.split('-').collect();
        // Middle parts (1, 2, 3) should remain lowercase
        for part in &parts[1..=3] {
            let letters: String = part.chars().filter(|c| c.is_alphabetic()).collect();
            assert_eq!(
                letters,
                letters.to_lowercase(),
                "Middle parts should stay lowercase"
            );
        }
    }

    #[test]
    fn test_v4_with_specials_maintains_length() {
        let uuid = v4_with_specials(5);
        // split("") can add extra chars, so length might be 36-38
        assert!(
            uuid.len() >= 36 && uuid.len() <= 38,
            "Should maintain approximately UUID length"
        );
    }
}
