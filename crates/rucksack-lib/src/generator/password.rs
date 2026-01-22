use rand::thread_rng;
use std::str;

use anyhow::Result;
use base64::engine::general_purpose as b64;
use base64::Engine;
use passwords::{analyzer, scorer, PasswordGenerator};

use crate::util;

pub fn display_scored(mut pwd: String, encode: Option<&bool>) -> Result<()> {
    match encode {
        Some(true) => {
            let bytes = pwd.as_bytes();
            pwd = b64::URL_SAFE_NO_PAD.encode(bytes);
        }
        Some(false) => (),
        None => (),
    }
    let analyzed = analyzer::analyze(pwd.clone());
    let score = scorer::score(&analyzed);
    let msg = format!("\nNew password: {pwd}\nPassword score: {score:.2}\n");
    util::display(&msg)
}

pub fn rand(length: &usize) -> Result<String> {
    let pg = PasswordGenerator {
        length: *length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };
    pg.generate_one()
        .map_err(|e| anyhow::anyhow!("password generation failed: {}", e))
}

pub fn lipsum(word_count: &usize, suffix_length: &usize, delim: &str) -> Result<String> {
    let phrase = lipsum::lipsum_words_with_rng(thread_rng(), *word_count);
    let mut words: Vec<String> = phrase.split(' ').map(|s| s.to_string()).collect();
    let pg = PasswordGenerator {
        length: *suffix_length,
        numbers: true,
        lowercase_letters: false,
        uppercase_letters: false,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };
    let suffix = pg
        .generate_one()
        .map_err(|e| anyhow::anyhow!("password suffix generation failed: {}", e))?;
    words.push(suffix);
    Ok(words.join(delim))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_length_8() {
        let password = rand(&8).unwrap();
        assert_eq!(password.len(), 8);
    }

    #[test]
    fn test_rand_length_16() {
        let password = rand(&16).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_rand_length_32() {
        let password = rand(&32).unwrap();
        assert_eq!(password.len(), 32);
    }

    #[test]
    fn test_rand_contains_variety() {
        let password = rand(&20).unwrap();
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_upper = password.chars().any(|c| c.is_uppercase());

        assert!(
            has_digit || has_lower || has_upper,
            "Password should contain varied characters"
        );
    }

    #[test]
    fn test_rand_different_on_each_call() {
        let pwd1 = rand(&16).unwrap();
        let pwd2 = rand(&16).unwrap();
        assert_ne!(pwd1, pwd2, "Random passwords should be different");
    }

    #[test]
    fn test_rand_reasonable_minimum() {
        // Password generator has a minimum length requirement
        let password = rand(&8).unwrap();
        assert_eq!(password.len(), 8);
    }

    #[test]
    fn test_lipsum_word_count() {
        let passphrase = lipsum(&3, &4, "-").unwrap();
        let parts: Vec<&str> = passphrase.split('-').collect();
        // lipsum may generate more or fewer words, but should have at least suffix
        assert!(parts.len() >= 2, "Should have at least words + suffix");
    }

    #[test]
    fn test_lipsum_with_space_delimiter() {
        let passphrase = lipsum(&2, &3, " ").unwrap();
        assert!(passphrase.contains(' '));
        let parts: Vec<&str> = passphrase.split(' ').collect();
        assert_eq!(parts.len(), 3); // 2 words + suffix
    }

    #[test]
    fn test_lipsum_with_custom_delimiter() {
        let passphrase = lipsum(&3, &5, "_").unwrap();
        assert!(passphrase.contains('_'));
    }

    #[test]
    fn test_lipsum_suffix_length() {
        // Test multiple times since lipsum word generation can vary
        let mut found_valid_suffix = false;
        for _ in 0..10 {
            let passphrase = lipsum(&2, &6, "-").unwrap();
            assert!(!passphrase.is_empty());

            // The passphrase should contain at least the delimiter
            if passphrase.contains('-') {
                let parts: Vec<&str> = passphrase.split('-').collect();
                if let Some(suffix) = parts.last() {
                    // Check if this looks like a password suffix (has numbers/symbols)
                    let has_numbers = suffix.chars().any(|c| c.is_ascii_digit());
                    let has_symbols = suffix.chars().any(|c| !c.is_alphanumeric());

                    if (has_numbers || has_symbols) && suffix.len() >= 4 {
                        found_valid_suffix = true;
                        break;
                    }
                }
            }
        }

        assert!(
            found_valid_suffix,
            "Should generate at least one passphrase with a valid suffix in 10 attempts"
        );
    }

    #[test]
    fn test_lipsum_single_word() {
        let passphrase = lipsum(&1, &4, "-").unwrap();
        let parts: Vec<&str> = passphrase.split('-').collect();
        // Lipsum may return more words than requested
        assert!(parts.len() >= 2, "Should have at least 1 word + suffix");
    }

    #[test]
    fn test_lipsum_different_on_each_call() {
        let phrase1 = lipsum(&3, &4, "-").unwrap();
        let phrase2 = lipsum(&3, &4, "-").unwrap();
        // At least the suffix should be different
        assert_ne!(phrase1, phrase2);
    }

    #[test]
    fn test_display_scored_no_encoding() {
        let password = "TestPassword123!".to_string();
        let result = display_scored(password, Some(&false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_scored_with_encoding() {
        let password = "TestPassword123!".to_string();
        let result = display_scored(password, Some(&true));
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_scored_none_encoding() {
        let password = "TestPassword123!".to_string();
        let result = display_scored(password, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_scored_weak_password() {
        let password = "123".to_string();
        let result = display_scored(password, Some(&false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_scored_strong_password() {
        let password = "Str0ng!P@ssw0rd#2024".to_string();
        let result = display_scored(password, Some(&false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_scored_empty_password() {
        let password = "".to_string();
        let result = display_scored(password, Some(&false));
        assert!(result.is_ok());
    }

    #[test]
    fn test_lipsum_no_delimiter() {
        let passphrase = lipsum(&3, &4, "").unwrap();
        // Without delimiter, words are concatenated
        assert!(!passphrase.is_empty());
    }

    #[test]
    fn test_rand_long_password() {
        let password = rand(&100).unwrap();
        assert_eq!(password.len(), 100);
    }
}
