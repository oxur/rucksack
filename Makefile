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
	@echo "  $(YELLOW)make check-deps$(RESET)       - Check for outdated dependencies"
	@echo ""
	@echo "$(GREEN)Dependencies:$(RESET)"
	@echo "  $(YELLOW)make deps$(RESET)             - Update dependencies to latest compatible versions"
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

.PHONY: test-cli
test-cli:
	@echo "$(BLUE)Testing CLI argument validation...$(RESET)"
	@cargo test -p rucksack test_cli_validates --lib
	@echo "$(GREEN)✓ CLI validation passed$(RESET)"

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

cicd-lint: lint

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

# Common checks
.PHONY: common-checks
common-checks: check-deps lint build test-cli

# Combined check targets
.PHONY: check
check: common-checks test
	@echo ""
	@echo "$(GREEN)✓ All checks passed (build + lint + test + cli-validation)$(RESET)"
	@echo ""

# Ensure cargo-binstall is available for fast tool installation
.PHONY: ensure-binstall
ensure-binstall:
	@command -v cargo-binstall >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-binstall...$(RESET)"; \
		curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
	}

.PHONY: check-deps
check-deps: ensure-binstall
	@echo "$(BLUE)Checking for outdated dependencies...$(RESET)"
	@command -v cargo-outdated >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-outdated...$(RESET)"; \
		cargo binstall -y cargo-outdated; \
	}
	@OUTPUT=$$(cargo outdated --root-deps-only); \
	echo "$$OUTPUT"; \
	echo ""; \
	if echo "$$OUTPUT" | grep -E "^[a-z0-9_-]+\s+" | grep -v "^----" | awk '{print $$3}' | grep -v "^---$$" | grep -v "^Compat$$" | grep -E "^[0-9]" | grep -q .; then \
		echo "$(RED)✗ Compatible dependency updates available$(RESET)"; \
		echo "$(YELLOW)→ Run 'make deps' to update and commit the updated Cargo.lock$(RESET)"; \
		exit 1; \
	else \
		echo "$(GREEN)✓ All dependencies up to date$(RESET)"; \
	fi

.PHONY: deps
deps: ensure-binstall
	@echo "$(BLUE)Updating dependencies ...$(RESET)"
	@command -v cargo-upgrade >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-edit...$(RESET)"; \
		cargo binstall -y cargo-edit; \
	}
	@cargo upgrade
	@echo "$(GREEN)✓ Cargo deps upgraded$(RESET)"

.PHONY: check-all
check-all: common-checks coverage
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

# Crates in dependency order (leaf crates first, dependent crates later)
PUBLISH_ORDER := rucksack-lib rucksack-db rucksack

.PHONY: publish
publish:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET) $(BLUE)Publishing $(PROJECT_NAME) Crates to crates.io$(RESET)                       $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)⚠ This will publish all crates in dependency order$(RESET)"
	@echo "$(YELLOW)⚠ Ensure all tests pass and versions are updated$(RESET)"
	@echo ""
	@read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
			echo "$(RED)✗ Aborted$(RESET)"; \
			exit 1; \
	fi
	@echo ""
	@echo "$(BLUE)Publishing crates in dependency order...$(RESET)"
	@for crate in $(PUBLISH_ORDER); do \
		echo ""; \
		echo "$(CYAN)• Publishing $$crate...$(RESET)"; \
		cd crates/$$crate && \
		output=$$(cargo publish 2>&1); \
		result=$$?; \
		cd ../..; \
		if [ $$result -eq 0 ]; then \
			echo "  $(GREEN)✓$(RESET) $$crate published successfully"; \
			echo "  $(YELLOW)→ Waiting 30s for crates.io index update...$(RESET)"; \
			sleep 30; \
		elif echo "$$output" | grep -q "already exists"; then \
			echo "  $(YELLOW)⊙$(RESET) $$crate already published, skipping"; \
		else \
			echo "  $(RED)✗$(RESET) Failed to publish $$crate"; \
			echo "$$output"; \
			exit 1; \
		fi; \
	done
	@echo ""
	@echo "$(GREEN)✓ All crates published successfully!$(RESET)"
	@echo ""

.PHONY: publish-dry-run
publish-dry-run:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET) $(BLUE)Dry Run: Publishing $(PROJECT_NAME) Crates$(RESET)                       $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(BLUE)Publishing order (in dependency order):$(RESET)"
	@i=1; \
	for crate in $(PUBLISH_ORDER); do \
		echo "  $(YELLOW)$$i.$(RESET) $$crate"; \
		i=$$((i+1)); \
	done
	@echo ""
	@echo "$(BLUE)Verifying each crate...$(RESET)"
	@for crate in $(PUBLISH_ORDER); do \
		echo ""; \
		echo "$(CYAN)• Checking $$crate...$(RESET)"; \
		cd crates/$$crate && \
		cargo publish --dry-run && \
		cd ../..; \
		if [ $$? -eq 0 ]; then \
			echo "  $(GREEN)✓$(RESET) $$crate passed validation"; \
		else \
			echo "  $(RED)✗$(RESET) $$crate failed validation"; \
			exit 1; \
		fi; \
	done
	@echo ""
	@echo "$(GREEN)✓ All crates ready for publishing!$(RESET)"
	@echo "$(CYAN)→ Run 'make publish' to publish to crates.io$(RESET)"
	@echo ""
