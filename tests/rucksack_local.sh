#!/bin/bash
make build

. ./tests/common.sh || . ./common.sh

echo
header "Show top-level help"
echo
./bin/rucksack --config=$CFG_FILE help

echo
header "Generate encoded password"

./bin/rucksack --config=$CFG_FILE gen --type uuid++ --encode

header "Show config file"

./bin/rucksack --config=$CFG_FILE show --db=$DB_FILE --db-pass 1234 config-file

header "Show config"

./bin/rucksack --config=$CFG_FILE show --db=$DB_FILE --db-pass 1234 config

header "Show data dir"

./bin/rucksack --config=$CFG_FILE show --db=$DB_FILE --db-pass 1234 data-dir

header "Show DB file"

./bin/rucksack --config=$CFG_FILE show --db=$DB_FILE --db-pass 1234 db-file

header "Add a new account (shelly)"
echo

./bin/rucksack --config=$CFG_FILE add --db=$DB_FILE --db-pass 1234 \
    --url http://example.com --user shelly --password whyyyyyy
./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234

header "Change the account user name"

./bin/rucksack --config=$CFG_FILE set --db=$DB_FILE --db-pass 1234 \
    user --url http://example.com --old-user shelly --new-user clammy
./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234

header "List all accounts (with decrypted data)"

./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234 --decrypt

header "Add a new account (sully)"
echo

./bin/rucksack --config=$CFG_FILE add --db=$DB_FILE --db-pass 1234 \
    --url http://boo.co --user sully --password numb3r1fan
./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234

header "List all accounts (with decrypted data and revealed passwords)"

./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234 \
    --decrypt --reveal

header "Filter accounts with 'exa' (decrypted data and revealed passwords)"

./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234 \
    --decrypt  --reveal --filter exa

header "Filter accounts with 'boo' (decrypted data and revealed passwords)"

./bin/rucksack --config=$CFG_FILE list --db=$DB_FILE --db-pass 1234 \
    --decrypt  --reveal --filter boo
