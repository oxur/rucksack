use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use anyhow::{Result};
use clap::ArgMatches;
use passwords::{analyzer, scorer, PasswordGenerator};
use uuid::{Uuid};

use super::util;

const SPECIALS: &[u8] = b"!@#%&*?=+:";

// pub fn generate(matches: &ArgMatches, config: &config::Config) -> Result<()> {
pub fn new(matches: &ArgMatches) -> Result<()> {
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("lipsum") => generate_pwd_lipsum(matches),
        Some("random") => generate_pwd(matches),
        Some("uuid") => generate_pwd_uuid(),
        Some("uuid+") => generate_pwd_uuid_plus(),
        Some("uuid++") => generate_pwd_uuid_special(),
        Some(_) => todo!(),
        None => todo!(),
    }
}

// Generator type dispatch functions

fn generate_pwd(matches: &ArgMatches) -> Result<()> {
    let length = matches.get_one::<usize>("length").unwrap();
    display_scored_password(&rand_pwd(length))
}

fn generate_pwd_lipsum(matches: &ArgMatches) -> Result<()> {
    let delimiter = matches.get_one::<String>("delimiter").map(|s| s.as_str()).unwrap();
    let suffix_length = matches.get_one::<usize>("suffix-length").unwrap();
    let word_count = matches.get_one::<usize>("word-count").unwrap();
    display_scored_password(&lipsum_pwd(word_count, suffix_length, delimiter))
}

fn generate_pwd_uuid() -> Result<()> {
    display_scored_password(&uuid4_string())
}

fn generate_pwd_uuid_plus() -> Result<()> {
    display_scored_password(&uuid4_with_uppers())
}

fn generate_pwd_uuid_special() -> Result<()> {
    let number_of_specials = 3;
    display_scored_password(&uuid4_with_specials(number_of_specials))
}

// Utility functions

fn display_scored_password(pwd: &str) -> Result<()> {
    let analyzed = analyzer::analyze(pwd);
    let score = scorer::score(&analyzed);
    let msg = format!("\nNew password: {}\nPassword score: {:.2}\n", pwd, score);
    util::display(&msg)
}

fn uuid4_string() -> String {
    Uuid::new_v4().to_string()
}

fn uuid4_with_uppers() -> String {
    let uuid = uuid4_string();
    let parts: Vec<&str> = uuid.split('-').collect();
    let (first_part, rest_parts) = parts.split_at(1);
    let first = first_part.to_vec().pop().unwrap().to_uppercase();
    let mut rest = rest_parts.to_vec();
    let last = rest.pop().unwrap().to_uppercase();
    rest.insert(0, &first);
    rest.push(&last);
    rest.join("-")
}

fn uuid4_with_specials(count: usize) -> String {
    let mut rng = rand::thread_rng();
    let uuid = uuid4_with_uppers();
    let mut parts: Vec<String> = uuid.split("").map(|s| s.to_string()).collect();
    let len = parts.len();
    let die = Uniform::from(1..len);
    let specials = random_specials(count);
    for special in specials.iter().take(count)  {
        let throw = die.sample(&mut rng);
        parts[throw] = String::from_utf8_lossy(&[*special]).to_string();
    }
    parts.join("")
}

fn random_specials(count: usize) -> Vec<u8> {
    let mut specials: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..count+1 {
        specials.push(SPECIALS[rng.gen_range(0..SPECIALS.len())])
    }
    specials
}

fn rand_pwd(length: &usize) -> String {
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

fn lipsum_pwd(word_count: &usize, suffix_length: &usize, delim: &str) -> String {
    let mut rng = rand::thread_rng();
    let phrase = lipsum::lipsum_words_from_seed(*word_count, rng.gen_range(0..10000));
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
