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
	@cargo install --path . --root .

lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

test:
	@RUST_BACKTRACE=1 cargo test

integration:
	@./tests/rucksack_dev.sh
	@./tests/rucksack.sh

deps:
	@cargo update

publish:
	@cargo publish

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
