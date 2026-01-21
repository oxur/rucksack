//! # Password Generator
//!
//! Use a UUID:
//!
//! ```shell
//! rucksack gen --type uuid
//!
//! New password: 229ef9b4-b95b-4d91-a6ac-f6b7ef1cfc47
//! Password score: 88.50
//! ```
//!
//! Augmented UUID:
//!
//! ```shell
//! rucksack gen --type uuid++
//!
//! New password: 4C7360%E-4@60-4?03-b559-491C8A52E750
//! Password score: 100.00
//! ```
//!
//! Random:
//!
//! ```shell
//! rucksack gen --type random
//!
//! New password: A&6YU?#xk.?)
//! Password score: 91.22
//! ```
//!
//! Lorem-ipsum inspired:
//!
//! ```shell
//! rucksack gen --type lipsum
//!
//! New password: Esse-maius-amicitia,-nihil.-]9^,
//! Password score: 100.00
//! ```
//!
//! Some systems can't handle special characters, so a flag is available for encoding with base64, with the generated encoding getting scored:
//!
//! ```shell
//! rucksack gen --type lipsum --encode
//!
//! New password: VmVydW0sLW9waW5vciwtc2NyaXB0b3JlbS10YW1lbi4tLjYrfQ
//! Password score: 100.00
//! ```
//!
//! Or how about a long random token, chock-a-block with tasty entropy?
//!
//! ```shell
//! rucksack gen --type random --length 256 --encode
//!
//! New password: VFdCQVM-MmVUUUNDTlEpbl4rLztrMlc0cHtMSjVodzs4OTRIK00kK2ZBc0dfcGpCe
//! zlFIXouY19Hd1R-NSskLV1kXDMpTkQtX1EkcltUOFcyLDRQbmpobnJML1lxQmtDZjg0clhoPUg_JmVS
//! Pz4pUDpGVjsseWZCPlx4JXtwZS1tekU4eUBHZGRhVlRwOi0oK1IsRHkzO0J0JSFOVSNbXSEsSDwjLFA
//! ocjtCXT0-XFNYeHI0JkJQdEJ1X0E5YWZFa2Yhc0VSZnYvVyhROC45WF8kak05PWYzLk52UzQoPWQqc3
//! YlJHpqbS85UXhzKnI6ZlhAPWdRLmZxcVZWQXM4fg
//! Password score: 100.00
//! ```
//!
use anyhow::Result;
use clap::ArgMatches;

use rucksack_lib::generator::{password, uuid};

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
