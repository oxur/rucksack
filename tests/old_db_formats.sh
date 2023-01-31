#!/bin/bash

make build

. ./tests/common.sh || . ./common.sh

header "Read an old database (v0.5.0)"

cp ./tests/testing-data/secrets-v0.5.0.db "$DB_FILE"
./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

header "Read an old database (v0.6.0)"

cp ./tests/testing-data/secrets-v0.6.0.db "$DB_FILE"
./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234

./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass 1234
