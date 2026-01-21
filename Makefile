# Makefile for the Rucksack Project

# ANSI color codes
BLUE := \033[1;34m
GREEN := \033[1;32m
YELLOW := \033[1;33m
RED := \033[1;31m
CYAN := \033[1;36m
RESET := \033[0m

# Variables
PROJECT_NAME := Rucksack
MODE := debug
TARGET := ./target/$(MODE)
BIN_DIR := ./bin
GIT_COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
BUILD_TIME := $(shell date -u '+%Y-%m-%dT%H:%M:%SZ')
RUST_VERSION := $(shell rustc --version 2>/dev/null || echo "unknown")

# List of binaries to build and install
BINARIES := rucksack fliterec

# External tools configuration
AI_RUST := ./assets/ai/ai-rust

# Default target
.DEFAULT_GOAL := help

# Build directory creation
$(BIN_DIR):
	@echo "$(BLUE)Creating bin directory...$(RESET)"
	@mkdir -p $(BIN_DIR)
	@echo "$(GREEN)✓ Directory created$(RESET)"

# Help target
.PHONY: help
help:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET) $(BLUE)$(PROJECT_NAME) Build System$(RESET)                                      $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Building:$(RESET)"
	@echo "  $(YELLOW)make build$(RESET)            - Build the library"
	@echo "  $(YELLOW)make build-release$(RESET)    - Build optimized release library"
	@echo "  $(YELLOW)make build MODE=release$(RESET) - Build with custom mode"
	@echo "  $(YELLOW)make examples$(RESET)         - Build all examples"
	@echo ""
	@echo "$(GREEN)Testing & Quality:$(RESET)"
	@echo "  $(YELLOW)make test$(RESET)             - Run all tests"
	@echo "  $(YELLOW)make lint$(RESET)             - Run clippy and format check"
	@echo "  $(YELLOW)make format$(RESET)           - Format all code with rustfmt"
	@echo "  $(YELLOW)make coverage$(RESET)         - Generate test coverage report"
	@echo "  $(YELLOW)make check$(RESET)            - Build + lint + test"
	@echo "  $(YELLOW)make check-all$(RESET)        - Build + lint + coverage"
	@echo ""
	@echo "$(GREEN)Cleaning:$(RESET)"
	@echo "  $(YELLOW)make clean$(RESET)            - Clean target directory"
	@echo ""
	@echo "$(GREEN)Utilities:$(RESET)"
	@echo "  $(YELLOW)make push$(RESET)             - Pushes to Codeberg and Github"
	@echo "  $(YELLOW)make auth$(RESET)             - Helper notes for authenticating cargo"
	@echo "  $(YELLOW)make publish$(RESET)          - Publishes crate to crates.io"
	@echo "  $(YELLOW)make tracked-files$(RESET)    - Save list of tracked files"
	@echo ""
	@echo "$(GREEN)Information:$(RESET)"
	@echo "  $(YELLOW)make info$(RESET)             - Show build information"
	@echo "  $(YELLOW)make check-tools$(RESET)      - Verify required tools are installed"
	@echo ""
	@echo "$(CYAN)Current status:$(RESET) Branch: $(GIT_BRANCH) | Commit: $(GIT_COMMIT)"
	@echo ""

# Info target
.PHONY: info
info:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(BLUE)Build Information$(RESET)                                       $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Project:$(RESET)"
	@echo "  Name:           $(PROJECT_NAME)"
	@echo "  Build Mode:     $(MODE)"
	@echo "  Build Time:     $(BUILD_TIME)"
	@echo ""
	@echo "$(GREEN)Paths:$(RESET)"
	@echo "  Target Dir:     $(TARGET)/"
	@echo "  Project Dir:    $$(pwd)"
	@echo ""
	@echo "$(GREEN)Git:$(RESET)"
	@echo "  Branch:         $(GIT_BRANCH)"
	@echo "  Commit:         $(GIT_COMMIT)"
	@echo ""
	@echo "$(GREEN)Tools:$(RESET)"
	@echo "  Rust:           $(RUST_VERSION)"
	@echo "  Cargo:          $$(cargo --version 2>/dev/null || echo 'not found')"
	@echo "  Rustfmt:        $$(rustfmt --version 2>/dev/null || echo 'not found')"
	@echo "  Clippy:         $$(cargo clippy --version 2>/dev/null || echo 'not found')"
	@echo ""
	@echo "$(GREEN)Library:$(RESET)"
	@if [ -f $(TARGET)/librucksack.rlib ]; then \
		echo "  rucksack:         $(GREEN)✓ built$(RESET)"; \
	else \
		echo "  rucksack:         $(RED)✗ not built$(RESET)"; \
	fi
	@echo ""

# Check tools target
.PHONY: check-tools
check-tools:
	@echo "$(BLUE)Checking for required tools...$(RESET)"
	@command -v rustc >/dev/null 2>&1 && echo "$(GREEN)✓ rustc found (version: $$(rustc --version))$(RESET)" || echo "$(RED)✗ rustc not found$(RESET)"
	@command -v cargo >/dev/null 2>&1 && echo "$(GREEN)✓ cargo found (version: $$(cargo --version))$(RESET)" || echo "$(RED)✗ cargo not found$(RESET)"
	@command -v rustfmt >/dev/null 2>&1 && echo "$(GREEN)✓ rustfmt found$(RESET)" || echo "$(RED)✗ rustfmt not found (install: rustup component add rustfmt)$(RESET)"
	@cargo clippy --version >/dev/null 2>&1 && echo "$(GREEN)✓ clippy found$(RESET)" || echo "$(RED)✗ clippy not found (install: rustup component add clippy)$(RESET)"
	@cargo llvm-cov --version >/dev/null 2>&1 && echo "$(GREEN)✓ llvm-cov found$(RESET)" || echo "$(RED)✗ llvm-cov not found (install: cargo install cargo-llvm-cov)$(RESET)"
	@command -v git >/dev/null 2>&1 && echo "$(GREEN)✓ git found$(RESET)" || echo "$(RED)✗ git not found$(RESET)"
	@test -f Cargo.toml && echo "$(GREEN)✓ Cargo.toml found$(RESET)" || echo "$(RED)✗ Cargo.toml not found$(RESET)"

# Build targets
.PHONY: build
build: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) library in $(MODE) mode...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release; \
	else \
		cargo build; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"


.PHONY: build-release
build-release:
	@$(MAKE) build MODE=release

.PHONY: examples
examples:
	@echo "$(BLUE)Building examples...$(RESET)"
	@cargo build --examples
	@echo "$(GREEN)✓ Examples built$(RESET)"

# Cleaning targets
.PHONY: clean
clean:
	@echo "$(BLUE)Cleaning target directory...$(RESET)"
	@cargo clean
	@echo "$(GREEN)✓ Clean complete$(RESET)"

# Testing & Quality targets
.PHONY: test
test:
	@echo "$(BLUE)Running tests...$(RESET)"
	@cargo test --all-features
	@echo "$(GREEN)✓ All tests passed$(RESET)"

integration:
	@./tests/rucksack.sh
	@./tests/rucksack_dev.sh

.PHONY: lint
lint:
	@echo "$(BLUE)Running linter checks...$(RESET)"
	@echo "$(CYAN)• Running clippy...$(RESET)"
	@cargo clippy --all-features -- -D warnings
	@echo "$(GREEN)✓ Clippy passed$(RESET)"
	@echo "$(CYAN)• Checking code formatting...$(RESET)"
	@cargo fmt -- --check
	@echo "$(GREEN)✓ Format check passed$(RESET)"

.PHONY: format
format:
	@echo "$(BLUE)Formatting code...$(RESET)"
	@cargo fmt
	@echo "$(GREEN)✓ Code formatted$(RESET)"

.PHONY: coverage
coverage:
	@echo "$(BLUE)Generating test coverage report...$(RESET)"
	@echo "$(CYAN)• Running tests with coverage (includes integration tests in ./tests)...$(RESET)"
	@cargo llvm-cov --all-features --workspace
	@echo "$(GREEN)✓ Coverage report generated$(RESET)"
	@echo "$(YELLOW)→ For detailed HTML report, run: make coverage-html$(RESET)"

.PHONY: coverage-html
coverage-html:
	@echo "$(BLUE)Generating HTML coverage report...$(RESET)"
	@echo "$(CYAN)• Running tests with coverage (includes integration tests in ./tests)...$(RESET)"
	@cargo llvm-cov --html --all-features --workspace
	@echo "$(GREEN)✓ HTML coverage report generated$(RESET)"
	@echo "$(CYAN)→ Report: target/llvm-cov/html/index.html$(RESET)"
	@echo "$(YELLOW)→ Open in browser: open target/llvm-cov/html/index.html$(RESET)"

# Combined check targets
.PHONY: check
check: build lint test
	@echo ""
	@echo "$(GREEN)✓ All checks passed (build + lint + test)$(RESET)"
	@echo ""

.PHONY: check-all
check-all: build lint coverage
	@echo ""
	@echo "$(GREEN)✓ Full validation complete (build + lint + coverage)$(RESET)"
	@echo ""

$(AI_RUST):
	@echo "$(BLUE)Cloning ai-rust skill ...$(RESET)"
	@git clone git@github.com:oxur/ai-rust.git ./assets/ai/ai-rust
	@echo "$(GREEN)✓ ai-rust set up$(RESET)"

# Utility targets
.PHONY: tracked-files
tracked-files:
	@echo "$(BLUE)Saving tracked files list...$(RESET)"
	@mkdir -p $(TARGET)
	@git ls-files > $(TARGET)/git-tracked-files.txt
	@echo "$(GREEN)✓ Tracked files saved to $(TARGET)/git-tracked-files.txt$(RESET)"
	@echo "$(CYAN)• Total files: $$(wc -l < $(TARGET)/git-tracked-files.txt)$(RESET)"

push:
	@echo "$(BLUE)Pushing changes ...$(RESET)"
	@echo "$(CYAN)• Codeberg:$(RESET)"
	@git push codeberg $(GIT_BRANCH) && git push codeberg --tags
	@echo "$(GREEN)✓ Pushed$(RESET)"
	@echo "$(CYAN)• Github:$(RESET)"
	@git push github $(GIT_BRANCH) && git push github --tags
	@echo "$(GREEN)✓ Pushed$(RESET)"

.PHONY: auth
auth:
	@echo "$(BLUE)Copy and paste the following in the terminal where you$(RESET)"
	@echo "$(BLUE)will be executing cargo commands:$(RESET)"
	@echo ""
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo ""

docs: DOCS_PATH = target/doc/rucksack
docs:
	@cargo doc --all-features --no-deps --workspace
	@echo
	@echo "Docs are available here:"
	@echo " * $(DOCS_PATH)"
	@echo " * file://$(shell pwd)/$(DOCS_PATH)/index.html"
	@echo

open-docs:
	@cargo doc --all-features --no-deps --workspace --open

.PHONY: publish
publish:
	@echo "$(BLUE)Publishing to crates.io...$(RESET)"
	@echo "$(YELLOW)⚠ Make sure you've updated the version in Cargo.toml$(RESET)"
	@echo "$(CYAN)• Running checks before publish...$(RESET)"
	@cargo test --all-features
	@cargo clippy --all-features -- -D warnings
	@echo "$(CYAN)• Publishing crate...$(RESET)"
	@cargo publish
	@echo "$(GREEN)✓ Published to crates.io$(RESET)"
