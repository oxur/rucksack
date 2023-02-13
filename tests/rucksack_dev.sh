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

./bin/rucksack show config-file --log-level error

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

header "Add a new record (shelly)"
echo

./bin/rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user shelly \
    --password whyyyyyy --log-level trace

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 --log-level trace

exit 1

header "Show DB file format version"

./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Change the record user name"

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

header "List all records (with decrypted data)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt

header "Add a new record (sully)"
echo

./bin/rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://boo.co \
    --user sully \
    --password numb3r1fan \
    --tags "best friend",monster,blue

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List all records (with decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal

header "Filter records with 'exa' (decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt  \
    --reveal \
    --filter exa

header "Filter records with 'boo' (decrypted data and revealed passwords)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --filter boo

header "Remove an record (clammy)"

./bin/rucksack rm \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user clammy

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "List deleted records"

./bin/rucksack list deleted \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Add a records for different 'kinds' and categories"
echo

./bin/rucksack add \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --url http://example.com \
    --user alice \
    --password 1234 \
    --category "personal" \
    --type account \
    --account-id "ar314159"

./bin/rucksack add \
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

./bin/rucksack add \
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

./bin/rucksack add \
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

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \

header "Show just password types"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type password

header "Show just account types"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type account

header "Show just asymmetric-crypto types"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type asymmetric-crypto

header "Show just certificate types"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --type certs

header "Show just service credential types"

./bin/rucksack list \
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

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "default"

header "Show just 'personal' category"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "personal"

header "Show just 'business' category"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --category "business"

header "Show all categories"

./bin/rucksack show categories \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Show just 'monster'-tagged"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags monster

header "Show just records tagged with 'server' (using --all-tags)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --all-tags server

header "Show just records tagged with 'server' (using --any-tags)"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags server

header "Show all tagged with 'server' and 'cloud'"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --all-tags server,cloud

header "Show all tagged with 'server' or 'cloud'"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --decrypt \
    --reveal \
    --any-tags server,cloud

header "Show all tags"

./bin/rucksack show tags \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Export password data"

mkdir -p exports
EXPORT_FILE=exports/secrets.csv
rm -f $EXPORT_FILE
./bin/rucksack export \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    --type "password" \
    -o $EXPORT_FILE
echo

header "Import password export"

echo
./bin/rucksack import \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234 \
    -f $EXPORT_FILE

echo
header "List with latest access counts"

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

# TODO: Uncomment when JSON exports land ... see ticket:
# * https://github.com/oxur/rucksack/issues/71

# echo
# ./bin/rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# ./bin/rucksack export \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234 \
#     --type debug

# TODO: Uncomment when these tests pass on Linux/Docker ... see ticket:
# * https://github.com/oxur/rucksack/issues/64

# header "Read an old database (v0.5.0)"

# cp ./tests/testing-data/secrets-v0.5.0.db "$DB_FILE"
# ./bin/rucksack show db-version \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# ./bin/rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# header "Read an old database (v0.6.0)"

# cp ./tests/testing-data/secrets-v0.6.0.db "$DB_FILE"
# ./bin/rucksack show db-version \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# ./bin/rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# header "Read an old database (v0.7.0)"

# cp ./tests/testing-data/secrets-v0.7.0.db "$DB_FILE"
# ./bin/rucksack show db-version \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234

# ./bin/rucksack list \
#     --config-file "$CFG_FILE" \
#     --db "$DB_FILE" \
#     --db-pass 1234
