use anyhow::Result;
use clap::ArgMatches;
use passwords::{analyzer, scorer};

use super::util;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct ListResult {
    url: String,
    user: String,
    pwd: String,
    score: i64,
}

fn new_result(user: String, url: String) -> ListResult {
    ListResult {
        user,
        url,

        ..Default::default()
    }
}

pub fn all(matches: &ArgMatches) -> Result<()> {
    let decrypt = matches.get_one::<bool>("decrypt");
    let filter = matches.get_one::<String>("filter");
    let reveal = matches.get_one::<bool>("reveal");
    let sort_by = matches.get_one::<String>("sort-by").map(|s| s.as_str());
    let db = util::setup_db(matches)?;
    let mut results: Vec<ListResult> = Vec::new();
    for i in db.iter() {
        let record = i.value().decrypt(db.store_pwd(), db.salt())?;
        let mut result = new_result(record.user(), record.metadata().url);
        if let Some(check) = filter {
            if !i.key().contains(check) {
                continue;
            }
        }
        let hidden = "*".repeat(10).to_string();
        match decrypt {
            Some(true) => {
                let analyzed = analyzer::analyze(record.password());
                let score = scorer::score(&analyzed);
                let pwd = match reveal {
                    Some(true) => record.password(),
                    Some(false) => hidden,
                    None => unreachable!(),
                };
                result.pwd = pwd;
                result.score = score.trunc() as i64;
            }
            Some(false) => result.pwd = hidden,
            None => unreachable!(),
        }
        results.push(result)
    }
    match sort_by {
        Some("score") => results.sort_by(|a, b| b.score.cmp(&a.score)),
        Some("url") => results.sort(),
        Some("user") => results.sort_by(|a, b| a.user.cmp(&b.user)),
        Some(&_) => (),
        None => (),
    };
    print_results(&results, decrypt);
    print_report(results.len(), db.hash_map().len());
    Ok(())
}

fn print_results(sorted: &Vec<ListResult>, decrypted: Option<&bool>) {
    match decrypted {
        Some(true) => decrypted_header(),
        Some(false) => encrypted_header(),
        None => unreachable!(),
    }
    for r in sorted {
        match decrypted {
            Some(true) => decrypted_result(r),
            Some(false) => encrypted_result(r),
            None => unreachable!(),
        }
    }
}

fn print_report(count: usize, total: usize) {
    println!("\n{} records (of {} total)\n", count, total)
}

const URL_HEADER: &str = "URL";
const USER_HEADER: &str = "User / Account";
const PWD_HEADER: &str = "Password";
const SCORE_HEADER: &str = "Score / Strength";

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
        "-".repeat(16)
    )
}

fn encrypted_header() {
    println!("{: <40} | {: <30}", URL_HEADER, USER_HEADER);
    println!("{: <40}-+-{}", "-".repeat(40), "-".repeat(30))
}

fn decrypted_result(r: &ListResult) {
    println!(
        "{: <40} | {: <30} | {: <20} | {:.2}",
        r.url, r.user, r.pwd, r.score
    )
}

fn encrypted_result(r: &ListResult) {
    println!("{: <40} | {}", r.url, r.user)
}
