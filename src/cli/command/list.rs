use anyhow::Result;
use clap::ArgMatches;
use passwords::{analyzer, scorer};

use super::util;

pub fn all(matches: &ArgMatches) -> Result<()> {
    let decrypt = matches.get_one::<bool>("decrypt");
    let db = util::setup_db(matches)?;
    match decrypt {
        Some(true) => decrypted_header(),
        Some(false) => encrypted_header(),
        None => encrypted_header(),
    }
    for i in db.iter() {
        let record = i.value().decrypt(db.store_pwd())?;
        match decrypt {
            Some(true) => {
                let analyzed = analyzer::analyze(record.creds.password.clone());
                let score = scorer::score(&analyzed);
                println!(
                    "{: <40} | {: <30} | {: <20} | {:.2}",
                    record.metadata().url,
                    record.creds.user,
                    record.creds.password,
                    score
                )
            }
            Some(false) => println!("{: <40} | {}", record.metadata().url, record.creds.user),
            None => println!("{:?}", record),
        }
    }
    Ok(())
}

const URL_HEADER: &str = "URL";
const USER_HEADER: &str = "User / Account";
const PWD_HEADER: &str = "Password";
const SCORE_HEADER: &str = "Strength";

fn decrypted_header() {
    println!(
        "{: <40} | {: <30} | {: <20} | {}",
        URL_HEADER, USER_HEADER, PWD_HEADER, SCORE_HEADER
    );
    println!(
        "{: <40}-+-{: <30}-+-{: <20}-+-{}",
        "-".repeat(40),
        "-".repeat(30),
        "-".repeat(20),
        "-".repeat(10)
    )
}

fn encrypted_header() {
    println!("{: <40} | {: <30}", URL_HEADER, USER_HEADER);
    println!("{: <40}-+-{}", "-".repeat(40), "-".repeat(30))
}
