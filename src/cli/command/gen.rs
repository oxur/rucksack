use anyhow::Result;
use clap::ArgMatches;

use crate::generator::{password, uuid};

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
    password::display_scored(&password::rand(length))
}

fn generate_pwd_lipsum(matches: &ArgMatches) -> Result<()> {
    let delimiter = matches
        .get_one::<String>("delimiter")
        .map(|s| s.as_str())
        .unwrap();
    let suffix_length = matches.get_one::<usize>("suffix-length").unwrap();
    let word_count = matches.get_one::<usize>("word-count").unwrap();
    password::display_scored(&password::lipsum(word_count, suffix_length, delimiter))
}

fn generate_pwd_uuid() -> Result<()> {
    password::display_scored(&uuid::v4_string())
}

fn generate_pwd_uuid_plus() -> Result<()> {
    password::display_scored(&uuid::v4_with_uppers())
}

fn generate_pwd_uuid_special() -> Result<()> {
    let number_of_specials = 3;
    password::display_scored(&uuid::v4_with_specials(number_of_specials))
}
