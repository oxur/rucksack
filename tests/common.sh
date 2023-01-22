export RUST_BACKTRACE=FULL
TMP_DIR=/tmp/$(date +"%Y%m%d.%H%M%S")/rucksack
DB_FILE=$TMP_DIR/data/secrets.db
CFG_FILE=$TMP_DIR/config.toml

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
END_COLOR="\033[0m"

function header () {
    echo -e " ${GREEN}>>${END_COLOR} ${YELLOW}${1}${END_COLOR}"
}
