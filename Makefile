##
## Newtype-tools Makefile
##

all:: test lcov html publish

help::
	@echo "Newtype-tools targets:"
	@echo "    help        This help message."
	@echo "    all         Run all the targets: test, publish."
	@echo "    lcov        Generate lcov code coverage report."
	@echo "    html        Generate HTML code coverage report."
	@echo "    open        Open HTML code coverage report."
	@echo "    test        Run the cargo format, check, clippy and tests."
	@echo "    publish     Run the cargo publish dry run."

test::
	@echo "==> Running the cargo format, check, clippy and tests..."
	cargo fmt
	cargo check
	cargo clippy
	cargo test
	cargo check --no-default-features
	cargo clippy --no-default-features
	cargo test --no-default-features
	@echo "All OK."

lcov::
	@echo "==> Generating lcov code coverage report..."
	cargo llvm-cov --lcov --output-path lcov.info
	@echo "All OK."

html::
	@echo "==> Generating HTML code coverage report..."
	cargo llvm-cov --html
	@echo "All OK."

open:: html
	@echo "==> Open HTML code coverage report..."
	open target/llvm-cov/html/index.html

publish::
	@echo "==> Running cargo publish dry run..."
	cargo publish --dry-run
	@echo "All OK."
