# lithe-cli

# vet
.PHONY: vet\:check check vet\:format format fmt vet\:lint lint vet\:all vet

vet\:check:
	@cargo check --verbose

check: vet\:check

vet\:format:
	@cargo fmt --all -- --check

vet\:fmt: vet\:format

format: vet\:format

fmt: vet\:format

vet\:lint:
	@cargo clippy --all-targets

lint: vet\:lint

vet\:all: check fmt lint

vet: vet\:check
