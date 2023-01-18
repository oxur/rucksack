# rucksack

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A terminal-based password manager, generator, and importer/exporter (Firefox, Chrome) backed with a concurrent hashmap*

## Features

* [x] Password generator (0.1.0)
* [x] Encrypted local storage (0.2.0)
* [x] Concurrent hashmap for use by daemons (0.2.0)
* [x] List secrets, both encrypted and decrypted (0.3.0)
* [x] Supports Firefox and Chrome CSV formats (for importing, 0.3.0 and exporting, 0.5.0)
* [x] Searching secrets via filtering (0.4.0)
* [x] Reports on password quality, duplicates, etc. (0.5.0)
* [x] Add new records to the DB (and support updates) via CLI subcommands (0.6.0)
* [ ] [Database restores](https://github.com/oxur/rucksack/milestone/9)
* [ ] [Local network sync](https://github.com/oxur/rucksack/milestone/10)
* [ ] [Firefox Account Sync Client](https://github.com/oxur/rucksack/milestone/11)

## Installation

```shell
cargo install rucksack
```

## Usage

Top-level help:

```shell
rucksack help
```

Output:

```shell
rucksack: A terminal-based password manager, generator, and importer/exporter (Firefox, Chrome) backed with a concurrent hashmap

Usage: rucksack [OPTIONS] [COMMAND]

Commands:
  add     add a new secret
  export  export the rucksack db
  gen     generate a secret
  import  pull in creds from other sources
  list    list all secrets
  rm      delete a single record
  set     perform various 'write' operations
  show    display rucksack-specific information
  help    Print this message or the help of the given subcommand(s)

Options:
      --completions <SHELL>  emit shell tab completions [possible values: bash, elvish, fish, powershell, zsh]
  -v, --version              Print version information
  -h, --help                 Print help
```

### Password Generator

Use a UUID:

```shell
./bin/rucksack gen --type uuid

New password: 229ef9b4-b95b-4d91-a6ac-f6b7ef1cfc47
Password score: 88.50
```

Augmented UUID:

```shell
./bin/rucksack gen --type uuid++

New password: 4C7360%E-4@60-4?03-b559-491C8A52E750
Password score: 100.00
```

Random:

```shell
./bin/rucksack gen --type random

New password: A&6YU?#xk.?)
Password score: 91.22
```

Lorem-ipsum inspired:

```shell
./bin/rucksack gen --type lipsum

New password: Esse-maius-amicitia,-nihil.-]9^,
Password score: 100.00
```

Some systems can't handle special characters, so a flag is available for encoding with base64, with the generated encoding getting scored:

```shell
./bin/rucksack gen --type lipsum --encode

New password: VmVydW0sLW9waW5vciwtc2NyaXB0b3JlbS10YW1lbi4tLjYrfQ
Password score: 100.00
```

### Importing and Exporting

Import login data from Firefox Sync:

```shell
./bin/rucksack import \
  --db-pass abc123 \
  --type firefox \
  --file ~/Downloads/logins.csv
```

Logins may be exported to files that can then be used to import into browsers:

```shell
./bin/rucksack export \
  --db-pass abc123 \
  --type chrome \
  --file /tmp/exported-logins.csv
```

For both importing and exporting, there are currently two supported types: `firefox` and `chrome`.

### Adding and Updating via Command

To add a single record via the CLI:

```shell
./bin/rucksack add \
  --url http://example.com \
  --user shelly \
  --password whyyyyyy
```

Note that `--user` and `--url` are required when adding a new record. A password is required, too: if one is not provided with `--password`, then you will be prompted:

```shell
./bin/rucksack add \
  --url http://example.com \
  --user shelly
```

```shell
Enter db password:
```

```shell
Enter password for record:
```

There are several types of changes to records that can't be made via an "update" subcommand due to how the data is used in the database. That did't leave too much data left for an "update" command, so the "record type" update was moved into the "set" group, too. The total list of `set` operations is:

* changing the password
* changing the user (account name)
* changing the URL
* changing the type of record

As such, these have their own sub commands (under `set`), as well as their flags and logic.

Changing a password:

```shell
./bin/rucksack set password \
  --url http://example.com \
  --user shelly
  --password whyyyyyyyyyyyyyyyyyyy
```

If the password isn't provided, you will be prompted at the terminal:

```shell
Enter account password:
```

Changing a user:

```shell
./bin/rucksack set user \
  --url http://example.com \
  --old-user shelly
  --new-user clammy
```

Changing a URL:

```shell
./bin/rucksack set url \
  --old-url http://example.com \
  --new-url http://shelly.com \
  --user clammy
```

Changing the record type:

```shell
./bin/rucksack set type \
  --url http://example.com \
  --user clammy
  --type account
```

Note that for all of this, should you want to pass the DB password, file, or salt, you will need to make sure those flags come after `set` but before the following subcommmand.

### List Secrets

Show URL/accounts for all secrets:

```shell
./bin/rucksack list
```

```shell
Enter db password:
```

Show URLs, accounts, passwords, and password scores for all secrets:

```shell
./bin/rucksack list --decrypt
```

```shell
Enter db password:
```

In both cases a password may be passed with the `--db-pass` flag. By default, the salt is the value of the `USER` environment variable, but it may be overridden with the `--salt` flag.

Note that without `--decrypt`, only the user and URL are displayed. With `--decrypt`, those as well as masked password and password score are displayed. To unmask the password, one must also set `--reveal`.

The default database location used is `./data/creds.db`. To use another location, the `--db` flag is available.

The flags `--db`, `--db-pass`, and `--salt` may be set for any subcommand that access the database.

### Search / Filter Secrets

Simple filtering is also possible (done using a flag with the `list` command, with or without sorting):

```shell
./bin/rucksack list \
  --db-pass abc123 \
  --filter exa \
  --sort-by score \
  --decrypt
```

```text
URL                                      | User / Account                 | Password             | Strength
-----------------------------------------+--------------------------------+----------------------+-----------
https://www.bugworld.com                 | hexapod123                     | **********           | 93
https://accounts.cloud.com               | hexapod@thing.systems          | **********           | 90
https://entymology.slack.com             | 6pod@example.com               | **********           | 86
https://bugs.slack.com                   | Alice "Hexapod" Roberts        | **********           | 85
https://twitter.com                      | TheOtherHexapod                | **********           | 83
https://portal-hexapod.testing.app       | alice@example.com              | **********           | 58
http://localhost:3000                    | alice@example.com              | **********           | 30

7 records (of 7 total)
```

It is also possible to perform negative filtering using `--exclude`. Additionally, `--include` is provided as an alias for `--filter`.

You may sort on `score` (strength), `user`, or `url`. If not provided, `url` sorting is used. Also note that `order-by` is provided as an alias for `sort-by`.

### Grouping Results

#### By Password

For use in auditing, sites+user combinations that share the same password can be reported:

```shell
./bin/rucksack list \
  --group-by db-pass \
  --decrypt
```

```text
+========================================================================

Password: ********** (Score: 99)
Accounts using: 5
Accounts:

URL                                      | User / Account
-----------------------------------------+-------------------------------
https://smile.amazon.com                 | alice@example.com
https://smile.amazon.com/ap/signin       | alice@example.com
https://www.amazon.com                   | alice@example.com
https://www.amazon.com/ap/signin         | alice@example.com
https://mybank.com                       | alice@example.com

+========================================================================

Password: ********** (Score: 86)
Accounts using: 2
Accounts:

URL                                      | User / Account
-----------------------------------------+-------------------------------
https://blurp.com                        | alice
https://bleep.net                        | alice

2 groups (with 7 records out of 16 total)
```

#### By User

You may also group by user name (account name):

```shell
./bin/rucksack list \
  --group-by user \
  --decrypt
```

## Related

[Here](https://crates.io/keywords/password-manager?sort=downloads) are other cargo projects tagged with "password manager" ...

Projects of particular interest:

* [kbs2](https://github.com/woodruffw/kbs2) - A secret manager backed by age
* [RustCrypto](https://github.com/RustCrypto) - A Github org collecting a handful of pure-Rust encryption libraries
* [Firefox Sync](https://support.mozilla.org/en-US/kb/how-firefox-securely-saves-passwords)

## License

Copyright © 2022-2023, Oxur Group

Apache License, Version 2.0

[//]: ---Named-Links---

[logo]: resources/images/logo-v1-x250.png
[logo-large]: resources/images/logo-v1-x1000.png
[build]: https://github.com/oxur/rucksack/actions/workflows/cicd.yml
[build-badge]: https://github.com/oxur/rucksack/actions/workflows/cicd.yml/badge.svg
[crate]: https://crates.io/crates/rucksack
[crate-badge]: https://img.shields.io/crates/v/rucksack.svg
[docs]: https://docs.rs/rucksack/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/rucksack.svg
[tag]: https://github.com/oxur/rucksack/tags
