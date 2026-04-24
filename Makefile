##
## Newtype-tools Makefile
##

MSRV := $(shell grep ^rust-version Cargo.toml | cut -d '"' -f 2)
NO_STD_TARGET := "thumbv7em-none-eabi"

check:: clippy fmt
	cargo test
	cargo check
	cargo test --no-default-features
	@echo "All OK."

help::
	@echo "Newtype-tools targets:"
	@echo "    check       Run quick checks: clippy, fmt, cargo test, cargo check."
	@echo "    help        This help message."
	@echo "    ci          Run CI pipeline locally: clippy, build, no_std, test, fmt, publish."
	@echo "  Test targets:"
	@echo "    clippy      Run cargo clippy."
	@echo "    build       Run cargo build."
	@echo "    no_std      Run no_std check."
	@echo "    test        Run cargo test."
	@echo "    trybuild    Overwrite trybuild test results."
	@echo "  Misc targets:"
	@echo "    expand      Show macro expansion for a specified test: TEST=from make expand"
	@echo "    fmt         Run cargo fmt."
	@echo "    keepsorted  Sort Rust derives alphabetically (cargo install keepsorted)."
	@echo "    publish     Run the cargo publish dry run."
	@echo "  Code coverage targets:"
	@echo "    lcov        Generate lcov code coverage report."
	@echo "    html        Generate HTML code coverage report."
	@echo "    open        Open HTML code coverage report."

ci:: clippy build no_std test fmt publish
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

no_std::
	rustup target add ${NO_STD_TARGET}
	cargo check --target ${NO_STD_TARGET}
	cargo check --target ${NO_STD_TARGET} --no-default-features

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

expand::
	cargo expand -p newtype-tools --test ${TEST}

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
