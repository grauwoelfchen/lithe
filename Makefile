PACKAGE = lithe

# vet
.PHONY: vet\:check check vet\:format format fmt vet\:lint lint vet\:all vet

vet\:check: # Check error [synonym: check]
	@cargo check --workspace --verbose

check: vet\:check

vet\:format: # Show format diff [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check

vet\:fmt: vet\:format

format: vet\:format

fmt: vet\:format

vet\:lint: # Show suggestions relates to hygiene [synonym: lint]
	@cargo clippy --all-targets

lint: vet\:lint

vet\:all: check fmt lint # Run all vet targets

vet: vet\:check # Alias for vet:check

# test
.PHONY: test\:unit\:lib test\:unit\:bins test\:unit test\:doc test\:integration \
	test\:all test cov\:lib coverage\:lib coverage cov

test\:unit\:lib: # Run only unit tests for lib
	@cargo test --lib -- --nocapture

test\:unit\:bins: # Run only unit tests for bins
	@cargo test --bins -- --nocapture

test\:unit: # Run all unit tests for lib and bins both [synonym: test]
	@cargo test --lib --bins -- --nocapture

test\:doc: # Run only doc tests
	@cargo test --doc

test\:integration: # Run only integration tests
	@cargo test --test integration -- --nocapture

test\:all: test\:doc # Run all tests
	@cargo test --lib --bins --test integration -- --nocapture

test: test\:unit

cov\:lib: coverage\:lib

coverage\:lib: # Generate a coverage report for lib crate [synonym: cov:lib]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/lib"; \
	cargo test --lib --no-run --target-dir=$${target_dir}; \
	result=($${target_dir}/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$${target_dir}/index.js*"; \
	fi; \
	file=($$target_dir/debug/deps/$(PACKAGE)-*); \
	kcov --verify --include-path=$$dir/src $$target_dir $${file[0]}; \
	grep 'index.html' $$target_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | \
		sed -E 's/[a-z\:"]*//g;s/([0-9\.]+)/\1%/'

cov\:lib: coverage\:lib

coverage: coverage\:lib # Alias for coverage:lib [synonym: cov]

cov: coverage

# documentation
.PHONY: document doc

document: # Generate documentation files [synonym: doc]
	cargo doc --no-deps --package $(PACKAGE)
.PHONY: document

doc: document

# build
.PHONY: build build\:cli build\:lib \
	build\:debug build\:debug\:cli build\:debug\:lib \
	build\:release build\:release\:cli build\:release\:lib

build\:debug: # Run packages [synonym: build]
	cargo build --workspace

build: build\:debug

build\:debug\:cli: # Build only cli package [synonym: build:cli]
	cargo build --bin $(PACKAGE)

build\:cli: build\:debug\:cli

build\:debug\:lib: # Build only lib package [synonym: build:lib]
	cargo build --lib

build\:lib: build\:debug\:lib

build\:release: # Build packages with release mode
	cargo build --workspace --release

build\:release\:cli: # Build only cli package with release mode
	cargo build --package $(PACKAGE)-cli --bin $(PACKAGE) --release

build\:release\:lib: # Build only lib package with release mode
	cargo build --package $(PACKAGE) --lib --release

# utility
.PHONY: watch\:lib watch\:cli clean package install help

watch\:lib: # Monitor build process for lib (require cargo-watch)
	cargo watch --exec 'build --package $(PACKAGE)' --delay 0.3

watch\:cli: # Monitor build process for cli (require cargo-watch)
	cargo watch --exec 'build --package $(PACKAGE)-cli' --delay 0.3

clean: # Remove built artifacts
	@cargo clean

package\:%: # Create a package of `lithe` or `lithe-cli`
	@cargo package --manifest-path src/$(subst package:,,$@)/Cargo.toml

install: # Install `lithe-cli` into the dir same with cargo
	@cargo install --path src/$(PACKAGE)-cli --force

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

.DEFAULT_GOAL = vet\:all
default: vet\:all
