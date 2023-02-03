#!/bin/bash

# This script is for integration tests of latest released version of rucksack.

. ./tests/common.sh || . ./common.sh

rm -f ~/.cargo/bin/rucksack ~/.cargo/registry/cache/github.com*/rucksack*

echo
header "Install rucksack"
echo

cargo install rucksack

echo
header "Show top-level help"
echo

RUST_BACKTRACE=1 rucksack help

# rucksack --config=$CFG_FILE gen --type uuid++ --encode
# rucksack --config=$CFG_FILE show config-file
# rucksack --config=$CFG_FILE show config
# rucksack --config=$CFG_FILE show --db=$DB_FILE data-dir
# rucksack --config=$CFG_FILE show --db=$DB_FILE db-file

header "Add a new record (shelly)"
echo

rucksack add --db=$DB_FILE --db-pass 1234 \
    --url http://example.com --user shelly --password whyyyyyy
rucksack list --db=$DB_FILE --db-pass 1234

header "Change the record user name"

rucksack set --db=$DB_FILE --db-pass 1234 \
    user --url http://example.com --old-user shelly --new-user clammy
rucksack list --db=$DB_FILE --db-pass 1234

header "List all records (with decrypted data)"

rucksack list --db=$DB_FILE --db-pass 1234 --decrypt

header "Add a new record (sully)"
echo

rucksack add --db=$DB_FILE --db-pass 1234 \
    --url http://boo.co --user sully --password numb3r1fan
rucksack list --db=$DB_FILE --db-pass 1234

header "List all records (with decrypted data and revealed passwords)"

rucksack list --db=$DB_FILE --db-pass 1234 \
    --decrypt --reveal

header "Filter records with 'exa' (decrypted data and revealed passwords)"

rucksack list --db=$DB_FILE --db-pass 1234 \
    --decrypt  --reveal --filter exa

header "Filter records with 'boo' (decrypted data and revealed passwords)"

rucksack list --db=$DB_FILE --db-pass 1234 \
    --decrypt  --reveal --filter boo
