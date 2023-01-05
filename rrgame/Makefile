format:
	cargo fmt --quiet

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo-clippy --all-targets --all-features -- -D warnings 

test:
	cargo test 

build-release:
	@echo "Building release version for platfomr $(shell uname -s)"
	cargo build --release 

run:
	cargo run -- dedupe --path tests --pattern .txt

all: format lint test run build-release