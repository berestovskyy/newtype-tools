##
## Newtype-tools Makefile
##

MSRV := $(shell grep ^rust-version Cargo.toml | cut -d '"' -f 2)

check:: clippy fmt
	cargo test
	cargo check
	cargo test --no-default-features
	@echo "All OK."

help::
	@echo "Newtype-tools targets:"
	@echo "    check       Run quick checks: clippy, fmt, cargo test, cargo check."
	@echo "    help        This help message."
	@echo "    ci          Run CI pipeline locally: clippy, build, test, fmt, publish."
	@echo "  Test targets:"
	@echo "    clippy      Run cargo clippy."
	@echo "    build       Run cargo build."
	@echo "    test        Run cargo test."
	@echo "    trybuild    Overwrite trybuild test results."
	@echo "  Misc targets:"
	@echo "    fmt         Run cargo fmt."
	@echo "    keepsorted  Sort Rust derives alphabetically (cargo install keepsorted)."
	@echo "    publish     Run the cargo publish dry run."
	@echo "  Code coverage targets:"
	@echo "    lcov        Generate lcov code coverage report."
	@echo "    html        Generate HTML code coverage report."
	@echo "    open        Open HTML code coverage report."

ci:: clippy build test fmt publish
	@echo "All OK."

########################################################################
## Test Targets

clippy::
	cargo +stable clippy --workspace --all-targets -- -D warnings

build::
	cargo +stable build
	cargo +nightly build
	cargo +${MSRV} build
	cargo +stable build --no-default-features
	cargo +nightly build --no-default-features
	cargo +${MSRV} build --no-default-features
	cargo +nightly build --all-features

test::
	cargo +stable test
	cargo +nightly test
	cargo +${MSRV} test
	cargo +stable test --no-default-features
	cargo +nightly test --no-default-features
	cargo +${MSRV} test --no-default-features
	cargo +nightly test --all-features

trybuild::
	TRYBUILD=overwrite cargo test

########################################################################
## Misc Targets

fmt::
	cargo +nightly fmt -- --config reorder_impl_items=true,error_on_unformatted=true,error_on_line_overflow=true

keepsorted::
	keepsorted -f rust_derive_alphabetical -r .

publish::
	cargo publish --dry-run

########################################################################
## Code Coverage Targets

lcov::
	cargo llvm-cov --lcov --output-path lcov.info

html::
	cargo llvm-cov --html

open::
	cargo llvm-cov --html --open
