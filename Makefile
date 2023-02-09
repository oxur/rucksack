PROJ = rucksack
BIN_DIR = ./bin
BIN = target/release/$(PROJ)

default: all

all: clean deps build lint test

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

build: $(BIN_DIR)
	@cargo build --release
	@rm -f $(BIN_DIR)/*
	@cd rucksack && cargo install --path ./ --root ../

lint:
	@cargo +nightly clippy --version
	@cargo +nightly clippy --all-targets --all-features -- --no-deps -D clippy::all

cicd-lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

test:
	@RUST_BACKTRACE=1 cargo test

integration:
	@./tests/rucksack_dev.sh
	@./tests/rucksack.sh

deps:
	@cargo update

publish-lib:
	@cd rucksack-lib && cargo publish

publish-db:
	@cd rucksack-db && cargo publish

publish: publish-lib publish-db

publish-cli:
	@cd rucksack && cargo publish

publish-all: publish publish-cli

tag:
	@git tag $$($(BIN_DIR)/$(PROJ) -v)
	@git push --tags

release: build lint test tag publish

clean:
	@cargo clean
	@rm -f $(BIN_DIR)/$(PROJ)

clean-all: clean
	@rm .crates.toml .crates2.json Cargo.lock

fresh-all: clean-all all

fresh: clean all

nightly:
	@rustup toolchain install nightly

docs:
	@cargo doc --all-features --no-deps --workspace

open-docs:
	@cargo doc --all-features --no-deps --workspace --open
