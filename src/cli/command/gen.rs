use anyhow::Result;
use clap::ArgMatches;

use crate::generator::{password, uuid};

// pub fn generate(matches: &ArgMatches, config: &config::Config) -> Result<()> {
pub fn new(matches: &ArgMatches) -> Result<()> {
    let encode = matches.get_one::<bool>("encode");
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("lipsum") => generate_pwd_lipsum(matches, encode),
        Some("random") => generate_pwd(matches, encode),
        Some("uuid") => generate_pwd_uuid(encode),
        Some("uuid+") => generate_pwd_uuid_plus(encode),
        Some("uuid++") => generate_pwd_uuid_special(encode),
        Some(_) => todo!(),
        None => todo!(),
    }
}

// Generator type dispatch functions

fn generate_pwd(matches: &ArgMatches, encode: Option<&bool>) -> Result<()> {
    let length = matches.get_one::<usize>("length").unwrap();
    password::display_scored(password::rand(length), encode)
}

fn generate_pwd_lipsum(matches: &ArgMatches, encode: Option<&bool>) -> Result<()> {
    let delimiter = matches
        .get_one::<String>("delimiter")
        .map(|s| s.as_str())
        .unwrap();
    let suffix_length = matches.get_one::<usize>("suffix-length").unwrap();
    let word_count = matches.get_one::<usize>("word-count").unwrap();
    password::display_scored(
        password::lipsum(word_count, suffix_length, delimiter),
        encode,
    )
}

fn generate_pwd_uuid(encode: Option<&bool>) -> Result<()> {
    password::display_scored(uuid::v4_string(), encode)
}

fn generate_pwd_uuid_plus(encode: Option<&bool>) -> Result<()> {
    password::display_scored(uuid::v4_with_uppers(), encode)
}

fn generate_pwd_uuid_special(encode: Option<&bool>) -> Result<()> {
    let number_of_specials = 3;
    password::display_scored(uuid::v4_with_specials(number_of_specials), encode)
}
