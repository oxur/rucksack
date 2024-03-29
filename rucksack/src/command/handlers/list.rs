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
//! You may also group by account name:
//!
//! ```shell
//! rucksack list \
//!   --group-by name \
//!   --decrypt
//! ```
//!
use anyhow::Result;
use clap::ArgMatches;
use passwords::{analyzer, scorer};
use sha2::{Digest, Sha256};

use rucksack_db::records;
use rucksack_db::Status;
use rucksack_lib::time;

use crate::app::App;
use crate::input::{options, query, Flag};
use crate::output::{result, table, Column, Opts};

use super::{backup, dedupe};

// TODO: once there's config for it, pull from config and pass
// options here from top-level app.
// TODO: or not, depending upon the outcome of this ticket:
// * https://github.com/oxur/rucksack/issues/92
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

pub fn backups(matches: &ArgMatches, app: &App) -> Result<()> {
    backup::list(matches, app)
}

// TODO: once there's config for it, pull from config and pass
// options here from top-level app.
// TODO: or not, depending upon the outcome of this ticket:
// * https://github.com/oxur/rucksack/issues/92
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

pub fn duplicates(matches: &ArgMatches, app: &App) -> Result<()> {
    let mut hash_fields: Vec<Column> = Vec::new();
    let mut default_hash_fields = exact_hash_fields();
    match matches.get_one::<String>("dedupe-type").map(|s| s.as_str()) {
        Some("exact") => {
            hash_fields.append(&mut exact_hash_fields());
        }
        Some("updated") => {
            hash_fields.append(&mut updated_hash_fields());
        }
        Some("any") => {
            hash_fields.append(&mut default_hash_fields);
        }
        Some(t) => {
            log::error!("Got unexpected type '{}'; using default ...", t);
            hash_fields.append(&mut default_hash_fields);
        }
        None => {
            hash_fields.append(&mut default_hash_fields);
        }
    }
    process_records(
        matches,
        app,
        Opts {
            group_by_hash: true,
            hash_fields,
            ..Default::default()
        },
    )
}

// TODO: once there's config for it, pull from config and pass
// options here from top-level app.
pub fn keys(matches: &ArgMatches, app: &App) -> Result<()> {
    process_records(
        matches,
        app,
        Opts {
            only_keys: true,
            ..Default::default()
        },
    )
}

pub fn passwords(matches: &ArgMatches, app: &App) -> Result<()> {
    let mut opts = Opts {
        password_history: true,
        ..Default::default()
    };
    opts.reveal = options::reveal(matches);
    opts.decrypted = options::decrypt(matches);
    let mut results: Vec<result::ResultRow> = Vec::new();
    let record = query::record(app)?;
    let analyzed = analyzer::analyze(record.password());
    let score = scorer::score(&analyzed).trunc() as i64;
    let md = record.metadata();
    let mut pwd = record.password();
    if !opts.reveal {
        pwd = hidden();
    }
    results.push(result::password(
        pwd,
        score.to_string(),
        md.created,
        md.updated,
        md.last_used,
    ));
    log::debug!("history length: {}", record.history().len());
    // Let's get these in order of most recent to oldest:
    let mut history = record.history();
    history.reverse();
    for old in history {
        pwd = old.secrets.password;
        if !opts.reveal {
            pwd = hidden();
        }
        results.push(result::password(
            pwd,
            score.to_string(),
            old.metadata.created,
            old.metadata.updated,
            old.metadata.last_used,
        ));
    }
    log::debug!("results length: {}", results.len());
    let mut t = table::new(results.to_owned(), opts);
    t.display();
    println!();
    Ok(())
}

fn process_records(matches: &ArgMatches, app: &App, mut opts: Opts) -> Result<()> {
    let sort_by = matches.get_one::<String>("sort-by").map(|s| s.as_str());
    opts.category = app.inputs.category(Flag::Many);
    opts.all_tags = options::all_tags(matches);
    opts.any_tags = options::any_tags(matches);
    opts.kind = options::record_kind(matches);
    opts.reveal = options::reveal(matches);
    opts.decrypted = options::decrypt(matches);
    if opts.reveal && opts.decrypted {
        opts.with_passwd = true;
    }
    match matches.get_one::<String>("group-by").map(|s| s.as_str()) {
        Some("name") => opts.group_by_name = true,
        Some("password") => opts.group_by_password = true,
        Some(_) => (),
        None => (),
    }
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
    let mut results: Vec<result::ResultRow> = Vec::new();
    let mut groups = result::GroupByString::new();
    if results.is_empty() {
        let rg = extract_results(matches, app, opts.clone())?;
        results = rg.results;
        groups = rg.groups;
    }
    sort(&mut results, sort_by);
    let mut group_count: i32 = 0;
    let mut count: usize = results.len();
    let total: usize = app.db.hash_map().len();
    if opts.group_by_name {
        (group_count, count) = print_user_group(groups, sort_by, &opts);
    } else if opts.group_by_password {
        (group_count, count) = print_password_group(groups, sort_by, &opts);
    } else if opts.group_by_hash {
        // Post-process groups
        groups = dedupe::post_process(groups);
        (group_count, count) = print_hash_group(groups, sort_by, &opts);
    } else {
        let mut t = table::new(results.to_owned(), opts.clone());
        t.display();
    }
    print_report(group_count, count, total, &opts);
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

fn extract_results(
    matches: &ArgMatches,
    app: &App,
    opts: Opts,
) -> Result<result::ResultsAndGroups> {
    let mut results: Vec<result::ResultRow> = Vec::new();
    let mut groups = result::GroupByString::new();
    let filter = matches.get_one::<String>("filter");
    let exclude = matches.get_one::<String>("exclude");
    let max_score = matches.get_one::<f64>("max-score");
    let min_score = matches.get_one::<f64>("min-score");

    for i in app.db.iter() {
        let record = i.value().decrypt(app.db.store_pwd(), app.inputs.salt())?;
        let analyzed = analyzer::analyze(record.password());
        let score = scorer::score(&analyzed);
        let mut result = result::new(record.key(), record.name_or_user(), record.metadata().url);
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
        if opts.kind != records::Kind::Any && opts.kind != record.metadata().kind {
            continue;
        }
        if opts.category != *records::ANY_CATEGORY && record.metadata().category != opts.category {
            continue;
        };
        if let Some(ref ts) = opts.all_tags {
            if !rucksack_lib::util::all(ts.clone(), record.metadata().tag_values()) {
                continue;
            }
        }
        if let Some(ref ts) = opts.any_tags {
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
        // TODO: generalise this logic ... maybe move it to impl ResultRow ...
        let md = record.metadata();
        result.add(Column::Key, record.key());
        result.add(Column::Kind, md.kind.name());
        result.add(Column::Category, md.category.clone());
        result.add(Column::Count, md.access_count.to_string());
        result.add(Column::Status, md.status().to_string());
        result.add(Column::HistoryCount, format!("{:}", record.history().len()));
        match opts.decrypted {
            true => {
                let pwd = if opts.reveal {
                    record.password()
                } else {
                    hidden()
                };
                result.add(Column::Password, pwd);
                result.add(Column::Score, (score.trunc() as i64).to_string());
            }
            false => result.add(Column::Password, hidden()),
        }
        if opts.group_by_name {
            let entry = groups.entry(record.name_or_user()).or_default();
            entry.push(result.clone());
        } else if opts.group_by_password {
            let entry = groups.entry(record.password()).or_default();
            entry.push(result.clone());
        } else if opts.group_by_hash {
            result.add(Column::LastUpdated, record.metadata().last_used);
            let entry = groups
                .entry(hash_by_columns(record.clone(), opts.hash_fields.clone()))
                .or_default();
            entry.push(result.clone());
        }
        results.push(result);
    }
    Ok(result::ResultsAndGroups { results, groups })
}

fn print_password_group(
    groups: result::GroupByString,
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
        password_section(&group[0], opts);
        println!("Records using: {}\nRecords:", group.len());
        let mut t = table::new(group.to_owned(), opts.clone());
        t.display();
        record_count += group.len();
    }
    (group_count, record_count)
}

fn print_user_group(
    groups: result::GroupByString,
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
        user_section(&group[0], opts);
        println!("Records using: {}\nRecords:", group.len());
        let mut t = table::new(group.to_owned(), opts.clone());
        t.display();
        record_count += group.len();
    }
    (group_count, record_count)
}

fn print_hash_group(
    groups: result::GroupByString,
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
        hash_section(&group[0], opts);
        println!("Duplicate record count: {}\nRecords:", group.len());
        let mut t = table::new(group.to_owned(), opts.clone());
        t.display();
        record_count += group.len();
    }
    (group_count, record_count)
}

fn print_report(group_count: i32, records: usize, total: usize, opts: &Opts) {
    if opts.group_by_name || opts.group_by_password || opts.group_by_hash {
        println!("\n{group_count} groups (with {records} records out of {total} total)\n",)
    } else {
        println!("\n{records} records (of {total} total)\n")
    }
}

fn password_section(r: &result::ResultRow, opts: &Opts) {
    println!("\n\n+{}\n", "=".repeat(40 + 30 + 2));
    match opts.decrypted {
        true => {
            if opts.reveal {
                println!(
                    "Password: {} (Score: {})",
                    r.get(&Column::Password).unwrap(),
                    r.get(&Column::Score).unwrap()
                )
            } else {
                println!(
                    "Password: {} (Score: {})",
                    hidden(),
                    r.get(&Column::Score).unwrap()
                )
            }
        }
        false => println!("Password Group"),
    }
}

fn user_section(r: &result::ResultRow, opts: &Opts) {
    match opts.decrypted {
        true => {
            println!("\n\n+{}\n", "=".repeat(40 + 20 + 16 + 5));
            println!("User: {}", r.get(&Column::Name).unwrap())
        }
        false => {
            println!("\n\n+{}\n", "=".repeat(40 - 1));
            println!("User: {}", r.get(&Column::Name).unwrap())
        }
    }
}

fn hash_section(r: &result::ResultRow, opts: &Opts) {
    match opts.decrypted {
        true => {
            println!("\n\n+{}\n", "=".repeat(40 + 20 + 16 + 5));
            println!("ID: {}", r.get(&Column::Id).unwrap())
        }
        false => {
            println!("\n\n+{}\n", "=".repeat(40 - 1));
            println!("ID: {}", r.get(&Column::Id).unwrap())
        }
    }
}

fn hidden() -> String {
    "*".repeat(10)
}

fn sort(results: &mut [result::ResultRow], sort_by: Option<&str>) {
    if results.is_empty() {
        return;
    }
    match sort_by {
        Some("score") => results.sort_by(|a, b| {
            b.get(&Column::Score)
                .unwrap()
                .parse::<i32>()
                .unwrap()
                .cmp(&a.get(&Column::Score).unwrap().parse::<i32>().unwrap())
        }),
        Some("url") => results.sort(),
        Some("name") => results.sort_by(|a, b| {
            a.get(&Column::Name)
                .unwrap()
                .cmp(b.get(&Column::Name).unwrap())
        }),
        Some(&_) => (),
        None => (),
    };
}

fn exact_hash_fields() -> Vec<Column> {
    vec![
        Column::Name,
        Column::Url,
        Column::Kind,
        Column::Category,
        Column::Password,
    ]
}

// The "updated" hash type is one where we expect all the key fields to be
// the same but differ only by password (a loser definition of "duplicate").
fn updated_hash_fields() -> Vec<Column> {
    vec![Column::Name, Column::Url, Column::Kind, Column::Category]
}

fn get_by_column(rec: records::DecryptedRecord, col: &Column) -> String {
    match col {
        Column::Category => rec.metadata().category,
        Column::Kind => rec.metadata().kind.to_string(),
        Column::Name => rec.name_or_user(),
        Column::Password => rec.password(),
        Column::Url => rec.url(),
        _ => unimplemented!(),
    }
}

fn get_by_columns(rec: records::DecryptedRecord, cols: Vec<Column>) -> Vec<String> {
    let mut vals: Vec<String> = vec![];
    for col in cols.iter() {
        vals.push(get_by_column(rec.clone(), col));
    }
    vals
}

fn hash_by_columns(rec: records::DecryptedRecord, cols: Vec<Column>) -> String {
    format!(
        "{:x}",
        Sha256::new()
            .chain_update(get_by_columns(rec, cols).join(":"))
            .finalize()
    )
}
