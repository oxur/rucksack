use std::collections::HashMap;

use anyhow::Result;
use clap::ArgMatches;
use passwords::{analyzer, scorer};

use crate::app::App;
use crate::store::db::V1;
use crate::time;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct ListResult {
    id: String,
    user: String,
    url: String,
    pwd: String,
    access_count: u64,
    score: i64,
}

fn new_result(id: String, user: String, url: String) -> ListResult {
    ListResult {
        id,
        user,
        url,

        ..Default::default()
    }
}

impl ListResult {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

type GroupByString = HashMap<String, Vec<ListResult>>;

pub fn all(matches: &ArgMatches, app: &App) -> Result<()> {
    let decrypt = matches.get_one::<bool>("decrypt");
    let filter = matches.get_one::<String>("filter");
    let exclude = matches.get_one::<String>("exclude");
    let max_score = matches.get_one::<f64>("max-score");
    let min_score = matches.get_one::<f64>("min-score");
    let reveal = matches.get_one::<bool>("reveal");
    let sort_by = matches.get_one::<String>("sort-by").map(|s| s.as_str());
    let group_by = matches.get_one::<String>("group-by").map(|s| s.as_str());
    let mut results: Vec<ListResult> = Vec::new();
    let mut groups = GroupByString::new();
    for i in app.db.iter() {
        let record = i.value().decrypt(app.db.store_pwd(), app.db.salt())?;
        let analyzed = analyzer::analyze(record.password());
        let score = scorer::score(&analyzed);
        let mut result = new_result(record.key(), record.user(), record.metadata().url);
        if let Some(check) = filter {
            if !i.key().contains(check) {
                continue;
            }
        }
        if let Some(check) = exclude {
            if i.key().contains(check) {
                continue;
            }
        }
        if let Some(check) = max_score {
            if &score.trunc() > check {
                continue;
            }
        }
        if let Some(check) = min_score {
            if &score.trunc() < check {
                continue;
            }
        }
        match decrypt {
            Some(true) => {
                let pwd = match reveal {
                    Some(true) => record.password(),
                    Some(false) => hidden(),
                    None => unreachable!(),
                };
                result.pwd = pwd;
                result.access_count = record.metadata().access_count;
                result.score = score.trunc() as i64;
            }
            Some(false) => result.pwd = hidden(),
            None => unreachable!(),
        }
        match group_by {
            Some("password") => {
                let entry = groups.entry(record.password()).or_default();
                entry.push(result.clone());
            }
            Some("user") => {
                let entry = groups.entry(record.user()).or_default();
                entry.push(result.clone());
            }
            Some(&_) => (),
            None => (),
        }
        results.push(result);
    }
    sort(&mut results, sort_by);
    match group_by {
        Some("password") => {
            let (group_count, record_count) =
                print_password_group(groups, decrypt, reveal, sort_by);
            print_group_report(group_count, record_count, app.db.hash_map().len());
        }
        Some("user") => {
            let (group_count, record_count) = print_user_group(groups, decrypt, sort_by);
            print_group_report(group_count, record_count, app.db.hash_map().len());
        }
        Some(&_) => (),
        None => {
            print_results(&results, decrypt);
            print_report(results.len(), app.db.hash_map().len());
        }
    }
    // With the dash_map iteration finished, the lock is gone, and we can
    // now update all the records whose passwords were revealed:
    for r in results {
        match reveal {
            Some(true) => {
                if let Some(mut metadata) = app.db.get_metadata(r.id()) {
                    metadata.last_used = time::now();
                    metadata.access_count += 1;
                    app.db.update_metadata(r.id(), metadata);
                }
            }
            Some(false) => (),
            None => unreachable!(),
        }
    }
    app.db.close()?;
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

fn print_password_group(
    groups: GroupByString,
    decrypted: Option<&bool>,
    reveal: Option<&bool>,
    sort_by: Option<&str>,
) -> (i32, usize) {
    let mut group_count = 0;
    let mut record_count = 0;
    for (_, mut group) in groups {
        if group.len() == 1 {
            continue;
        }
        group_count += 1;
        sort(&mut group, sort_by);
        password_section(&group[0], decrypted, reveal);
        println!("Accounts using: {}\nAccounts:", group.len());
        encrypted_header();
        record_count += group.len();
        for r in group {
            encrypted_result(&r)
        }
    }
    (group_count, record_count)
}

fn print_user_group(
    groups: GroupByString,
    decrypted: Option<&bool>,
    sort_by: Option<&str>,
) -> (i32, usize) {
    let mut group_count = 0;
    let mut record_count = 0;
    for (_, mut group) in groups {
        if group.len() == 1 {
            continue;
        }
        group_count += 1;
        sort(&mut group, sort_by);
        user_section(&group[0], decrypted);
        println!("Accounts using: {}\nAccounts:", group.len());
        match decrypted {
            Some(true) => decrypted_no_user_header(),
            Some(false) => encrypted_no_user_header(),
            None => unreachable!(),
        }
        record_count += group.len();
        for r in group {
            match decrypted {
                Some(true) => decrypted_no_user_result(&r),
                Some(false) => encrypted_no_user_result(&r),
                None => unreachable!(),
            }
        }
    }
    (group_count, record_count)
}

fn print_group_report(count: i32, records: usize, total: usize) {
    println!(
        "\n{} groups (with {} records out of {} total)\n",
        count, records, total
    )
}

const URL_HEADER: &str = "URL";
const USER_HEADER: &str = "User / Account";
const PWD_HEADER: &str = "Password";
const SCORE_HEADER: &str = "Score / Strength";
const COUNT_HEADER: &str = "Access Count";

fn decrypted_header() {
    println!(
        "\n{: <40} | {: <30} | {: <20} | {: <15} | {}",
        URL_HEADER, USER_HEADER, PWD_HEADER, SCORE_HEADER, COUNT_HEADER
    );
    println!(
        "{: <40}-+-{: <30}-+-{: <20}-+-{: <15}-+-{}",
        "-".repeat(40),
        "-".repeat(30),
        "-".repeat(20),
        "-".repeat(16),
        "-".repeat(12),
    )
}

fn decrypted_no_user_header() {
    println!(
        "\n{: <40} | {: <20} | {}",
        URL_HEADER, PWD_HEADER, SCORE_HEADER
    );
    println!(
        "{: <40}-+-{: <20}-+-{}",
        "-".repeat(40),
        "-".repeat(20),
        "-".repeat(16)
    )
}

fn encrypted_header() {
    println!(
        "\n{: <40} | {: <30} | {}",
        URL_HEADER, USER_HEADER, COUNT_HEADER
    );
    println!(
        "{:40}-+-{:30}-+-{}",
        "-".repeat(40),
        "-".repeat(30),
        "-".repeat(12)
    )
}

fn encrypted_no_user_header() {
    println!("\n{: <40} | {}", URL_HEADER, COUNT_HEADER);
    println!("{:40}-+-{}", "-".repeat(40), "-".repeat(12))
}

fn decrypted_result(r: &ListResult) {
    println!(
        "{: <40} | {: <30} | {: <20} | {: ^16.2} | {: ^12}",
        r.url, r.user, r.pwd, r.score, r.access_count
    )
}

fn password_section(r: &ListResult, decrypted: Option<&bool>, reveal: Option<&bool>) {
    println!("\n\n+{}\n", "=".repeat(40 + 30 + 2));
    match decrypted {
        Some(true) => match reveal {
            Some(true) => println!("Password: {} (Score: {})", r.pwd, r.score),
            Some(false) => println!("Password: {} (Score: {})", hidden(), r.score),
            None => unreachable!(),
        },
        Some(false) => println!("Password Group"),
        None => unreachable!(),
    }
}

fn user_section(r: &ListResult, decrypted: Option<&bool>) {
    match decrypted {
        Some(true) => {
            println!("\n\n+{}\n", "=".repeat(40 + 20 + 16 + 5));
            println!("User: {}", r.user)
        }
        Some(false) => {
            println!("\n\n+{}\n", "=".repeat(40 - 1));
            println!("User: {}", r.user)
        }
        None => unreachable!(),
    }
}

fn encrypted_result(r: &ListResult) {
    println!("{: <40} | {: <30} | {: ^12}", r.url, r.user, r.access_count)
}

fn decrypted_no_user_result(r: &ListResult) {
    println!(
        "{: <40} | {: <20} | {: ^16.2} | {}",
        r.url, r.pwd, r.score, r.access_count
    )
}

fn encrypted_no_user_result(r: &ListResult) {
    println!("{: <40} | {: ^12}", r.url, r.access_count)
}

fn hidden() -> String {
    "*".repeat(10)
}

fn sort(results: &mut [ListResult], sort_by: Option<&str>) {
    match sort_by {
        Some("score") => results.sort_by(|a, b| b.score.cmp(&a.score)),
        Some("url") => results.sort(),
        Some("user") => results.sort_by(|a, b| a.user.cmp(&b.user)),
        Some(&_) => (),
        None => (),
    };
}
