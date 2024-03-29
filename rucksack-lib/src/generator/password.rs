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

pub fn rand(length: &usize) -> String {
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
    pg.generate_one().unwrap()
}

pub fn lipsum(word_count: &usize, suffix_length: &usize, delim: &str) -> String {
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
    words.push(pg.generate_one().unwrap());
    words.join(delim)
}
