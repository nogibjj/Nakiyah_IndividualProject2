# Makefile for managing Rust project tasks

# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	@rustc --version              # Rust compiler
	@cargo --version              # Rust package manager
	@rustfmt --version            # Rust code formatter
	@rustup --version             # Rust toolchain manager
	@clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Clean and load data
load_data:
	cargo run -- clean_load

# Query top records
query_top:
	cargo run -- query_top 20

# Query specific record
query_specific:
	cargo run -- query_specific 101

# Create a new record
create:
	cargo run -- create_record 101 28 "Data Scientist" "Tech" 5 "Remote" 40 "None" true

# Update a record
update:
	cargo run -- update_record 101 30 "Senior Data Scientist" "Tech" 6 "On-site" 45 "None" false

# Delete a record
delete:
	cargo run -- delete_record 101

