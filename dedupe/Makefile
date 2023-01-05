format:
	cargo fmt --quiet

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --quiet

test:
	cargo test --quiet

build-osx:
	#TBD
	#rustup target add x86_64-apple-darwin
	#cargo build --release --target x86_64-apple-darwin

run:
	cargo run -- --path /tmp

all: format lint test run