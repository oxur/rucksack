#!/bin/bash

# This script is for integration tests of unreleased versions of rucksack
# that are still in-development.

make build

. ./tests/common.sh || . ./common.sh

echo
header "Show top-level help"
echo
./bin/rucksack help

echo
header "Show config file (default)"

./bin/rucksack show config-file

header "Show config file"

./bin/rucksack show config-file --config-file "$CFG_FILE"

header "Show config"

./bin/rucksack show config --config-file "$CFG_FILE"

header "Show data dir (default)"

./bin/rucksack show data-dir --config-file "$CFG_FILE"

header "Show data dir"

./bin/rucksack show data-dir --config-file "$CFG_FILE" --db "$DB_FILE"

header "Show DB file (default)"

./bin/rucksack show db-file --config-file "$CFG_FILE"

header "Show DB file"

./bin/rucksack show db-file --config-file "$CFG_FILE" --db "$DB_FILE"

echo
header "Generate encoded password"

./bin/rucksack gen --config-file "$CFG_FILE" --type uuid++ --encode

header "Add a new account (shelly)"
echo

./bin/rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user shelly \
    --password whyyyyyy

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Show DB file format version"

./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Change the account user name"

./bin/rucksack set user \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --old-user shelly \
    --new-user clammy

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List all accounts (with decrypted data)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt

header "Add a new account (sully)"
echo

./bin/rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://boo.co \
    --user sully \
    --password numb3r1fan

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List all accounts (with decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal

header "Filter accounts with 'exa' (decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt  \
    --reveal \
    --filter exa

header "Filter accounts with 'boo' (decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --filter boo

# header "Remove an account (clammy)"

# ./bin/rucksack rm \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234 \
#     --url http://example.com \
#     --user clammy --log-level trace
# ./bin/rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

header "Read an old database (v0.5.0)"

cp ./tests/testing-data/secrets-v0.5.0.db "$DB_FILE"
./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 --log-level trace

exit 1

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --log-level trace

header "Read an old database (v0.6.0)"

cp ./tests/testing-data/secrets-v0.6.0.db "$DB_FILE"
./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --log-level trace

header "Debug: check the data directory"

DIR=$(./bin/rucksack show data-dir --config-file "$CFG_FILE" --db "$DB_FILE")
ls -l $(echo $DIR|xargs)
