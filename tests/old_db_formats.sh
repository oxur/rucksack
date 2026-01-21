#!/bin/bash

make build

. ./tests/common.sh || . ./common.sh

# Create directories for test database files
mkdir -p "$(dirname "$DB_FILE")"
mkdir -p "$BACKUP_DIR"

# Test database credentials
# These databases were created with password "1234" and salt "oubiwann"
# (the USER environment variable on the machine that created them)
TEST_DB_PASS="1234"
TEST_DB_SALT="oubiwann"

header "Read an old database (v0.5.0)"

cp ./tests/testing-data/secrets-v0.5.0.db "$DB_FILE"
./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"

./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"

header "Read an old database (v0.6.0)"

cp ./tests/testing-data/secrets-v0.6.0.db "$DB_FILE"
./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"

./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"

header "Read an old database (v0.7.0)"

cp ./tests/testing-data/secrets-v0.7.0.db "$DB_FILE"
./bin/rucksack list \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"

./bin/rucksack show db-version \
    --config-file "$CFG_FILE" \
    --db "$DB_FILE" \
    --db-pass "$TEST_DB_PASS" \
    --salt "$TEST_DB_SALT"
