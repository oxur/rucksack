PROJ = rucksack
BIN_DIR = ./bin
BIN = target/release/$(PROJ)

default: all

all: deps build lint test

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
	@cargo install --path . --root .

lint:
	@cargo +nightly clippy --version
	@cargo +nightly clippy --all-targets --all-features -- --no-deps -D clippy::all

cicd-lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

test:
	@RUST_BACKTRACE=1 cargo test

deps:
	@cargo update

publish:
	@cargo publish

tag:
	@git tag $$($(BIN_DIR)/$(PROJ) -v)
	@git push --tags

nightly:
	@rustup toolchain install nightly

release: build lint test tag publish
