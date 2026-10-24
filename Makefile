.PHONY: check fmt lint test clean

# Check that the project compiles
check:
	cargo check --all-targets --all-features

# Check code formatting
fmt:
	cargo fmt --all -- --check

# Run Clippy
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	cargo test --all-targets

# Remove build artifacts
clean:
	cargo clean