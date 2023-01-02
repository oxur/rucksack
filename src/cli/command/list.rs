use anyhow::Result;
use clap::ArgMatches;

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
                println!(
                    "{: <40} | {: <30} | {}",
                    record.metadata().url,
                    record.creds.user,
                    record.creds.password
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

fn decrypted_header() {
    println!(
        "{: <40} | {: <30} | {}",
        URL_HEADER, USER_HEADER, PWD_HEADER
    );
    println!(
        "{: <40}-+-{: <30}-+-{}",
        "-".repeat(40),
        "-".repeat(30),
        "-".repeat(20)
    )
}

fn encrypted_header() {
    println!("{: <40} | {: <30}", URL_HEADER, USER_HEADER);
    println!("{: <40}-+-{}", "-".repeat(40), "-".repeat(30))
}
