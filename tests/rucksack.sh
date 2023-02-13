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


echo
header "Show config file (default)"

rucksack show config-file --log-level error

header "Show config file"

rucksack show config-file --config-file "$CFG_FILE"

header "Show config"

rucksack show config --config-file "$CFG_FILE"

header "Show data dir (default)"

rucksack show data-dir --config-file "$CFG_FILE"

header "Show data dir"

rucksack show data-dir --config-file "$CFG_FILE" --db "$DB_FILE"

header "Show DB file (default)"

rucksack show db-file --config-file "$CFG_FILE"

header "Show DB file"

rucksack show db-file --config-file "$CFG_FILE" --db "$DB_FILE"

echo
header "Generate encoded password"

rucksack gen --config-file "$CFG_FILE" --type uuid++ --encode

header "Add a new record (shelly)"
echo

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user shelly \
    --password whyyyyyy

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Show DB file format version"

rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Change the record user name"

rucksack set user \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --old-user shelly \
    --new-user clammy

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List all records (with decrypted data)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt

header "Add a new record (sully)"
echo

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://boo.co \
    --user sully \
    --password numb3r1fan \
    --tags "best friend",monster,blue

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List all records (with decrypted data and revealed passwords)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal

header "Filter records with 'exa' (decrypted data and revealed passwords)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt  \
    --reveal \
    --filter exa

header "Filter records with 'boo' (decrypted data and revealed passwords)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --filter boo

header "Remove an record (clammy)"

rucksack rm \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user clammy

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List deleted records"

rucksack list deleted \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Add a records for different 'kinds' and categories"
echo

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user alice \
    --password 1234 \
    --category "personal" \
    --type account \
    --account-id "ar314159"

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user alice \
    --password 1234 \
    --category "personal" \
    --tags ssh,server \
    --type asymmetric-crypto \
    --public "abc" \
    --private "def" \

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user alice \
    --password 1234 \
    --category "business" \
    --tags http,"rest server",cloud,server \
    --type certs \
    --public "abc" \
    --private "def" \
    --root "ghi"

rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user alice \
    --password 1234 \
    --category "business" \
    --tags "api keys",cloud \
    --type service-creds \
    --key "abc" \
    --secret "def"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \

header "Show just password types"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type password

header "Show just account types"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type account

header "Show just asymmetric-crypto types"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type asymmetric-crypto

header "Show just certificate types"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type certs

header "Show just service credential types"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type service-creds

header "Show the list of supported types"

rucksack show types \
    --config-file "$CFG_FILE"

header "Show just 'default' category"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "default"

header "Show just 'personal' category"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "personal"

header "Show just 'business' category"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "business"

header "Show all categories"

rucksack show categories \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Show just 'monster'-tagged"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags monster

header "Show just records tagged with 'server' (using --all-tags)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --all-tags server

header "Show just records tagged with 'server' (using --any-tags)"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags server

header "Show all tagged with 'server' and 'cloud'"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --all-tags server,cloud

header "Show all tagged with 'server' or 'cloud'"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags server,cloud

header "Show all tags"

rucksack show tags \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Export password data"

mkdir -p exports
EXPORT_FILE=exports/secrets.csv
rm -f $EXPORT_FILE
rucksack export \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --type "password" \
    -o $EXPORT_FILE
echo

header "Import password export"

echo
rucksack import \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    -f $EXPORT_FILE

echo
header "List with latest access counts"

rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

# TODO: Uncomment when JSON exports land ... see ticket:
# * https://github.com/oxur/rucksack/issues/71

# echo
# rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# rucksack export \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234 \
#     --type debug

# TODO: Uncomment when these tests pass on Linux/Docker ... see ticket:
# * https://github.com/oxur/rucksack/issues/64

# header "Read an old database (v0.5.0)"

# cp ./tests/testing-data/secrets-v0.5.0.db "$DB_FILE"
# rucksack show db-version \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# header "Read an old database (v0.6.0)"

# cp ./tests/testing-data/secrets-v0.6.0.db "$DB_FILE"
# rucksack show db-version \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234
