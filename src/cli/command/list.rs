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
                    "{} | {} | {}",
                    record.metadata().url,
                    record.creds.user,
                    record.creds.password
                )
            }
            Some(false) => println!("{} | {}", record.metadata().url, record.creds.user),
            None => println!("{:?}", record),
        }
    }
    Ok(())
}

const URL_HEADER: &str = "URL";
const USER_HEADER: &str = "User / Account";
const PWD_HEADER: &str = "Password";

fn decrypted_header() {
    println!("{} | {} | {}", URL_HEADER, USER_HEADER, PWD_HEADER);
    println!(
        "{}-+-{}-+-{}",
        "-".repeat(URL_HEADER.len()),
        "-".repeat(USER_HEADER.len()),
        "-".repeat(PWD_HEADER.len())
    )
}

fn encrypted_header() {
    println!("{} | {}", URL_HEADER, USER_HEADER);
    println!(
        "{}-+-{}",
        "-".repeat(URL_HEADER.len()),
        "-".repeat(USER_HEADER.len())
    )
}
