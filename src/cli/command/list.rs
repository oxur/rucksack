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

fn decrypted_header() {
    println!("{} | {} | {}", "URL", "User / Account", "Password");
    println!("{}-+-{}-+-{}", "---", "--------------", "--------")
}

fn encrypted_header() {
    println!("{} | {}", "URL", "User / Account");
    println!("{}-+-{}", "---", "--------------")
}
