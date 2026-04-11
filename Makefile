##
## Newtype-tools Makefile
##

help::
	@echo "Newtype-tools targets:"
	@echo "    help        This help message."
	@echo "    test        Run the cargo format, check, clippy and tests."
	@echo "    dry-run     Run the cargo publish dry run."

test::
	@echo "==> Running the cargo format, check, clippy and tests..."
	cargo fmt && cargo check && cargo clippy && cargo test && cargo check --no-default-features&& cargo clippy --no-default-features && cargo test --no-default-features
	@echo "All OK."

dry-run::
	@echo "==> Running cargo publish dry run..."
	cargo publish --dry-run
	@echo "All OK."
