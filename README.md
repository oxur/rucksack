# rucksack

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A terminal-based password manager, importer (Firefox Sync), and generator*

## Features

* [x] Password generator
* [x] Encrypted local storage
* [x] Concurrent hashmap for use by daemons
* [x] Supports Firefox Sync
* [x] List secrets (encrypted and decrypted)
* [ ] Searching secrets
* [ ] Reports (quality, duplicates, etc.)
* [ ] Local network sync

## Usage

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

### Import Examples

```shell
./bin/rucksack import \
  --type firefox \
  --db ./data/creds.db \
  --password abc123 \
  --file ~/Downloads/logins.csv
```

### List Secrets

Show URL/accounts for all secrets:

```shell
./bin/rucksack list \
  --db ./data/creds.db \
```

```shell
Enter db password:
```

Show URLs, accounts, passwords, and password scores for all secrets:

```shell
./bin/rucksack list \
  --db ./data/creds.db \
  --decrypt
```

```shell
Enter db password:
```

In both cases a password may be passed with the `--password` flag. By default, the salt is the value of the `USER` environment variable; it may be overridden with `--salt`.

## Other

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
[build]: https://github.com/oxur/rucksack/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/oxur/rucksack/workflows/build/badge.svg
[crate]: https://crates.io/crates/rucksack
[crate-badge]: https://img.shields.io/crates/v/rucksack.svg
[docs]: https://docs.rs/rucksack/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/rucksack.svg
[tag]: https://github.com/oxur/rucksack/tags
