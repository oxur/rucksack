//! # Listing Secrets
//!
//! Show all secrets records:
//!
//! ```shell
//! rucksack list
//! ```
//!
//! ```shell
//! Enter db password:
//! ```
//!
//! Show URLs, names, passwords, and password scores for all secrets:
//!
//! ```shell
//! rucksack list --decrypt
//! ```
//!
//! ```shell
//! Enter db password:
//! ```
//!
//! In both cases a password may be passed with the `--db-pass` flag. By default, the salt is the value of the `USER` environment variable, but it may be overridden with the `--salt` flag.
//!
//! Note that without `--decrypt`, only the user and URL are displayed. With `--decrypt`, those as well as masked password and password score are displayed. To unmask the password, one must also set `--reveal`.
//!
//! The default database location depends upon operating system. To see the location for your system, you can run `rucksack show db-file`. To use another location, the `--db` flag is available.
//!
//! The flags `--db`, `--db-pass`, and `--salt` may be set for any subcommand that access the database.
//!
//! # Searching / Filtering Secrets
//!
//! Simple filtering is also possible (done using a flag with the `list` command, with or without sorting):
//!
//! ```shell
//! rucksack list \
//!   --db-pass abc123 \
//!   --filter exa \
//!   --sort-by score \
//!   --decryspt
//! ```
//!
//! ```text
//! URL                                      | User / Record                 | Password             | Strength
//! -----------------------------------------+--------------------------------+----------------------+-----------
//! https://www.bugworld.com                 | hexapod123                     | **********           | 93
//! https://accounts.cloud.com               | hexapod@thing.systems          | **********           | 90
//! https://entymology.slack.com             | 6pod@example.com               | **********           | 86
//! https://bugs.slack.com                   | Alice "Hexapod" Roberts        | **********           | 85
//! https://twitter.com                      | TheOtherHexapod                | **********           | 83
//! https://portal-hexapod.testing.app       | alice@example.com              | **********           | 58
//! http://localhost:3000                    | alice@example.com              | **********           | 30
//!
//! 7 records (of 7 total)
//! ```
//!
//! It is also possible to perform negative filtering using `--exclude`. Additionally, `--include` is provided as an alias for `--filter`.
//!
//! You may sort on `score` (strength), `user`, or `url`. If not provided, `url` sorting is used. Also note that `order-by` is provided as an alias for `sort-by`.
//!
//! ## Additional Searching
//!
//! You may also limit results with the following:
//!
//! * by type of record with `--type`
//! * by user-supplied category with `--category`
//! * by tags, where `--all-tags` will only match records that have all the supplied tags, and where `--any-tags` will match any record that has at least one of the tags listed (both are supplied comma-separated; tags with spaces need to be quoted)
//!
//! The list of supported types may be shown with: `rucksack show types` and doesn't need access to the database to do so.
//!
//! A full list of categories created by the user does need access to the database (so you will be prompted for a password if you don't use the `--db-pass` flag): `rucksack show categories`.
//!
//! Same for user-created tags: `rucksack show tags`.
//!
//! ### Grouping Results
//!
//! #### By Password
//!
//! For use in auditing, sites+user combinations that share the same password can be reported:
//!
//! ```shell
//! rucksack list \
//!   --group-by db-pass \
//!   --decrypt
//! ```
//!
//! ```text
//! +========================================================================
//!
//! Password: ********** (Score: 99)
//! Records using: 5
//! Records:
//!
//! URL                                      | User / Record
//! -----------------------------------------+-------------------------------
//! https://smile.amazon.com                 | alice@example.com
//! https://smile.amazon.com/ap/signin       | alice@example.com
//! https://www.amazon.com                   | alice@example.com
//! https://www.amazon.com/ap/signin         | alice@example.com
//! https://mybank.com                       | alice@example.com
//!
//! +========================================================================
//!
//! Password: ********** (Score: 86)
//! Records using: 2
//! Records:
//!
//! URL                                      | User / Record
//! -----------------------------------------+-------------------------------
//! https://blurp.com                        | alice
//! https://bleep.net                        | alice
//!
//! 2 groups (with 7 records out of 16 total)
//! ```
//!
//! ## By User
//!
//! You may also group by user name (account name):
//!
//! ```shell
//! rucksack list \
//!   --group-by user \
//!   --decrypt
//! ```
//!
use std::collections::HashMap;

use anyhow::Result;
use clap::ArgMatches;
use passwords::{analyzer, scorer};

use rucksack_db::records;
use rucksack_db::Status;
use rucksack_lib::time;

use crate::app::App;
use crate::option;
use crate::query;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct ListResult {
    id: String,
    user: String,
    url: String,
    pwd: String,
    access_count: u64,
    score: i64,
    status: String,
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

#[derive(Default)]
pub struct Opts {
    pub skip_deleted: bool,
    pub only_deleted: bool,
    pub with_status: bool,
    pub reveal: bool,
}

// TODO: once there's config for it, pull from config and pass
// options here from top-level app.
pub fn all(matches: &ArgMatches, app: &App) -> Result<()> {
    process_records(
        matches,
        app,
        Opts {
            skip_deleted: true,
            ..Default::default()
        },
    )
}

// TODO: once there's config for it, pull from config and pass
// options here from top-level app.
pub fn deleted(matches: &ArgMatches, app: &App) -> Result<()> {
    process_records(
        matches,
        app,
        Opts {
            only_deleted: true,
            ..Default::default()
        },
    )
}

pub fn passwords(matches: &ArgMatches, app: &App) -> Result<()> {
    let reveal = matches.get_one::<bool>("reveal").unwrap();
    let record = query::record(&app.db, matches)?;
    log::debug!("temp: {reveal:}");
    log::debug!("temp: {record:?}");
    // TODO: print header
    // TODO iterate over password, displaying data
    Ok(())
}

fn process_records(matches: &ArgMatches, app: &App, mut opts: Opts) -> Result<()> {
    let decrypt = matches.get_one::<bool>("decrypt");
    let filter = matches.get_one::<String>("filter");
    let exclude = matches.get_one::<String>("exclude");
    let max_score = matches.get_one::<f64>("max-score");
    let min_score = matches.get_one::<f64>("min-score");
    let reveal = matches.get_one::<bool>("reveal").unwrap();
    let sort_by = matches.get_one::<String>("sort-by").map(|s| s.as_str());
    let group_by = matches.get_one::<String>("group-by").map(|s| s.as_str());
    let kind = option::record_kind(matches);
    let category = option::category(matches);
    let all_tags = option::all_tags(matches);
    let any_tags = option::any_tags(matches);
    opts.reveal = *reveal;
    // If we want to see the status of all records, we're going to override
    // skip_deleted and only_deleted:
    match matches.get_one::<bool>("with-status") {
        Some(true) => {
            opts.only_deleted = false;
            opts.skip_deleted = false;
            opts.with_status = true;
        }
        Some(_) => (),
        None => (),
    }
    let mut results: Vec<ListResult> = Vec::new();
    let mut groups = GroupByString::new();
    for i in app.db.iter() {
        let record = i.value().decrypt(app.db.store_pwd(), app.db.salt())?;
        let analyzed = analyzer::analyze(record.password());
        let score = scorer::score(&analyzed);
        let mut result = new_result(record.key(), record.name_or_user(), record.metadata().url);
        // If we're only showing non-deleted records and the record has been
        // deleted, move on to the next one:
        if opts.skip_deleted && record.metadata().state == Status::Deleted {
            continue;
        }
        // If we're only showing deleted records and the record hasn't been
        // deleted, move on to the next one:
        if opts.only_deleted && record.metadata().state != Status::Deleted {
            continue;
        }
        if kind != records::Kind::Any && kind != record.metadata().kind {
            continue;
        }
        if category != *records::ANY_CATEGORY && record.metadata().category != category {
            continue;
        };
        if let Some(ref ts) = all_tags {
            if !rucksack_lib::util::all(ts.clone(), record.metadata().tag_values()) {
                continue;
            }
        }
        if let Some(ref ts) = any_tags {
            if !rucksack_lib::util::any(ts.clone(), record.metadata().tag_values()) {
                continue;
            }
        }
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
        let md = record.metadata();
        result.access_count = md.access_count;
        result.status = md.status().to_string();
        match decrypt {
            Some(true) => {
                let pwd = if opts.reveal {
                    record.password()
                } else {
                    hidden()
                };
                result.pwd = pwd;
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
            let (group_count, record_count) = print_password_group(groups, decrypt, sort_by, &opts);
            print_group_report(group_count, record_count, app.db.hash_map().len());
        }
        Some("user") => {
            let (group_count, record_count) = print_user_group(groups, decrypt, sort_by, &opts);
            print_group_report(group_count, record_count, app.db.hash_map().len());
        }
        Some(&_) => (),
        None => {
            print_results(&results, decrypt, &opts);
            print_report(results.len(), app.db.hash_map().len());
        }
    }
    // With the dash_map iteration finished, the lock is gone, and we can
    // now update all the records whose passwords were revealed:
    for r in results {
        if opts.reveal {
            if let Some(mut metadata) = app.db.get_metadata(r.id()) {
                metadata.last_used = time::now();
                metadata.access_count += 1;
                app.db.update_metadata(r.id(), metadata);
            }
        }
    }
    app.db.close()?;
    Ok(())
}

fn print_results(sorted: &Vec<ListResult>, decrypted: Option<&bool>, opts: &Opts) {
    match decrypted {
        Some(true) => decrypted_header(opts),
        Some(false) => encrypted_header(opts),
        None => unreachable!(),
    }
    for r in sorted {
        match decrypted {
            Some(true) => decrypted_result(r, opts),
            Some(false) => encrypted_result(r, opts),
            None => unreachable!(),
        }
    }
}

fn print_report(count: usize, total: usize) {
    println!("\n{count} records (of {total} total)\n")
}

fn print_password_group(
    groups: GroupByString,
    decrypted: Option<&bool>,
    sort_by: Option<&str>,
    opts: &Opts,
) -> (i32, usize) {
    let mut group_count = 0;
    let mut record_count = 0;
    for (_, mut group) in groups {
        if group.len() == 1 {
            continue;
        }
        group_count += 1;
        sort(&mut group, sort_by);
        password_section(&group[0], decrypted, opts);
        println!("Records using: {}\nRecords:", group.len());
        encrypted_header(opts);
        record_count += group.len();
        for r in group {
            encrypted_result(&r, opts)
        }
    }
    (group_count, record_count)
}

fn print_user_group(
    groups: GroupByString,
    decrypted: Option<&bool>,
    sort_by: Option<&str>,
    opts: &Opts,
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
        println!("Records using: {}\nRecords:", group.len());
        match decrypted {
            Some(true) => decrypted_no_user_header(opts),
            Some(false) => encrypted_no_user_header(opts),
            None => unreachable!(),
        }
        record_count += group.len();
        for r in group {
            match decrypted {
                Some(true) => decrypted_no_user_result(&r, opts),
                Some(false) => encrypted_no_user_result(&r, opts),
                None => unreachable!(),
            }
        }
    }
    (group_count, record_count)
}

fn print_group_report(count: i32, records: usize, total: usize) {
    println!("\n{count} groups (with {records} records out of {total} total)\n",)
}

const URL_HEADER: &str = "URL";
const USER_HEADER: &str = "User / Record";
const PWD_HEADER: &str = "Password";
const SCORE_HEADER: &str = "Score / Strength";
const COUNT_HEADER: &str = "Access Count";
const STATUS_HEADER: &str = "Status";

fn decrypted_header(opts: &Opts) {
    if opts.with_status {
        println!(
            "\n{USER_HEADER: <30} | {PWD_HEADER: <20} | {SCORE_HEADER: <15} | {COUNT_HEADER: <12} | {STATUS_HEADER} | {URL_HEADER}",
        );
        println!(
            "{: <30}-+-{: <20}-+-{: <15}-+-{}-+-{: <12}-+-{}",
            "-".repeat(30),
            "-".repeat(20),
            "-".repeat(16),
            "-".repeat(12),
            "-".repeat(8),
            "-".repeat(40),
        )
    } else {
        println!(
            "\n{USER_HEADER: <30} | {PWD_HEADER: <20} | {SCORE_HEADER: <15} | {COUNT_HEADER: <12} | {URL_HEADER: <40}",
        );
        println!(
            "{: <30}-+-{: <20}-+-{: <15}-+-{: <12}-+-{}",
            "-".repeat(30),
            "-".repeat(20),
            "-".repeat(16),
            "-".repeat(12),
            "-".repeat(40),
        )
    }
}

fn decrypted_no_user_header(opts: &Opts) {
    if opts.with_status {
        println!("\n{PWD_HEADER: <20} | {SCORE_HEADER: <16} | {COUNT_HEADER: <12} | {URL_HEADER}",);
        println!(
            "{: <20}-+-{: <16}-+-{: <12}-+-{}",
            "-".repeat(20),
            "-".repeat(16),
            "-".repeat(12),
            "-".repeat(40),
        )
    } else {
        println!("\n{PWD_HEADER: <20} | {SCORE_HEADER: <16} | {URL_HEADER}",);
        println!(
            "{: <20}-+-{: <16}-+-{}",
            "-".repeat(20),
            "-".repeat(16),
            "-".repeat(40),
        )
    }
}

fn encrypted_header(opts: &Opts) {
    if opts.with_status {
        println!("\n{USER_HEADER: <30} | {COUNT_HEADER: <12} | {STATUS_HEADER} | {URL_HEADER}",);
        println!(
            "{:30}-+-{:12}-+-{:8}-+-{}",
            "-".repeat(30),
            "-".repeat(12),
            "-".repeat(8),
            "-".repeat(40),
        )
    } else {
        println!("\n{USER_HEADER: <30} | {COUNT_HEADER} | {URL_HEADER}",);
        println!(
            "{:30}-+-{:12}-+-{}",
            "-".repeat(30),
            "-".repeat(12),
            "-".repeat(40),
        )
    }
}

fn encrypted_no_user_header(opts: &Opts) {
    if opts.with_status {
        println!("\n{COUNT_HEADER: <12} | {STATUS_HEADER: <8} | {URL_HEADER}",);
        println!(
            "{:12}-+-{:8}-+-{}",
            "-".repeat(12),
            "-".repeat(8),
            "-".repeat(40),
        )
    } else {
        println!("\n{COUNT_HEADER} | {URL_HEADER: <40}");
        println!("{:12}-+-{}", "-".repeat(40), "-".repeat(12))
    }
}

fn decrypted_result(r: &ListResult, opts: &Opts) {
    if opts.with_status {
        println!(
            "{: <30} | {: <20} | {: ^16.2} | {: ^12} | {: ^8} | {} ",
            r.user, r.pwd, r.score, r.access_count, r.status, r.url,
        )
    } else {
        println!(
            "{: <30} | {: <20} | {: ^16.2} | {: ^12} | {}",
            r.user, r.pwd, r.score, r.access_count, r.url,
        )
    }
}

fn password_section(r: &ListResult, decrypted: Option<&bool>, opts: &Opts) {
    println!("\n\n+{}\n", "=".repeat(40 + 30 + 2));
    match decrypted {
        Some(true) => {
            if opts.reveal {
                println!("Password: {} (Score: {})", r.pwd, r.score)
            } else {
                println!("Password: {} (Score: {})", hidden(), r.score)
            }
        }
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

fn encrypted_result(r: &ListResult, opts: &Opts) {
    if opts.with_status {
        println!(
            "{: <30} | {: ^12} | {: <8} | {}",
            r.user, r.access_count, r.status, r.url,
        )
    } else {
        println!("{: <30} | {: ^12} | {}", r.user, r.access_count, r.url)
    }
}

fn decrypted_no_user_result(r: &ListResult, opts: &Opts) {
    if opts.with_status {
        println!(
            "{: <20} | {: ^16.2} | {: ^12} | {: <8} | {}",
            r.pwd, r.score, r.access_count, r.status, r.url,
        )
    } else {
        println!(
            "{: <20} | {: ^16.2} | {: ^12} | {}",
            r.pwd, r.score, r.access_count, r.url,
        )
    }
}

fn encrypted_no_user_result(r: &ListResult, opts: &Opts) {
    if opts.with_status {
        println!("{: ^12} | {: <8} | {}", r.access_count, r.status, r.url)
    } else {
        println!("{: ^12} | {}", r.access_count, r.url)
    }
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
