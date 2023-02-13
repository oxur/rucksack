# rucksack

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A terminal-based secrets manager, generator, and importer/exporter (Firefox, Chrome) backed with a concurrent hashmap*

## Features

* [x] Password generator (0.1.0)
* [x] Encrypted local storage (0.2.0)
* [x] Concurrent hashmap for use by daemons (0.2.0)
* [x] List secrets, both encrypted and decrypted (0.3.0)
* [x] Supports Firefox and Chrome CSV formats (importing, 0.3.0; exporting, 0.5.0)
* [x] Searching secrets via filtering (0.4.0)
* [x] Reports on password quality, duplicates, etc. (0.5.0)
* [x] Add new records to the DB (and support updates) via CLI subcommands (0.6.0)
* [x] Archived deletes (0.7.0)
* [x] Tags and categories (0.7.0)
* [x] Support public/private keys, certificates, API keys (0.7.0)
* [x] Split repo into rucksack-lib, rucksack-db, and rucksack crates (0.8.0)
* [ ] Password history (0.8.0)
* [ ] [Database restores](https://github.com/oxur/rucksack/milestone/11)
* [ ] [1Password Support](https://github.com/oxur/rucksack/milestone/12)
* [ ] [Local network sync](https://github.com/oxur/rucksack/milestone/13)

## Documentation

Primary project documentation is here:

* [https://docs.rs/rucksack/](https://docs.rs/rucksack/)

A quick peak at the top-level help from the terminal:

```text
rucksack: A terminal-based secrets manager, generator, and importer/exporter (Firefox, Chrome) backed with a concurrent hashmap

Usage: rucksack [OPTIONS] [COMMAND]

Commands:
  add     Add a new secret
  export  Export the rucksack db
  gen     Generate a secret
  import  Pull in creds from other sources
  list    List all secrets
  rm      Delete a single record [aliases: delete, remove]
  set     Perform various 'write' operations
  show    Display rucksack-specific information
  help    Print this message or the help of the given subcommand(s)

Options:
      --config-file <config-file>  The path to the config file to use or create [default: "<system config dir>/rucksack/config.toml"]
      --log-level <log-level>      Override the configured log-level setting [default: ] [possible values: error, warn, info, debug, trace, ]
      --completions <SHELL>        Emit shell tab completions [possible values: bash, elvish, fish, powershell, zsh]
  -v, --version                    Print version information
  -h, --help                       Print help
```

## Related

[Other projects](https://crates.io/keywords/password-manager?sort=downloads) on crates.io tagged as `#password-manager` ...

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
