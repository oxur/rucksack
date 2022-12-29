use anyhow::{Result};
use clap::ArgMatches;

// pub fn generate(matches: &ArgMatches, config: &config::Config) -> Result<()> {
pub fn generate(matches: &ArgMatches) -> Result<()> {
    match matches.get_one::<String>("type").map(|s| s.as_str()) {
        Some("uuid") => generate_pwd_uuid(),
        Some("default") => generate_pwd_uuid(),
        Some(_) => todo!(),
        None => todo!(),
    }
}

fn generate_pwd_uuid() -> Result<()> {
    let pwd = uuid::Uuid::new_v4().to_string();
    display(&pwd)
}

pub fn version(version: &str) -> Result<()> {
    display(version)
}

fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}
