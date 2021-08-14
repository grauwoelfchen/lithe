PACKAGE = lithe

# vet
vet\:check: # Check error [synonym: check]
	@cargo check --all --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Show format diff [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Show suggestions relates to hygiene [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: check fmt lint # Run all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias for vet:check
.PHONY: vet

# test
test\:unit: # Run only unit tests for lib
	@cargo test --lib --bins -- --nocapture
.PHONY: test\:unit

test\:doc: # Run only doc tests
	@cargo test --doc
.PHONY: test\:doc

test\:integration: # Run only integration tests
	@cargo test --test integration -- --nocapture
.PHONY: test\:integration

test\:all: test\:doc # Run all tests
	@cargo test --lib --bins --test integration -- --nocapture
.PHONY: test\:all

test: test\:unit # Alias ofr test:unit
.PHONY: test

# build
build\:debug: # Run packages [synonym: build]
	cargo build --workspace
.PHONY: build\:debug

build: build\:debug
.PHONY: build

build\:debug\:cli: # Build only cli package [synonym: build:cli]
	cargo build --bin $(PACKAGE)
.PHONY: build\:debug\:cli

build\:cli: build\:debug\:cli
.PHONY: build\:cli

build\:debug\:lib: # Build only lib package [synonym: build:lib]
	cargo build --lib
.PHONY: build\:debug\:lib

build\:lib: build\:debug\:lib
.PHONY: build\:lib

build\:release: # Build packages with release mode
	cargo build --workspace --release
.PHONY: build\:release

build\:release\:cli: # Build only cli package with release mode
	cargo build --package $(PACKAGE)-cli --bin $(PACKAGE) --release
.PHONY: build\:release\:cli

build\:release\:lib: # Build only lib package with release mode
	cargo build --package $(PACKAGE) --lib --release
.PHONY: build\:release\:lib

# utility
watch\:lib: # Start a process to watch for lib (require cargo-watch)
	cargo watch --exec 'run --package $(PACKAGE)' --delay 0.3
.PHONY: watch\:lib

watch\:cli: # Start a process to watch for cli (require cargo-watch)
	cargo watch --exec 'run --package $(PACKAGE)-cli' --delay 0.3
.PHONY: watch\:cli

clean: # Remove built artifacts
	@cargo clean
.PHONY: clean

package\:%: # Create a package of `lithe` or `lithe-cli`
	@cargo package --manifest-path src/$(subst package:,,$@)/Cargo.toml
.PHONY: package

install: # Install `lithe-cli` into the dir same with cargo
	@cargo install --path src/$(PACKAGE)-cli --force
.PHONY: install

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-18s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = vet\:all
default: vet\:all
