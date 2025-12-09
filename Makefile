# This enables unstables features in the stable compiler.
export RUSTC_BOOTSTRAP=1

all: build

build: ## Builds the program with release mode
	cargo br

check: ## Performs a cargo check with release mode
	cargo cr

example: ## Runs a given a example e.g. `make example AGRS=example1`
	cargo er $(ARGS)

clean: ## Cleans cargo generated artifacts
	cargo clean

clippy: ## Runs clippy
	unset RUSTC_BOOTSTRAP && cargo clippy --workspace -- -D warnings
	unset RUSTC_BOOTSTRAP && cargo clippy --tests --workspace -- -D warnings

test: ## Runs all tests
	unset RUSTC_BOOTSTRAP && cargo tr --all

run: ## Runs main in release mode
	cargo rr

fmt: ## Formats the code
	cargo fmt

fmt_check: ## Check if the code is formatted
	cargo fmt -- --check

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'


.PHONY: all build check example clean clippy test run fmt fmt_check help
