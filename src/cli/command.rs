use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use anyhow::{Result};
use clap::ArgMatches;
use passwords::{analyzer, scorer};

const SPECIALS: &[u8] = b"!@#%&*?=+:";

// pub fn generate(matches: &ArgMatches, config: &config::Config) -> Result<()> {
pub fn generate(matches: &ArgMatches) -> Result<()> {
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("uuid") => generate_pwd_uuid(),
        Some("uuid+") => generate_pwd_uuid_plus(),
        Some("uuid++") => generate_pwd_uuid_special(),
        Some("default") => generate_pwd_uuid(),
        Some(_) => todo!(),
        None => todo!(),
    }
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

pub fn version(version: &str) -> Result<()> {
    display(version)
}

fn display_scored_password(pwd: &str) -> Result<()> {
    let analyzed = analyzer::analyze(pwd);
    let score = scorer::score(&analyzed);
    let msg = format!("\nNew password: {}\nPassword score: {:.2}\n", pwd, score);
    display(&msg)
}

fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

fn uuid4_string() -> String {
    uuid::Uuid::new_v4().to_string()
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
