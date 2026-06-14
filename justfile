# Gaslighting Compiler — common tasks

set shell := ["bash", "-cu"]

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
release:
    cargo build --release

# Run all tests
test:
    cargo test

# Check formatting
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Run Clippy linter
lint:
    cargo clippy -- -D warnings

# Run all CI checks
ci: build test fmt-check lint

# Install locally via cargo
cargo-install:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean

# Run the compiler on the sample file
run-sample:
    cargo run -- compile test.c

# Show the shame diary
run-diary:
    cargo run -- diary
